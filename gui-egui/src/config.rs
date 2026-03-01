use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProvisioningConfig {
    pub ip_address: String,
    pub ssh_user: String,
    #[serde(default)]
    pub connection_password: String,
    pub ssh_key_path: String,
    #[serde(default)]
    pub ssh_key_passphrase: String,
    pub hostname: String,
    #[serde(default)]
    pub added_user: String,
    #[serde(default)]
    pub user_password: String,
    #[serde(default = "default_true")]
    pub create_user: bool,
    pub fail2ban: bool,
    pub docker: bool,
    pub swap: bool,
    pub lemp: bool,
    pub devtools: bool,
    pub wordpress: bool,
    pub certbot: bool,
    pub system_hardening: bool,
    pub apparmor: bool,
    pub rootkit_detection: bool,
    pub file_integrity: bool,
    pub audit_logging: bool,
    pub log_monitoring: bool,
    pub advanced_protection: bool,
    #[serde(default)]
    pub ssh_2fa_totp: bool,
    #[serde(default)]
    pub ssh_2fa_fido2: bool,
    #[serde(default)]
    pub ssh_2fa_duo: bool,
    #[serde(default)]
    pub backups: bool,
    #[serde(default)]
    pub usb_restrictions: bool,
    // Devtools sub-tasks
    #[serde(default = "default_true")]
    pub install_neovim: bool,
    #[serde(default = "default_true")]
    pub install_nodejs: bool,
    #[serde(default = "default_true")]
    pub install_claude_code: bool,
    #[serde(default)]
    pub install_gemini: bool,
    #[serde(default)]
    pub install_kiro: bool,
    #[serde(default)]
    pub install_github_cli: bool,
    #[serde(default)]
    pub install_btop: bool,
    #[serde(default)]
    pub install_tldr: bool,
    #[serde(default)]
    pub install_lazygit: bool,
    #[serde(default)]
    pub install_tmux: bool,
    #[serde(default)]
    pub install_zsh: bool,
    #[serde(default)]
    pub install_ripgrep: bool,
    #[serde(default)]
    pub install_fd: bool,
    #[serde(default)]
    pub install_duf: bool,
    #[serde(default)]
    pub install_ncdu: bool,
    #[serde(default)]
    pub install_lnav: bool,
    #[serde(default)]
    pub install_uv: bool,
    #[serde(default)]
    pub install_fzf: bool,
    #[serde(default)]
    pub install_bat: bool,
    #[serde(default)]
    pub install_eza: bool,
    #[serde(default)]
    pub install_zoxide: bool,
    #[serde(default)]
    pub install_jq: bool,
    #[serde(default)]
    pub install_htop: bool,
    #[serde(default)]
    pub install_gping: bool,
    #[serde(default)]
    pub install_nmap: bool,
    #[serde(default)]
    pub install_autossh: bool,
    #[serde(default)]
    pub install_starship: bool,
    #[serde(default)]
    pub install_direnv: bool,
    #[serde(default)]
    pub install_fish: bool,
    #[serde(default)]
    pub install_micro: bool,
    #[serde(default)]
    pub install_ranger: bool,
    // Extra security tasks
    #[serde(default)]
    pub secure_shm: bool,
    #[serde(default)]
    pub lynis: bool,
    #[serde(default)]
    pub disable_ipv6: bool,
    #[serde(default)]
    pub suricata: bool,
    pub cron_jobs: bool,
    pub periodic_reboot: bool,
    pub reboot_hour: String,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        let default_key_path = dirs::home_dir()
            .map(|p| p.join(".ssh/id_rsa").to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            ip_address: String::new(),
            ssh_user: "root".to_string(),
            connection_password: String::new(),
            ssh_key_path: default_key_path,
            ssh_key_passphrase: String::new(),
            hostname: String::new(),
            added_user: "organic".to_string(),
            user_password: String::new(),
            create_user: true,
            fail2ban: true,
            docker: true,
            swap: true,
            lemp: false,
            devtools: true,
            wordpress: false,
            certbot: false,
            system_hardening: false,
            apparmor: false,
            rootkit_detection: false,
            file_integrity: false,
            audit_logging: false,
            log_monitoring: false,
            advanced_protection: false,
            ssh_2fa_totp: false,
            ssh_2fa_fido2: false,
            ssh_2fa_duo: false,
            backups: false,
            usb_restrictions: false,
            install_neovim: true,
            install_nodejs: true,
            install_claude_code: true,
            install_gemini: false,
            install_kiro: false,
            install_github_cli: true,
            install_btop: true,
            install_tldr: true,
            install_lazygit: true,
            install_tmux: true,
            install_zsh: true,
            install_ripgrep: true,
            install_fd: true,
            install_duf: true,
            install_ncdu: true,
            install_lnav: true,
            install_uv: false,
            install_fzf: true,
            install_bat: true,
            install_eza: true,
            install_zoxide: true,
            install_jq: true,
            install_htop: true,
            install_gping: true,
            install_nmap: true,
            install_autossh: true,
            install_starship: true,
            install_direnv: true,
            install_fish: false,
            install_micro: true,
            install_ranger: true,
            secure_shm: false,
            lynis: false,
            disable_ipv6: false,
            suricata: false,
            cron_jobs: true,
            periodic_reboot: false,
            reboot_hour: "3".to_string(),
        }
    }
}

pub fn get_cache_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ansible_provisioning_cache.json");
    path
}

pub fn load_cache() -> Result<ProvisioningConfig, String> {
    let cache_path = get_cache_path();
    if cache_path.exists() {
        let contents = fs::read_to_string(&cache_path)
            .map_err(|e| format!("Failed to read cache: {}", e))?;
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse cache: {}", e))
    } else {
        Ok(ProvisioningConfig::default())
    }
}

pub fn save_cache(config: &ProvisioningConfig) -> Result<(), String> {
    let cache_path = get_cache_path();
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&cache_path, json)
        .map_err(|e| format!("Failed to write cache: {}", e))
}
