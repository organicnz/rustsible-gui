use eframe::egui::{self, Color32, Stroke, StrokeKind, CornerRadius, RichText, Margin};
use std::path::PathBuf;
use std::process::Stdio;
use std::os::unix::fs::PermissionsExt;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use sysinfo::System;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

mod config;
mod style;

use config::{ProvisioningConfig, load_cache, save_cache};
use style::{macos_v26_colors, setup_macos_v26_style, crystal_card, ansible_line_style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavSection {
    Connection,
    Features,
    Security,
    Maintenance,
    Output,
}

impl NavSection {
    fn label(&self) -> &'static str {
        match self {
            NavSection::Connection => "GATEWAY",
            NavSection::Features => "ASSETS",
            NavSection::Security => "SHIELD",
            NavSection::Maintenance => "NUCLEUS",
            NavSection::Output => "STREAM",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            NavSection::Connection => "🌐", 
            NavSection::Features => "📦",   
            NavSection::Security => "🔒",   
            NavSection::Maintenance => "🛠", 
            NavSection::Output => "📝",     
        }
    }
}

enum ProvisioningMessage {
    Output(String),
    Error(String),
    Complete(bool),
}

struct AnsibleProvisioningApp {
    config: ProvisioningConfig,
    provisioning: bool,
    testing_connection: bool,
    output_lines: Vec<String>,
    result_message: Option<String>,
    error_message: Option<String>,
    connection_test_result: Option<String>,
    rx: Option<Receiver<ProvisioningMessage>>,
    thread_handle: Option<thread::JoinHandle<()>>,
    shutdown_signal: Arc<AtomicBool>,
    child_pid: Arc<AtomicU32>,
    term_signal: Arc<AtomicBool>,
    selected_section: NavSection,
}

impl Default for AnsibleProvisioningApp {
    fn default() -> Self {
        let config = load_cache().unwrap_or_default();
        Self {
            config,
            provisioning: false,
            testing_connection: false,
            output_lines: Vec::new(),
            result_message: None,
            error_message: None,
            connection_test_result: None,
            rx: None,
            thread_handle: None,
            shutdown_signal: Arc::new(AtomicBool::new(false)),
            child_pid: Arc::new(AtomicU32::new(0)),
            term_signal: Arc::new(AtomicBool::new(false)),
            selected_section: NavSection::Connection,
        }
    }
}

impl AnsibleProvisioningApp {
    fn new(cc: &eframe::CreationContext<'_>, term_signal: Arc<AtomicBool>) -> Self {
        setup_macos_v26_style(&cc.egui_ctx);
        let mut app = Self::default();
        app.term_signal = term_signal;
        app
    }

    fn launch_provisioning(&mut self) {
        // Clean up any previous state first
        self.cleanup();
        
        self.provisioning = true;
        self.testing_connection = false;
        self.output_lines.clear();
        self.result_message = None;
        self.error_message = None;
        self.selected_section = NavSection::Output;

        self.shutdown_signal.store(false, Ordering::SeqCst);
        self.child_pid.store(0, Ordering::SeqCst);

        let (tx, rx) = channel();
        self.rx = Some(rx);

        let config = self.config.clone();
        let shutdown_signal = Arc::clone(&self.shutdown_signal);
        let child_pid = Arc::clone(&self.child_pid);

        // Save cache before starting
        if let Err(e) = save_cache(&config) {
            let _ = tx.send(ProvisioningMessage::Error(format!("Failed to save config: {}", e)));
        }

        let handle = std::thread::spawn(move || {
            let rt_res = tokio::runtime::Runtime::new();
            match rt_res {
                Ok(rt) => {
                    rt.block_on(async {
                        let _ = tx.send(ProvisioningMessage::Output("🚀 Initializing provisioning...".into()));
                        
                        match run_provisioning(config, tx.clone(), shutdown_signal.clone(), child_pid.clone()).await {
                            Ok(_) => {
                                let _ = tx.send(ProvisioningMessage::Output("✅ Provisioning completed successfully".into()));
                            }
                            Err(e) => {
                                let _ = tx.send(ProvisioningMessage::Error(format!("❌ Provisioning failed: {}", e)));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                            }
                        }
                    });
                    
                    // Graceful shutdown with timeout
                    rt.shutdown_timeout(Duration::from_secs(5));
                }
                Err(e) => {
                    let _ = tx.send(ProvisioningMessage::Error(format!("Failed to create Tokio runtime: {}", e)));
                    let _ = tx.send(ProvisioningMessage::Complete(false));
                }
            }
        });

        self.thread_handle = Some(handle);
    }

