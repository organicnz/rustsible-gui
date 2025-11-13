use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProvisioningConfig {
    // Connection Information
    pub ip_address: String,
    pub ssh_user: String,
    pub ssh_password: String,
    pub hostname: String,

    // Core Features
    pub fail2ban: bool,
    pub docker: bool,
    pub swap: bool,
    pub lemp: bool,
    pub devtools: bool,
    pub wordpress: bool,
    pub certbot: bool,

    // Security Clusters
    pub system_hardening: bool,
    pub apparmor: bool,
    pub rootkit_detection: bool,
    pub file_integrity: bool,
    pub audit_logging: bool,
    pub log_monitoring: bool,
    pub advanced_protection: bool,

    // Maintenance Settings
    pub cron_jobs: bool,
    pub periodic_reboot: bool,
    pub reboot_hour: String,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        Self {
            ip_address: String::new(),
            ssh_user: String::from("root"),
            ssh_password: String::new(),
            hostname: String::new(),
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
            cron_jobs: true,
            periodic_reboot: false,
            reboot_hour: String::from("3"),
        }
    }
}

fn get_cache_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ansible_provisioning_cache.json");
    path
}

#[tauri::command]
fn load_cache() -> Result<ProvisioningConfig, String> {
    let cache_path = get_cache_path();

    if cache_path.exists() {
        let contents = fs::read_to_string(&cache_path)
            .map_err(|e| format!("Failed to read cache: {}", e))?;

        let config: ProvisioningConfig = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse cache: {}", e))?;

        Ok(config)
    } else {
        Ok(ProvisioningConfig::default())
    }
}

#[tauri::command]
fn save_cache(config: ProvisioningConfig) -> Result<(), String> {
    let cache_path = get_cache_path();

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&cache_path, json)
        .map_err(|e| format!("Failed to write cache: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn launch_provisioning(config: ProvisioningConfig) -> Result<String, String> {
    // Save the configuration to cache first
    save_cache(config.clone())?;

    // Build the ansible-playbook command
    let mut cmd = Command::new("ansible-playbook");

    // Add playbook file (assuming it's in the parent directory)
    cmd.arg("../playbook.yml");

    // Add inventory
    cmd.arg("-i").arg("../inventory.ini");

    // Add connection parameters
    cmd.arg("-u").arg(&config.ssh_user);
    cmd.arg("-k"); // Ask for SSH password

    // Add extra vars for the playbook
    let mut extra_vars = vec![
        format!("ip_address={}", config.ip_address),
        format!("install_docker={}", config.docker),
        format!("enable_swap={}", config.swap),
        format!("install_lemp={}", config.lemp),
        format!("install_dev_tools={}", config.devtools),
        format!("install_wordpress={}", config.wordpress),
        format!("install_certbot={}", config.certbot),
        format!("enable_kernel_hardening={}", config.system_hardening),
        format!("enable_apparmor={}", config.apparmor),
        format!("enable_rkhunter={}", config.rootkit_detection),
        format!("enable_aide={}", config.file_integrity),
        format!("enable_auditd={}", config.audit_logging),
        format!("enable_logwatch={}", config.log_monitoring),
        format!("enable_cron_jobs={}", config.cron_jobs),
        format!("enable_periodic_reboot={}", config.periodic_reboot),
        format!("cron_reboot_hour={}", config.reboot_hour),
    ];

    // Add hostname if provided
    if !config.hostname.is_empty() {
        extra_vars.push(format!("server_hostname={}", config.hostname));
    }

    // Add all extra vars as a single -e argument
    cmd.arg("-e").arg(extra_vars.join(" "));

    // Execute the command
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute ansible-playbook: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!("Provisioning completed successfully!\n\n{}", stdout))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Provisioning failed:\n\n{}", stderr))
    }
}

#[tauri::command]
fn validate_ip(ip: String) -> bool {
    // Simple IP validation
    ip.split('.').count() == 4 &&
    ip.split('.').all(|part| part.parse::<u8>().is_ok())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_cache,
            save_cache,
            launch_provisioning,
            validate_ip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
