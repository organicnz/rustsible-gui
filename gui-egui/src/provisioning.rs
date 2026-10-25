#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::app::ProvisioningMessage;
use crate::config::ProvisioningConfig;

use std::sync::OnceLock;

pub(crate) fn format_reboot_schedule(hour: &str) -> String {
    match hour {
        "1" => "01:00 Standard",
        "3" => "03:00 Standard",
        "5" => "05:00 Standard",
        "*/6" => "Interval: 6 Hours",
        "*/12" => "Interval: 12 Hours",
        _ => "03:00 Standard",
    }
    .into()
}

pub(crate) fn get_repo_root() -> Result<PathBuf, String> {
    let mut current = std::env::current_exe().map_err(|e| e.to_string())?;
    while let Some(parent) = current.parent() {
        if parent.join("rustsible.yml").exists() {
            return Ok(parent.to_path_buf());
        }
        current = parent.to_path_buf();
    }
    std::env::current_dir().map_err(|e| e.to_string())
}

pub(crate) fn is_timing_only_line(s: &str) -> bool {
    s.contains("*******") && !s.contains("TASK") && !s.contains("PLAY")
}

pub(crate) fn strip_ansi(s: &str) -> String {
    static ANSI_RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = ANSI_RE.get_or_init(|| {
        regex::Regex::new(r"[\x1b\x00]\[[\d;]*m|\\u001b\[[\d;]*m|\\033\[[\d;]*m").unwrap()
    });
    re.replace_all(s, "").to_string()
}

pub(crate) fn bool_to_yes_no(v: bool) -> &'static str {
    if v {
        "yes"
    } else {
        "no"
    }
}

#[cfg(unix)]
pub(crate) async fn setup_ssh_agent(
    key_path: &std::path::Path,
    passphrase: &str,
) -> Result<(String, String), String> {
    let output = Command::new("ssh-agent")
        .arg("-s")
        .output()
        .await
        .map_err(|e| e.to_string())?;
    let out_str = String::from_utf8_lossy(&output.stdout);
    let mut socket = String::new();
    let mut pid = String::new();
    for line in out_str.lines() {
        if line.contains("SSH_AUTH_SOCK=") {
            socket = line
                .split(';')
                .next()
                .unwrap()
                .replace("SSH_AUTH_SOCK=", "");
        }
        if line.contains("SSH_AGENT_PID=") {
            pid = line
                .split(';')
                .next()
                .unwrap()
                .replace("SSH_AGENT_PID=", "");
        }
    }

    if socket.is_empty() || pid.is_empty() {
        return Err("Failed to initialize SSH agent".into());
    }

    let result = if !passphrase.is_empty() {
        let askpass = std::env::temp_dir().join(format!("askpass_{}.sh", pid));
        let script = format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", passphrase);
        tokio::fs::write(&askpass, script)
            .await
            .map_err(|e| e.to_string())?;
        tokio::fs::set_permissions(&askpass, PermissionsExt::from_mode(0o700))
            .await
            .map_err(|e| e.to_string())?;

        let o = Command::new("ssh-add")
            .arg(key_path)
            .env("SSH_AUTH_SOCK", &socket)
            .env("SSH_ASKPASS", &askpass)
            .env("DISPLAY", ":0")
            .env("SSH_ASKPASS_REQUIRE", "force")
            .output()
            .await
            .map_err(|e| e.to_string())?;

        let _ = tokio::fs::remove_file(askpass).await;
        o
    } else {
        Command::new("ssh-add")
            .arg(key_path)
            .env("SSH_AUTH_SOCK", &socket)
            .output()
            .await
            .map_err(|e| e.to_string())?
    };

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        let detail = stderr.trim();
        let _ = std::process::Command::new("kill").arg(&pid).status();
        let msg = if detail.is_empty() {
            "Decryption Failed — check passphrase or key format".to_string()
        } else {
            format!("Decryption Failed: {}", detail)
        };
        return Err(msg);
    }

    Ok((socket, pid))
}