    fn test_connection(&mut self) {
        self.testing_connection = true;
        self.connection_test_result = None;

        let config = self.config.clone();
        let (tx, rx) = channel();
        self.rx = Some(rx);

        // Save cache before testing
        let _ = save_cache(&config);

        let _handle = std::thread::spawn(move || {
            let rt_res = tokio::runtime::Runtime::new();
            if let Ok(rt) = rt_res {
                rt.block_on(async {
                    let key_path = if config.ssh_key_path.starts_with("~/") {
                        dirs::home_dir().unwrap().join(&config.ssh_key_path[2..])
                    } else {
                        PathBuf::from(&config.ssh_key_path)
                    };

                    if !key_path.exists() {
                        let _ = tx.send(ProvisioningMessage::Error(format!("Key not found: {}", key_path.display())));
                        let _ = tx.send(ProvisioningMessage::Complete(false));
                        return;
                    }

                    let mut auth_sock: Option<String> = None;
                    let mut agent_pid: Option<String> = None;

                    if !config.ssh_key_passphrase.is_empty() {
                        match setup_ssh_agent(&key_path, &config.ssh_key_passphrase).await {
                            Ok((socket, pid)) => {
                                auth_sock = Some(socket);
                                agent_pid = Some(pid);
                            },
                            Err(e) => {
                                let _ = tx.send(ProvisioningMessage::Error(format!("Agent Error: {}", e)));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                                return;
                            }
                        }
                    }

                    let mut cmd = Command::new("ssh");
                    cmd.arg("-o").arg("BatchMode=yes");
                    cmd.arg("-o").arg("ConnectTimeout=10");
                    cmd.arg("-o").arg("StrictHostKeyChecking=no");
                    if let Some(sock) = &auth_sock {
                        cmd.env("SSH_AUTH_SOCK", sock);
                    }
                    cmd.arg("-i").arg(&key_path);
                    cmd.arg(format!("{}@{}", config.ssh_user, config.ip_address));
                    cmd.arg("echo 'Online'");
                    
                    match cmd.output().await {
                        Ok(output) => {
                            if output.status.success() {
                                let _ = tx.send(ProvisioningMessage::Complete(true));
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                let _ = tx.send(ProvisioningMessage::Error(format!("SSH Denied: {}", stderr.trim())));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(ProvisioningMessage::Error(format!("Exec Failed: {}", e)));
                            let _ = tx.send(ProvisioningMessage::Complete(false));
                        }
                    };

                    if let Some(pid) = agent_pid {
                        let _ = std::process::Command::new("kill").arg("-9").arg(pid).status();
                    }
                });
            }
        });
    }

    fn cleanup(&mut self) {
        // Signal shutdown
        self.shutdown_signal.store(true, Ordering::SeqCst);
        
        // Kill child process if running
        let pid = self.child_pid.load(Ordering::SeqCst);
        if pid != 0 {
            #[cfg(unix)]
            {
                // Try graceful termination first (SIGTERM)
                let _ = std::process::Command::new("kill")
                    .arg(pid.to_string())
                    .status();
                
                // Wait a bit for graceful shutdown
                std::thread::sleep(Duration::from_millis(500));
                
                // Force kill if still running (SIGKILL)
                let _ = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .status();
            }
            self.child_pid.store(0, Ordering::SeqCst);
        }
        
        // Wait for thread to finish
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
        
        // Clear receiver
        self.rx = None;
    }

    fn render_v26_header(&self, ui: &mut egui::Ui, title: &str, subtitle: &str) {
        ui.vertical(|ui| {
            ui.add_space(12.0);
            ui.label(RichText::new(title).size(38.0).strong().color(macos_v26_colors::TEXT_BRIGHT).extra_letter_spacing(-0.8));
            ui.label(RichText::new(subtitle).size(15.0).color(macos_v26_colors::ACCENT_LIGHT).extra_letter_spacing(0.5));
        });
        ui.add_space(36.0);
    }

    fn render_connection(&mut self, ui: &mut egui::Ui) {
            self.render_v26_header(ui, "Core Gateway", "Establish secure infrastructure synchronization.");

            let mut config_changed = false;

            crystal_card(ui, |ui: &mut egui::Ui| {
                egui::Grid::new("conn_grid").spacing([32.0, 24.0]).show(ui, |ui| {
                    ui.label(RichText::new("PROTOCOL HOST").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    if ui.add(egui::TextEdit::singleline(&mut self.config.ip_address).desired_width(450.0)).changed() {
                        config_changed = true;
                    }
                    ui.end_row();

                    ui.label(RichText::new("IDENTITY").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    if ui.add(egui::TextEdit::singleline(&mut self.config.ssh_user).desired_width(450.0)).changed() {
                        config_changed = true;
                    }
                    ui.end_row();

                    ui.label(RichText::new("PASSWORD").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    if ui.add(egui::TextEdit::singleline(&mut self.config.connection_password).desired_width(450.0).password(true)).changed() {
                        config_changed = true;
                    }
                    ui.end_row();

                    ui.label(RichText::new("RSA ARCHIVE").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    ui.horizontal(|ui| {
                        if ui.add(egui::TextEdit::singleline(&mut self.config.ssh_key_path).desired_width(340.0)).changed() {
                            config_changed = true;
                        }
                        if ui.button("BROWSE").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.config.ssh_key_path = path.to_string_lossy().to_string();
                                config_changed = true;
                            }
                        }
                    });
                    ui.end_row();

                    ui.label(RichText::new("KEY PASSPHRASE").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    if ui.add(egui::TextEdit::singleline(&mut self.config.ssh_key_passphrase).desired_width(450.0).password(true)).changed() {
                        config_changed = true;
                    }
                    ui.end_row();

                    ui.label(RichText::new("ALIAS").strong().color(macos_v26_colors::TEXT_LOW).size(12.0).extra_letter_spacing(1.0));
                    if ui.add(egui::TextEdit::singleline(&mut self.config.hostname).desired_width(450.0)).changed() {
                        config_changed = true;
                    }
                    ui.end_row();
                });


                ui.add_space(32.0);
                ui.horizontal(|ui| {
                    let btn_text = if self.testing_connection { "PROBING..." } else { "VALIDATE GATEWAY" };
                    let btn = egui::Button::new(RichText::new(btn_text).strong().color(macos_v26_colors::ACCENT_LIGHT))
                        .min_size(egui::vec2(220.0, 44.0));

                    if ui.add_enabled(!self.testing_connection && !self.provisioning, btn).clicked() {
                        self.test_connection();
                    }

                    if let Some(result) = &self.connection_test_result {
                        let color = if result == "Verified" { macos_v26_colors::SUCCESS } else { macos_v26_colors::ERROR };
                        ui.label(RichText::new(format!("❯ {}", result)).color(color).strong().size(16.0));
                    }
                });
            });

            ui.add_space(40.0);

            crystal_card(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.config.create_user, "").changed() {
                        config_changed = true;
                    }
                    ui.label(RichText::new("DEPLOY SYSTEM IDENTITY").size(18.0).strong().color(macos_v26_colors::TEXT_BRIGHT));
                });

                if self.config.create_user {
                    ui.add_space(24.0);
                    ui.indent("user_indent", |ui| {
                        egui::Grid::new("user_grid").spacing([32.0, 20.0]).show(ui, |ui| {
                            ui.label(RichText::new("UID").strong().color(macos_v26_colors::TEXT_LOW).size(12.0));
                            if ui.add(egui::TextEdit::singleline(&mut self.config.added_user).desired_width(450.0)).changed() {
                                config_changed = true;
                            }
                            ui.end_row();

                            ui.label(RichText::new("PASSCODE").strong().color(macos_v26_colors::TEXT_LOW).size(12.0));
                            if ui.add(egui::TextEdit::singleline(&mut self.config.user_password).desired_width(450.0).password(true)).changed() {
                                config_changed = true;
                            }
                            ui.end_row();
                        });
                    });
                }
            });

            // Auto-save when any field changes
            if config_changed {
                let _ = save_cache(&self.config);
            }
        }

    fn render_features(&mut self, ui: &mut egui::Ui) {
            self.render_v26_header(ui, "Resource Assets", "Deploy high-performance environment clusters.");

            let mut config_changed = false;

            crystal_card(ui, |ui: &mut egui::Ui| {
                ui.label(RichText::new("CLUSTER CORE").small().strong().color(macos_v26_colors::TEXT_LOW).extra_letter_spacing(1.5));
                ui.add_space(16.0);
                if ui.checkbox(&mut self.config.docker, RichText::new("Docker Orchestration Matrix").size(17.0)).changed() {
                    config_changed = true;
                }
                ui.add_space(20.0);

                if ui.checkbox(&mut self.config.lemp, RichText::new("LEMP High-Performance Stack").size(17.0)).changed() {
                    config_changed = true;
                }
                if self.config.lemp {
                    ui.add_space(12.0);
                    ui.indent("lemp_indent", |ui: &mut egui::Ui| {
                        if ui.checkbox(&mut self.config.wordpress, "WordPress Platinum Engine").changed() {
                            config_changed = true;
                        }
                        if ui.checkbox(&mut self.config.certbot, "Quantum SSL Hardening").changed() {
                            config_changed = true;
                        }
                    });
                }
            });

            ui.add_space(40.0);

            crystal_card(ui, |ui: &mut egui::Ui| {
                ui.label(RichText::new("DEVELOPER ENVIRONMENT").small().strong().color(macos_v26_colors::TEXT_LOW).extra_letter_spacing(1.5));
                ui.add_space(16.0);
                if ui.checkbox(&mut self.config.devtools, RichText::new("Integrated IDE Subsystem").size(17.0).strong()).changed() {
                    config_changed = true;
                }

                if self.config.devtools {
                    ui.add_space(20.0);
                    ui.indent("dev_indent", |ui: &mut egui::Ui| {
                        config_changed |= ui.checkbox(&mut self.config.install_neovim, "Neovim Terminal IDE").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_micro, "Micro Text Editor").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_zsh, "Zsh Shell").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_fish, "Fish Shell").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_starship, "Starship Prompt").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_tmux, "tmux Multiplexer").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_nodejs, "Node.js 24 LTS").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_claude_code, "Claude Code").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_gemini, "Gemini AI CLI").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_kiro, "Kiro Tool").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_github_cli, "GitHub CLI (gh)").changed();

                        ui.add_space(8.0);
                        ui.label(RichText::new("Utilities").strong().color(macos_v26_colors::TEXT_MED));
                        config_changed |= ui.checkbox(&mut self.config.install_btop, "btop (System Monitor)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_htop, "htop (Process Viewer)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_ripgrep, "ripgrep (rg)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_fd, "fd-find (fd)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_fzf, "fzf (Fuzzy Finder)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_zoxide, "zoxide (Smarter cd)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_direnv, "direnv (Env Switcher)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_ranger, "ranger (File Manager)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_duf, "duf (Disk Usage)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_ncdu, "ncdu (Disk Analyzer)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_lnav, "lnav (Log Navigator)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_tldr, "tldr (Simplified Man)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_lazygit, "lazygit (Git TUI)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_uv, "uv (Fast Python PM)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_jq, "jq (JSON Processor)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_gping, "gping (Visual Ping)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_nmap, "nmap (Network Audit)").changed();
                        config_changed |= ui.checkbox(&mut self.config.install_autossh, "autossh (SSH Persistence)").changed();
                    });
                }
            });

            if config_changed {
                let _ = save_cache(&self.config);
            }
        }

    fn render_security(&mut self, ui: &mut egui::Ui) {
            self.render_v26_header(ui, "Shield Perimeter", "Activate multi-layered system hardening protocols.");

            let mut config_changed = false;

            ui.columns(2, |cols| {
                crystal_card(&mut cols[0], |ui: &mut egui::Ui| {
                    ui.label(RichText::new("THREAT MITIGATION").small().strong().color(macos_v26_colors::TEXT_LOW).extra_letter_spacing(1.0));
                    ui.add_space(16.0);
                    config_changed |= ui.checkbox(&mut self.config.fail2ban, "Fail2ban Shield").changed();
                    config_changed |= ui.checkbox(&mut self.config.apparmor, "AppArmor MAC").changed();
                    config_changed |= ui.checkbox(&mut self.config.rootkit_detection, "Malware Scan").changed();
                    config_changed |= ui.checkbox(&mut self.config.suricata, "Network IDS").changed();
                    config_changed |= ui.checkbox(&mut self.config.disable_ipv6, "Disable IPv6 Protocol").changed();
                });

                crystal_card(&mut cols[1], |ui: &mut egui::Ui| {
                    ui.label(RichText::new("ACCESS CONTROL").small().strong().color(macos_v26_colors::TEXT_LOW).extra_letter_spacing(1.0));
                    ui.add_space(16.0);
                    config_changed |= ui.checkbox(&mut self.config.ssh_2fa_totp, "2FA: Authenticator").changed();
                    config_changed |= ui.checkbox(&mut self.config.ssh_2fa_fido2, "2FA: FIDO2 Keys").changed();
                    config_changed |= ui.checkbox(&mut self.config.ssh_2fa_duo, "2FA: Duo Push").changed();
                    config_changed |= ui.checkbox(&mut self.config.usb_restrictions, "USB Hardware Lock").changed();
                    config_changed |= ui.checkbox(&mut self.config.backups, "Automated System Backups").changed();
                });
            });

            ui.add_space(40.0);

            crystal_card(ui, |ui: &mut egui::Ui| {
                ui.label(RichText::new("NUCLEUS HARDENING").small().strong().color(macos_v26_colors::TEXT_LOW).extra_letter_spacing(1.5));
                ui.add_space(20.0);
                ui.horizontal(|ui: &mut egui::Ui| {
                    config_changed |= ui.checkbox(&mut self.config.system_hardening, "Kernel Tuning").changed();
                    ui.add_space(32.0);
                    config_changed |= ui.checkbox(&mut self.config.file_integrity, "AIDE Sync").changed();
                    ui.add_space(32.0);
                    config_changed |= ui.checkbox(&mut self.config.audit_logging, "Auditd Stream").changed();
                });
            });

            if config_changed {
                let _ = save_cache(&self.config);
            }
        }

    fn render_maintenance(&mut self, ui: &mut egui::Ui) {
            self.render_v26_header(ui, "System Nucleus", "Optimize resource allocation and cycle management.");

            let mut config_changed = false;

            crystal_card(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.vertical(|ui: &mut egui::Ui| {
                        if ui.checkbox(&mut self.config.swap, RichText::new("Intelligent Swap").size(17.0)).changed() {
                            config_changed = true;
                        }
                        ui.label(RichText::new("Automated RAM paging").small().color(macos_v26_colors::TEXT_LOW));
                    });
                    ui.add_space(80.0);
                    ui.vertical(|ui: &mut egui::Ui| {
                        if ui.checkbox(&mut self.config.cron_jobs, RichText::new("Automated Ops").size(17.0)).changed() {
                            config_changed = true;
                        }
                        ui.label(RichText::new("Security patch cycles").small().color(macos_v26_colors::TEXT_LOW));
                    });
                });
            });

            ui.add_space(40.0);

            crystal_card(ui, |ui: &mut egui::Ui| {
                if ui.checkbox(&mut self.config.periodic_reboot, RichText::new("Scheduled Refresh Cycles").size(17.0).strong()).changed() {
                    config_changed = true;
                }

                if self.config.periodic_reboot {
                    ui.add_space(24.0);
                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.label(RichText::new("PROTOCOL FREQUENCY").color(macos_v26_colors::TEXT_LOW).strong().size(12.0).extra_letter_spacing(1.0));
                        let combo_response = egui::ComboBox::from_id_salt("reboot_hour")
                            .selected_text(format_reboot_schedule(&self.config.reboot_hour))
                            .width(280.0)
                            .show_ui(ui, |ui| {
                                let mut changed = false;
                                changed |= ui.selectable_value(&mut self.config.reboot_hour, "1".into(), "01:00 Standard").changed();
                                changed |= ui.selectable_value(&mut self.config.reboot_hour, "3".into(), "03:00 Standard").changed();
                                changed |= ui.selectable_value(&mut self.config.reboot_hour, "5".into(), "05:00 Standard").changed();
                                changed |= ui.selectable_value(&mut self.config.reboot_hour, "*/6".into(), "Interval: 6 Hours").changed();
                                changed |= ui.selectable_value(&mut self.config.reboot_hour, "*/12".into(), "Interval: 12 Hours").changed();
                                changed
                            });
                        if combo_response.inner.unwrap_or(false) {
                            config_changed = true;
                        }
                    });
                }
            });

            if config_changed {
                let _ = save_cache(&self.config);
            }
        }

    fn render_output(&mut self, ui: &mut egui::Ui) {
        if let Some(msg) = self.result_message.clone() {
            ui.label(RichText::new(format!("❯ SUCCESS: {}", msg)).color(macos_v26_colors::SUCCESS).strong().size(22.0));
            ui.add_space(24.0);
        }
        if let Some(msg) = self.error_message.clone() {
            ui.label(RichText::new(format!("❯ INTERRUPT: {}", msg)).color(macos_v26_colors::ERROR).strong().size(22.0));
            ui.add_space(24.0);
        }

        ui.horizontal(|ui| {
            ui.label(RichText::new("System Stream").size(26.0).strong().color(macos_v26_colors::TEXT_BRIGHT));
            if self.provisioning { ui.spinner(); }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("PURGE LOG").clicked() {
                    self.output_lines.clear();
                    self.result_message = None;
                    self.error_message = None;
                }
            });
        });

        ui.add_space(24.0);
        egui::Frame::NONE
            .fill(macos_v26_colors::TERMINAL_BG)
            .corner_radius(CornerRadius::same(20))
            .inner_margin(28.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 20.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        if self.output_lines.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label(RichText::new("SYSTEM READY FOR INITIALIZATION").color(macos_v26_colors::TEXT_LOW).strong().extra_letter_spacing(2.0));
                            });
                        } else {
                            for line in &self.output_lines {
                                let (color, bold) = ansible_line_style(line);
                                let mut text = RichText::new(line).font(egui::FontId::monospace(14.0)).color(color);
                                if bold { text = text.strong(); }
                                ui.label(text);
                            }
                        }
                    });
            });
    }
}