#[cfg(not(unix))]
pub(crate) async fn setup_ssh_agent(
    _key_path: &std::path::Path,
    _passphrase: &str,
) -> Result<(String, String), String> {
    Err("SSH agent passphrase unlock is not supported on this platform".into())
}

pub(crate) async fn run_provisioning(
    config: ProvisioningConfig,
    tx: Sender<ProvisioningMessage>,
    shutdown: Arc<AtomicBool>,
    child_pid: Arc<AtomicU32>,
) -> Result<(), String> {
    // Validate configuration first
    if config.ip_address.trim().is_empty() {
        return Err("IP address is required".to_string());
    }
    if config.ssh_user.trim().is_empty() {
        return Err("SSH user is required".to_string());
    }
    if config.ssh_key_path.trim().is_empty() {
        return Err("SSH key path is required".to_string());
    }

    let root = get_repo_root()?;

    let ssh_key_path = if config.ssh_key_path.starts_with("~/") {
        dirs::home_dir()
            .ok_or("Failed to get home directory")?
            .join(&config.ssh_key_path[2..])
    } else {
        PathBuf::from(&config.ssh_key_path)
    };

    // Verify SSH key exists
    if !ssh_key_path.exists() {
        return Err(format!("SSH key not found: {}", ssh_key_path.display()));
    }

    let mut auth_sock: Option<String> = None;
    let mut agent_pid: Option<String> = None;

    if !config.ssh_key_passphrase.is_empty() {
        let _ = tx.send(ProvisioningMessage::Output(
            "🔐 Unlocking SSH key...".into(),
        ));
        match setup_ssh_agent(&ssh_key_path, &config.ssh_key_passphrase).await {
            Ok((socket, pid)) => {
                auth_sock = Some(socket);
                agent_pid = Some(pid);
                let _ = tx.send(ProvisioningMessage::Output("✅ SSH key unlocked".into()));
            }
            Err(e) => {
                return Err(format!("Failed to setup SSH agent: {}", e));
            }
        }
    }

    let _ = tx.send(ProvisioningMessage::Output(
        "📋 Building Ansible command...".to_string(),
    ));

    let mut cmd = Command::new("ansible-playbook");
    cmd.current_dir(&root).env("ANSIBLE_NOCOLOR", "1");
    if let Some(sock) = &auth_sock {
        cmd.env("SSH_AUTH_SOCK", sock);
    }

    cmd.arg("rustsible.yml");
    cmd.arg("-e")
        .arg(format!("target_ip={}", config.ip_address));
    cmd.arg("-e")
        .arg(format!("target_user={}", config.ssh_user));
    if !config.connection_password.is_empty() {
        cmd.arg("-e").arg(format!(
            "connection_password={}",
            config.connection_password
        ));
    }
    cmd.arg("-e")
        .arg(format!("ssh_key_path={}", ssh_key_path.display()));

    if !config.hostname.is_empty() {
        cmd.arg("-e")
            .arg(format!("target_hostname={}", config.hostname));
    }
    cmd.arg("-e").arg(format!(
        "prompt_create_user={}",
        bool_to_yes_no(config.create_user)
    ));
    cmd.arg("-e")
        .arg(format!("added_user={}", config.added_user));
    cmd.arg("-e")
        .arg(format!("user_password={}", config.user_password));
    cmd.arg("-e").arg(format!(
        "prompt_install_docker={}",
        bool_to_yes_no(config.docker)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_lemp={}",
        bool_to_yes_no(config.lemp)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_wordpress={}",
        bool_to_yes_no(config.wordpress)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_certbot={}",
        bool_to_yes_no(config.certbot)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_dev_tools={}",
        bool_to_yes_no(config.devtools)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_neovim={}",
        bool_to_yes_no(config.install_neovim)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_zsh={}",
        bool_to_yes_no(config.install_zsh)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_tmux={}",
        bool_to_yes_no(config.install_tmux)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_nodejs={}",
        bool_to_yes_no(config.install_nodejs)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_claude_code={}",
        bool_to_yes_no(config.install_claude_code)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_gemini={}",
        bool_to_yes_no(config.install_gemini)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_kiro={}",
        bool_to_yes_no(config.install_kiro)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_github_cli={}",
        bool_to_yes_no(config.install_github_cli)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_btop={}",
        bool_to_yes_no(config.install_btop)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_ripgrep={}",
        bool_to_yes_no(config.install_ripgrep)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_fd={}",
        bool_to_yes_no(config.install_fd)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_duf={}",
        bool_to_yes_no(config.install_duf)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_ncdu={}",
        bool_to_yes_no(config.install_ncdu)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_lnav={}",
        bool_to_yes_no(config.install_lnav)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_tldr={}",
        bool_to_yes_no(config.install_tldr)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_lazygit={}",
        bool_to_yes_no(config.install_lazygit)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_uv={}",
        bool_to_yes_no(config.install_uv)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_fzf={}",
        bool_to_yes_no(config.install_fzf)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_bat={}",
        bool_to_yes_no(config.install_bat)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_eza={}",
        bool_to_yes_no(config.install_eza)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_zoxide={}",
        bool_to_yes_no(config.install_zoxide)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_jq={}",
        bool_to_yes_no(config.install_jq)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_htop={}",
        bool_to_yes_no(config.install_htop)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_gping={}",
        bool_to_yes_no(config.install_gping)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_nmap={}",
        bool_to_yes_no(config.install_nmap)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_autossh={}",
        bool_to_yes_no(config.install_autossh)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_starship={}",
        bool_to_yes_no(config.install_starship)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_direnv={}",
        bool_to_yes_no(config.install_direnv)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_fish={}",
        bool_to_yes_no(config.install_fish)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_micro={}",
        bool_to_yes_no(config.install_micro)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_install_ranger={}",
        bool_to_yes_no(config.install_ranger)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_enable_fail2ban={}",
        bool_to_yes_no(config.fail2ban)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_enable_swap={}",
        bool_to_yes_no(config.swap)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_enable_cron_jobs={}",
        bool_to_yes_no(config.cron_jobs)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_enable_periodic_reboot={}",
        bool_to_yes_no(config.periodic_reboot)
    ));
    cmd.arg("-e").arg(format!(
        "prompt_enable_update_before_reboot={}",
        bool_to_yes_no(config.update_before_reboot)
    ));
    cmd.arg("-e")
        .arg(format!("prompt_reboot_hour={}", config.reboot_hour));

    if config.system_hardening {
        cmd.arg("-e").arg("enable_kernel_hardening=yes");
    }
    if config.apparmor {
        cmd.arg("-e").arg("enable_apparmor=yes");
    }
    if config.rootkit_detection {
        cmd.arg("-e").arg("enable_rkhunter=yes");
    }
    if config.file_integrity {
        cmd.arg("-e").arg("enable_aide=yes");
    }
    if config.audit_logging {
        cmd.arg("-e").arg("enable_auditd=yes");
    }
    if config.log_monitoring {
        cmd.arg("-e").arg("enable_logwatch=yes");
    }

    if config.ssh_2fa_totp || config.ssh_2fa_fido2 || config.ssh_2fa_duo {
        cmd.arg("-e").arg("enable_ssh_2fa=yes");
        if config.ssh_2fa_fido2 {
            cmd.arg("-e").arg("enable_ssh_2fa_fido2=yes");
        }
        if config.ssh_2fa_duo {
            cmd.arg("-e").arg("enable_ssh_2fa_duo=yes");
        }
    }

    if config.backups {
        cmd.arg("-e").arg("enable_backups=yes");
    }
    if config.usb_restrictions {
        cmd.arg("-e").arg("enable_usb_restrictions=yes");
    }
    if config.disable_ipv6 {
        cmd.arg("-e").arg("disable_ipv6=yes");
    }
    if config.suricata {
        cmd.arg("-e").arg("enable_suricata=yes");
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let _ = tx.send(ProvisioningMessage::Output(
        "🚀 Starting Ansible playbook...".into(),
    ));

    let mut child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to spawn ansible-playbook: {}. Is Ansible installed?",
            e
        )
    })?;

    if let Some(pid) = child.id() {
        child_pid.store(pid, Ordering::SeqCst);
        let _ = tx.send(ProvisioningMessage::Output(format!(
            "📌 Process ID: {}",
            pid
        )));
    }

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    // Handle stdout
    let tx_stdout = tx.clone();
    let shutdown_stdout = Arc::clone(&shutdown);
    let stdout_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if shutdown_stdout.load(Ordering::SeqCst) {
                break;
            }
            let clean = strip_ansi(&line);
            if !is_timing_only_line(&clean) && !clean.trim().is_empty() {
                let _ = tx_stdout.send(ProvisioningMessage::Output(clean));
            }
        }
    });

    // Handle stderr
    let tx_stderr = tx.clone();
    let shutdown_stderr = Arc::clone(&shutdown);
    let stderr_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if shutdown_stderr.load(Ordering::SeqCst) {
                break;
            }
            let clean = strip_ansi(&line);
            if !clean.trim().is_empty() {
                let _ = tx_stderr.send(ProvisioningMessage::Output(format!("⚠️  {}", clean)));
            }
        }
    });

    // Wait for both tasks
    let _ = tokio::join!(stdout_task, stderr_task);

    // Check for shutdown signal
    if shutdown.load(Ordering::SeqCst) {
        let _ = tx.send(ProvisioningMessage::Output(
            "🛑 Shutdown requested, killing process...".into(),
        ));
        let _ = child.kill().await;
        let _ = tx.send(ProvisioningMessage::Complete(false));
        return Ok(());
    }

    let status = child
        .wait()
        .await
        .map_err(|e| format!("Failed to wait for child process: {}", e))?;

    // Cleanup SSH agent (Unix only — ssh-agent is a Unix daemon)
    #[cfg(unix)]
    if let Some(pid) = agent_pid {
        let _ = tx.send(ProvisioningMessage::Output(
            "🔒 Cleaning up SSH agent...".into(),
        ));
        let _ = std::process::Command::new("kill")
            .arg("-9")
            .arg(pid)
            .status();
    }

    let success = status.success();
    let exit_code = status.code().unwrap_or(-1);

    // Send prominent completion message
    let _ = tx.send(ProvisioningMessage::Output("".into()));
    let _ = tx.send(ProvisioningMessage::Output(
        "═══════════════════════════════════════════════════════════".into(),
    ));

    if success {
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(
            "    ✅ PROVISIONING COMPLETED SUCCESSFULLY".into(),
        ));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(format!(
            "    Exit Code: {}",
            exit_code
        )));
        let _ = tx.send(ProvisioningMessage::Output(format!(
            "    Server: {}",
            config.ip_address
        )));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(
            "    Your server is now configured and ready to use!".into(),
        ));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
    } else {
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(
            "    ❌ PROVISIONING FAILED".into(),
        ));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(format!(
            "    Exit Code: {}",
            exit_code
        )));
        let _ = tx.send(ProvisioningMessage::Output(format!(
            "    Server: {}",
            config.ip_address
        )));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(
            "    Check the output above for error details.".into(),
        ));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
    }

    let _ = tx.send(ProvisioningMessage::Output(
        "═══════════════════════════════════════════════════════════".into(),
    ));
    let _ = tx.send(ProvisioningMessage::Output("".into()));

    let _ = tx.send(ProvisioningMessage::Complete(success));
    Ok(())
}