impl Drop for AnsibleProvisioningApp {
    fn drop(&mut self) { self.cleanup(); }
}

impl eframe::App for AnsibleProvisioningApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) { self.cleanup(); }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.term_signal.load(Ordering::Relaxed) {
            self.cleanup();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                if self.testing_connection {
                    match msg {
                        ProvisioningMessage::Error(e) => { self.connection_test_result = Some(e); self.testing_connection = false; }
                        ProvisioningMessage::Complete(s) => { self.connection_test_result = Some(if s { "Verified".into() } else { "Failed".into() }); self.testing_connection = false; }
                        _ => {}
                    }
                } else {
                    match msg {
                        ProvisioningMessage::Output(line) => self.output_lines.push(line),
                        ProvisioningMessage::Error(line) => self.output_lines.push(format!("❯ ERR: {}", line)),
                        ProvisioningMessage::Complete(success) => {
                            self.provisioning = false;
                            if success { self.result_message = Some("INITIALIZATION COMPLETE".into()); }
                            else { self.error_message = Some("SYNC INTERRUPTED".into()); }
                        }
                    }
                }
            }
        }

        egui::SidePanel::left("v26_sidebar")
            .frame(egui::Frame::new().fill(macos_v26_colors::SIDEBAR_BG).inner_margin(Margin::same(32)))
            .exact_width(280.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.label(RichText::new("RUSTSIBLE").size(28.0).strong().color(macos_v26_colors::TEXT_BRIGHT).extra_letter_spacing(3.0));
                ui.label(RichText::new("PLATINUM v26").size(12.0).strong().color(macos_v26_colors::ACCENT).extra_letter_spacing(1.5));
                ui.add_space(56.0);
                
                let sections = [
                    NavSection::Connection, 
                    NavSection::Features, 
                    NavSection::Security, 
                    NavSection::Maintenance, 
                    NavSection::Output
                ];
                
                for section in sections {
                    let selected = self.selected_section == section;
                    
                    let bg = if selected { macos_v26_colors::GLASS_SURFACE } else { Color32::TRANSPARENT };
                    let stroke = if selected { Stroke::new(1.0, macos_v26_colors::ACCENT) } else { Stroke::NONE };
                    
                    let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 52.0), egui::Sense::click());
                    
                    if response.hovered() {
                         ui.painter().rect_filled(rect, CornerRadius::same(16), Color32::from_rgba_premultiplied(255, 255, 255, 10));
                    }
                    
                    ui.painter().rect(rect, CornerRadius::same(16), bg, stroke, StrokeKind::Inside);
                    
                    ui.painter().text(
                        egui::pos2(rect.left() + 24.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        format!("{}  {}", section.icon(), section.label()),
                        egui::FontId::proportional(15.0),
                        if selected { macos_v26_colors::TEXT_BRIGHT } else { macos_v26_colors::TEXT_MED }
                    );

                    if response.clicked() {
                        self.selected_section = section;
                        // Save cache when switching sections
                        let _ = save_cache(&self.config);
                    }
                    ui.add_space(14.0);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(32.0);
                    let deploy_text = if self.provisioning { "SYNCING..." } else { "INITIATE DEPLOY" };
                    let deploy_btn = egui::Button::new(RichText::new(deploy_text).size(17.0).strong().color(Color32::BLACK))
                        .fill(if self.provisioning { Color32::from_rgb(255, 100, 50) } else { macos_v26_colors::ACCENT })
                        .min_size(egui::vec2(ui.available_width(), 60.0));
                    
                    if ui.add_enabled(!self.provisioning, deploy_btn).clicked() { self.launch_provisioning(); }
                    
                    if self.provisioning { 
                         if ui.button("TERMINATE").clicked() { self.cleanup(); self.provisioning = false; }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(40.0);
                match self.selected_section {
                    NavSection::Connection => self.render_connection(ui),
                    NavSection::Features => self.render_features(ui),
                    NavSection::Security => self.render_security(ui),
                    NavSection::Maintenance => self.render_maintenance(ui),
                    NavSection::Output => self.render_output(ui),
                }
                ui.add_space(100.0);
            });
        });

        if self.provisioning || self.testing_connection { ctx.request_repaint(); }
    }
}

fn format_reboot_schedule(hour: &str) -> String {
    match hour { "1" => "01:00 Standard", "3" => "03:00 Standard", "5" => "05:00 Standard", "*/6" => "Interval: 6 Hours", "*/12" => "Interval: 12 Hours", _ => "03:00 Standard" }.into()
}

fn get_repo_root() -> Result<PathBuf, String> {
    let mut current = std::env::current_exe().map_err(|e| e.to_string())?;
    while let Some(parent) = current.parent() {
        if parent.join("playbook.yml").exists() { return Ok(parent.to_path_buf()); }
        current = parent.to_path_buf();
    }
    std::env::current_dir().map_err(|e| e.to_string())
}

fn is_timing_only_line(s: &str) -> bool {
    s.contains("*******") && !s.contains("TASK") && !s.contains("PLAY")
}

fn strip_ansi(s: &str) -> String {
    let re = regex::Regex::new(r"[\x1b\x00]\[[\d;]*m|\\u001b\[[\d;]*m|\\033\[[\d;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

fn bool_to_yes_no(v: bool) -> &'static str { if v { "yes" } else { "no" } }

async fn setup_ssh_agent(key_path: &std::path::Path, passphrase: &str) -> Result<(String, String), String> {
    let output = Command::new("ssh-agent").arg("-s").output().await.map_err(|e| e.to_string())?;
    let out_str = String::from_utf8_lossy(&output.stdout);
    let mut socket = String::new();
    let mut pid = String::new();
    for line in out_str.lines() {
        if line.contains("SSH_AUTH_SOCK=") { 
            socket = line.split(';').next().unwrap().replace("SSH_AUTH_SOCK=", ""); 
        }
        if line.contains("SSH_AGENT_PID=") { 
            pid = line.split(';').next().unwrap().replace("SSH_AGENT_PID=", ""); 
        }
    }
    
    if socket.is_empty() || pid.is_empty() { 
        return Err("Failed to initialize SSH agent".into()); 
    }

    let status = if !passphrase.is_empty() {
        let askpass = std::env::temp_dir().join(format!("askpass_{}.sh", pid));
        let script = format!("#!/bin/sh\ncat << 'EOF'\n{}\nEOF\n", passphrase);
        tokio::fs::write(&askpass, script).await.map_err(|e| e.to_string())?;
        tokio::fs::set_permissions(&askpass, PermissionsExt::from_mode(0o700)).await.map_err(|e| e.to_string())?;
        
        let s = Command::new("ssh-add")
            .arg(key_path)
            .env("SSH_AUTH_SOCK", &socket)
            .env("SSH_ASKPASS", &askpass)
            .env("DISPLAY", ":0")
            .env("SSH_ASKPASS_REQUIRE", "force")
            .status()
            .await
            .map_err(|e| e.to_string())?;
            
        let _ = tokio::fs::remove_file(askpass).await;
        s
    } else {
        Command::new("ssh-add")
            .arg(key_path)
            .env("SSH_AUTH_SOCK", &socket)
            .status()
            .await
            .map_err(|e| e.to_string())?
    };

    if !status.success() {
        let _ = std::process::Command::new("kill").arg(&pid).status();
        return Err("Decryption Failed".into());
    }

    Ok((socket, pid))
}

async fn run_provisioning(config: ProvisioningConfig, tx: Sender<ProvisioningMessage>, shutdown: Arc<AtomicBool>, child_pid: Arc<AtomicU32>) -> Result<(), String> {
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
        let _ = tx.send(ProvisioningMessage::Output("🔐 Unlocking SSH key...".into()));
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

    let _ = tx.send(ProvisioningMessage::Output(format!("📋 Building Ansible command...")));
    
    let mut cmd = Command::new("ansible-playbook");
    cmd.current_dir(&root).env("ANSIBLE_NOCOLOR", "1");
    if let Some(sock) = &auth_sock { cmd.env("SSH_AUTH_SOCK", sock); }
    
    cmd.arg("playbook.yml");
    cmd.arg("-e").arg(format!("target_ip={}", config.ip_address));
    cmd.arg("-e").arg(format!("target_user={}", config.ssh_user));
    if !config.connection_password.is_empty() { cmd.arg("-e").arg(format!("connection_password={}", config.connection_password)); }
    cmd.arg("-e").arg(format!("ssh_key_path={}", ssh_key_path.display()));
    
    if !config.hostname.is_empty() { cmd.arg("-e").arg(format!("target_hostname={}", config.hostname)); }
    cmd.arg("-e").arg(format!("prompt_create_user={}", bool_to_yes_no(config.create_user)));
    cmd.arg("-e").arg(format!("added_user={}", config.added_user));
    cmd.arg("-e").arg(format!("user_password={}", config.user_password));
    cmd.arg("-e").arg(format!("prompt_install_docker={}", bool_to_yes_no(config.docker)));
    cmd.arg("-e").arg(format!("prompt_install_lemp={}", bool_to_yes_no(config.lemp)));
    cmd.arg("-e").arg(format!("prompt_install_wordpress={}", bool_to_yes_no(config.wordpress)));
    cmd.arg("-e").arg(format!("prompt_install_certbot={}", bool_to_yes_no(config.certbot)));
    cmd.arg("-e").arg(format!("prompt_install_dev_tools={}", bool_to_yes_no(config.devtools)));
    cmd.arg("-e").arg(format!("prompt_install_neovim={}", bool_to_yes_no(config.install_neovim)));
    cmd.arg("-e").arg(format!("prompt_install_zsh={}", bool_to_yes_no(config.install_zsh)));
    cmd.arg("-e").arg(format!("prompt_install_tmux={}", bool_to_yes_no(config.install_tmux)));
    cmd.arg("-e").arg(format!("prompt_install_nodejs={}", bool_to_yes_no(config.install_nodejs)));
    cmd.arg("-e").arg(format!("prompt_install_claude_code={}", bool_to_yes_no(config.install_claude_code)));
    cmd.arg("-e").arg(format!("prompt_install_gemini={}", bool_to_yes_no(config.install_gemini)));
    cmd.arg("-e").arg(format!("prompt_install_kiro={}", bool_to_yes_no(config.install_kiro)));
    cmd.arg("-e").arg(format!("prompt_install_github_cli={}", bool_to_yes_no(config.install_github_cli)));
    cmd.arg("-e").arg(format!("prompt_install_btop={}", bool_to_yes_no(config.install_btop)));
    cmd.arg("-e").arg(format!("prompt_install_ripgrep={}", bool_to_yes_no(config.install_ripgrep)));
    cmd.arg("-e").arg(format!("prompt_install_fd={}", bool_to_yes_no(config.install_fd)));
    cmd.arg("-e").arg(format!("prompt_install_duf={}", bool_to_yes_no(config.install_duf)));
    cmd.arg("-e").arg(format!("prompt_install_ncdu={}", bool_to_yes_no(config.install_ncdu)));
    cmd.arg("-e").arg(format!("prompt_install_lnav={}", bool_to_yes_no(config.install_lnav)));
    cmd.arg("-e").arg(format!("prompt_install_tldr={}", bool_to_yes_no(config.install_tldr)));
    cmd.arg("-e").arg(format!("prompt_install_lazygit={}", bool_to_yes_no(config.install_lazygit)));
    cmd.arg("-e").arg(format!("prompt_install_uv={}", bool_to_yes_no(config.install_uv)));
    cmd.arg("-e").arg(format!("prompt_install_fzf={}", bool_to_yes_no(config.install_fzf)));
    cmd.arg("-e").arg(format!("prompt_install_bat={}", bool_to_yes_no(config.install_bat)));
    cmd.arg("-e").arg(format!("prompt_install_eza={}", bool_to_yes_no(config.install_eza)));
    cmd.arg("-e").arg(format!("prompt_install_zoxide={}", bool_to_yes_no(config.install_zoxide)));
    cmd.arg("-e").arg(format!("prompt_install_jq={}", bool_to_yes_no(config.install_jq)));
    cmd.arg("-e").arg(format!("prompt_install_htop={}", bool_to_yes_no(config.install_htop)));
    cmd.arg("-e").arg(format!("prompt_install_gping={}", bool_to_yes_no(config.install_gping)));
    cmd.arg("-e").arg(format!("prompt_install_nmap={}", bool_to_yes_no(config.install_nmap)));
    cmd.arg("-e").arg(format!("prompt_install_autossh={}", bool_to_yes_no(config.install_autossh)));
    cmd.arg("-e").arg(format!("prompt_install_starship={}", bool_to_yes_no(config.install_starship)));
    cmd.arg("-e").arg(format!("prompt_install_direnv={}", bool_to_yes_no(config.install_direnv)));
    cmd.arg("-e").arg(format!("prompt_install_fish={}", bool_to_yes_no(config.install_fish)));
    cmd.arg("-e").arg(format!("prompt_install_micro={}", bool_to_yes_no(config.install_micro)));
    cmd.arg("-e").arg(format!("prompt_install_ranger={}", bool_to_yes_no(config.install_ranger)));
    cmd.arg("-e").arg(format!("prompt_enable_fail2ban={}", bool_to_yes_no(config.fail2ban)));
    cmd.arg("-e").arg(format!("prompt_enable_swap={}", bool_to_yes_no(config.swap)));
    cmd.arg("-e").arg(format!("prompt_enable_cron_jobs={}", bool_to_yes_no(config.cron_jobs)));
    cmd.arg("-e").arg(format!("prompt_enable_periodic_reboot={}", bool_to_yes_no(config.periodic_reboot)));
    cmd.arg("-e").arg(format!("prompt_reboot_hour={}", config.reboot_hour));

    if config.system_hardening { cmd.arg("-e").arg("enable_kernel_hardening=yes"); }
    if config.apparmor { cmd.arg("-e").arg("enable_apparmor=yes"); }
    if config.rootkit_detection { cmd.arg("-e").arg("enable_rkhunter=yes"); }
    if config.file_integrity { cmd.arg("-e").arg("enable_aide=yes"); }
    if config.audit_logging { cmd.arg("-e").arg("enable_auditd=yes"); }
    if config.log_monitoring { cmd.arg("-e").arg("enable_logwatch=yes"); }
    
    if config.ssh_2fa_totp || config.ssh_2fa_fido2 || config.ssh_2fa_duo { 
        cmd.arg("-e").arg("enable_ssh_2fa=yes"); 
        if config.ssh_2fa_fido2 { cmd.arg("-e").arg("enable_ssh_2fa_fido2=yes"); }
        if config.ssh_2fa_duo { cmd.arg("-e").arg("enable_ssh_2fa_duo=yes"); }
    }
    
    if config.backups { cmd.arg("-e").arg("enable_backups=yes"); }
    if config.usb_restrictions { cmd.arg("-e").arg("enable_usb_restrictions=yes"); }
    if config.disable_ipv6 { cmd.arg("-e").arg("disable_ipv6=yes"); }
    if config.suricata { cmd.arg("-e").arg("enable_suricata=yes"); }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    
    let _ = tx.send(ProvisioningMessage::Output("🚀 Starting Ansible playbook...".into()));
    
    let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn ansible-playbook: {}. Is Ansible installed?", e))?;
    
    if let Some(pid) = child.id() { 
        child_pid.store(pid, Ordering::SeqCst);
        let _ = tx.send(ProvisioningMessage::Output(format!("📌 Process ID: {}", pid)));
    }

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    
    // Handle stdout
    let tx_stdout = tx.clone();
    let shutdown_stdout = Arc::clone(&shutdown);
    let stdout_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if shutdown_stdout.load(Ordering::SeqCst) { break; }
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
            if shutdown_stderr.load(Ordering::SeqCst) { break; }
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
        let _ = tx.send(ProvisioningMessage::Output("🛑 Shutdown requested, killing process...".into()));
        let _ = child.kill().await;
        let _ = tx.send(ProvisioningMessage::Complete(false));
        return Ok(());
    }
    
    let status = child.wait().await.map_err(|e| format!("Failed to wait for child process: {}", e))?;
    
    // Cleanup SSH agent
    if let Some(pid) = agent_pid { 
        let _ = tx.send(ProvisioningMessage::Output("🔒 Cleaning up SSH agent...".into()));
        let _ = std::process::Command::new("kill").arg(pid).status(); 
    }
    
    let success = status.success();
    let exit_code = status.code().unwrap_or(-1);
    
    // Send prominent completion message
    let _ = tx.send(ProvisioningMessage::Output("".into()));
    let _ = tx.send(ProvisioningMessage::Output("═══════════════════════════════════════════════════════════".into()));
    
    if success {
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output("    ✅ PROVISIONING COMPLETED SUCCESSFULLY".into()));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(format!("    Exit Code: {}", exit_code)));
        let _ = tx.send(ProvisioningMessage::Output(format!("    Server: {}", config.ip_address)));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output("    Your server is now configured and ready to use!".into()));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
    } else {
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output("    ❌ PROVISIONING FAILED".into()));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output(format!("    Exit Code: {}", exit_code)));
        let _ = tx.send(ProvisioningMessage::Output(format!("    Server: {}", config.ip_address)));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
        let _ = tx.send(ProvisioningMessage::Output("    Check the output above for error details.".into()));
        let _ = tx.send(ProvisioningMessage::Output("".into()));
    }
    
    let _ = tx.send(ProvisioningMessage::Output("═══════════════════════════════════════════════════════════".into()));
    let _ = tx.send(ProvisioningMessage::Output("".into()));
    
    let _ = tx.send(ProvisioningMessage::Complete(success));
    Ok(())
}

fn cleanup_previous_instances() -> Result<(), String> {
    use sysinfo::{ProcessRefreshKind, RefreshKind};
    let mut system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything())
    );
    system.refresh_processes();
    let current_pid = std::process::id();
    let current_pid_u32 = current_pid as u32;
    for (pid, process) in system.processes() {
        let pid_u32 = pid.as_u32();
        if process.name().contains("rustsible-gui") && pid_u32 != current_pid_u32 {
            let _ = process.kill();
        }
    }
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    // Set up global panic hook to catch crashes
    std::panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_else(|| "unknown".to_string());
        let message = panic_info.payload().downcast_ref::<&str>().map(|s| *s).unwrap_or_else(|| {
            panic_info.payload().downcast_ref::<String>().map(|s| &s[..]).unwrap_or("no message")
        });
        let log = format!("Panic at {}: {}\n", location, message);
        let _ = std::fs::write("crash.log", log);
        eprintln!("CRASH DETECTED: Check crash.log for details");
    }));

    // Temporarily disabled to debug crash
    // let _ = cleanup_previous_instances();
    let term_signal = Arc::new(AtomicBool::new(false));
    #[cfg(unix)]
    {
        use signal_hook::consts::{SIGTERM, SIGINT};
        let _ = signal_hook::flag::register(SIGTERM, Arc::clone(&term_signal));
        let _ = signal_hook::flag::register(SIGINT, Arc::clone(&term_signal));
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Rustsible Platinum v26"),
        ..Default::default()
    };
    eframe::run_native("Rustsible Platinum", options, Box::new(move |cc| Ok(Box::new(AnsibleProvisioningApp::new(cc, term_signal)))))
}
