use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

// macOS dark theme system colors (NSColor equivalents)
mod macos_colors {
    use eframe::egui::Color32;

    // Standard macOS dark mode backgrounds
    pub const WINDOW_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(45, 45, 47);
    pub const TOOLBAR_BG: Color32 = Color32::from_rgb(38, 38, 40);
    pub const GROUPED_BG: Color32 = Color32::from_rgb(44, 44, 46);
    pub const TERMINAL_BG: Color32 = Color32::from_rgb(24, 24, 24);

    // Text (NSColor.label equivalents)
    pub const LABEL_PRIMARY: Color32 = Color32::WHITE;
    pub const LABEL_SECONDARY: Color32 = Color32::from_rgb(152, 152, 157);
    pub const LABEL_TERTIARY: Color32 = Color32::from_rgb(99, 99, 102);

    // Accent
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(10, 132, 255);

    // Status (NSColor.system* equivalents)
    pub const SYSTEM_RED: Color32 = Color32::from_rgb(255, 69, 58);
    pub const SYSTEM_GREEN: Color32 = Color32::from_rgb(48, 209, 88);
    pub const SYSTEM_YELLOW: Color32 = Color32::from_rgb(255, 214, 10);
    pub const SYSTEM_ORANGE: Color32 = Color32::from_rgb(255, 159, 10);
    pub const SYSTEM_CYAN: Color32 = Color32::from_rgb(100, 210, 255);
    pub const SYSTEM_PURPLE: Color32 = Color32::from_rgb(191, 90, 242);

    // Borders
    pub const SEPARATOR: Color32 = Color32::from_rgb(56, 56, 58);
    pub const SELECTED_CONTENT_BG: Color32 = Color32::from_rgb(0, 88, 208);
}

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
            NavSection::Connection => "Connection",
            NavSection::Features => "Features",
            NavSection::Security => "Security",
            NavSection::Maintenance => "Maintenance",
            NavSection::Output => "Output",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProvisioningConfig {
    ip_address: String,
    ssh_user: String,
    ssh_key_path: String,
    hostname: String,
    fail2ban: bool,
    docker: bool,
    swap: bool,
    lemp: bool,
    devtools: bool,
    wordpress: bool,
    certbot: bool,
    system_hardening: bool,
    apparmor: bool,
    rootkit_detection: bool,
    file_integrity: bool,
    audit_logging: bool,
    log_monitoring: bool,
    advanced_protection: bool,
    cron_jobs: bool,
    periodic_reboot: bool,
    reboot_hour: String,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        let default_key_path = dirs::home_dir()
            .map(|p| p.join(".ssh/id_rsa").to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            ip_address: String::new(),
            ssh_user: "root".to_string(),
            ssh_key_path: default_key_path,
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
            reboot_hour: "3".to_string(),
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
    output_lines: Vec<String>,
    result_message: Option<String>,
    error_message: Option<String>,
    rx: Option<Receiver<ProvisioningMessage>>,
    // Thread lifecycle management
    thread_handle: Option<thread::JoinHandle<()>>,
    shutdown_signal: Arc<AtomicBool>,
    child_pid: Arc<AtomicU32>,
    // Signal handling for graceful shutdown
    term_signal: Arc<AtomicBool>,
    // Navigation
    selected_section: NavSection,
}

impl Default for AnsibleProvisioningApp {
    fn default() -> Self {
        let config = load_cache().unwrap_or_default();
        Self {
            config,
            provisioning: false,
            output_lines: Vec::new(),
            result_message: None,
            error_message: None,
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
        setup_custom_style(&cc.egui_ctx);
        let mut app = Self::default();
        app.term_signal = term_signal;
        app
    }

    fn launch_provisioning(&mut self) {
        self.provisioning = true;
        self.output_lines.clear();
        self.result_message = None;
        self.error_message = None;
        self.selected_section = NavSection::Output;

        // Reset shutdown signal for new provisioning run
        self.shutdown_signal.store(false, Ordering::SeqCst);
        self.child_pid.store(0, Ordering::SeqCst);

        let (tx, rx) = channel();
        self.rx = Some(rx);

        let config = self.config.clone();
        let shutdown_signal = Arc::clone(&self.shutdown_signal);
        let child_pid = Arc::clone(&self.child_pid);

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Err(e) = run_provisioning(config, tx.clone(), shutdown_signal.clone(), child_pid.clone()).await {
                    let _ = tx.send(ProvisioningMessage::Error(e));
                    let _ = tx.send(ProvisioningMessage::Complete(false));
                }
            });
            // Properly shutdown tokio runtime
            rt.shutdown_timeout(Duration::from_secs(2));
        });

        self.thread_handle = Some(handle);
    }

    /// Performs comprehensive cleanup of threads and child processes
    fn cleanup(&mut self) {
        println!("Starting comprehensive cleanup...");

        // Set shutdown signal
        self.shutdown_signal.store(true, Ordering::SeqCst);

        // Kill ansible-playbook child process if running
        let pid = self.child_pid.load(Ordering::SeqCst);
        if pid != 0 {
            println!("Killing ansible-playbook child process (PID: {})...", pid);
            #[cfg(unix)]
            {
                let _ = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            #[cfg(windows)]
            {
                let _ = std::process::Command::new("taskkill")
                    .arg("/F")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            self.child_pid.store(0, Ordering::SeqCst);
        }

        // Join background thread with timeout
        if let Some(handle) = self.thread_handle.take() {
            println!("Joining background thread...");
            match handle.join() {
                Ok(_) => println!("Background thread joined successfully"),
                Err(e) => println!("Warning: Thread join failed: {:?}", e),
            }
        }

        println!("Cleanup completed");
    }

    fn render_connection(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Connection")
            .size(15.0)
            .strong()
            .color(macos_colors::LABEL_PRIMARY));
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Server Address (IP or DNS)")
            .size(12.0)
            .color(macos_colors::LABEL_SECONDARY));
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.config.ip_address)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(13.0))
        );
        ui.add_space(8.0);

        ui.label(egui::RichText::new("SSH User")
            .size(12.0)
            .color(macos_colors::LABEL_SECONDARY));
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.config.ssh_user)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(13.0))
        );
        ui.add_space(8.0);

        ui.label(egui::RichText::new("SSH Private Key Path")
            .size(12.0)
            .color(macos_colors::LABEL_SECONDARY));
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.config.ssh_key_path)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(13.0))
        );
        ui.label(egui::RichText::new("Supports ~ for home directory")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Hostname (optional)")
            .size(12.0)
            .color(macos_colors::LABEL_SECONDARY));
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.config.hostname)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::proportional(13.0))
        );
    }

    fn render_features(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Features")
            .size(15.0)
            .strong()
            .color(macos_colors::LABEL_PRIMARY));
        ui.add_space(8.0);

        ui.checkbox(&mut self.config.fail2ban, egui::RichText::new("Fail2ban Intrusion Prevention").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.docker, egui::RichText::new("Docker & Docker Compose").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.swap, egui::RichText::new("Swap Memory (auto-sized)").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.lemp, egui::RichText::new("LEMP Stack (Nginx, MySQL, PHP)").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.devtools, egui::RichText::new("Development Tools (Neovim, Node.js, Claude Code)").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.wordpress, egui::RichText::new("WordPress CMS").size(13.0));
        ui.add_space(2.0);
        ui.checkbox(&mut self.config.certbot, egui::RichText::new("Certbot SSL/TLS Certificates").size(13.0));
    }

    fn render_security(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Security")
            .size(15.0)
            .strong()
            .color(macos_colors::LABEL_PRIMARY));
        ui.add_space(8.0);

        ui.checkbox(&mut self.config.system_hardening, egui::RichText::new("System Hardening").size(13.0));
        ui.label(egui::RichText::new("   Kernel hardening, secure shared memory")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.apparmor, egui::RichText::new("AppArmor Enforcement").size(13.0));
        ui.label(egui::RichText::new("   Mandatory access control")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.rootkit_detection, egui::RichText::new("Rootkit Detection (rkhunter)").size(13.0));
        ui.label(egui::RichText::new("   Daily scans for malware")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.file_integrity, egui::RichText::new("File Integrity Monitoring (AIDE)").size(13.0));
        ui.label(egui::RichText::new("   Tracks unauthorized file changes")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.audit_logging, egui::RichText::new("Audit Logging (auditd)").size(13.0));
        ui.label(egui::RichText::new("   System call auditing")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.log_monitoring, egui::RichText::new("Log Monitoring (Logwatch)").size(13.0));
        ui.label(egui::RichText::new("   Daily log analysis reports")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
        ui.add_space(4.0);

        ui.checkbox(&mut self.config.advanced_protection, egui::RichText::new("Advanced Protection").size(13.0));
        ui.label(egui::RichText::new("   2FA, Backups, USB restrictions")
            .size(11.0)
            .color(macos_colors::LABEL_TERTIARY));
    }

    fn render_maintenance(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Maintenance")
            .size(15.0)
            .strong()
            .color(macos_colors::LABEL_PRIMARY));
        ui.add_space(8.0);

        ui.checkbox(&mut self.config.cron_jobs, egui::RichText::new("Automated Updates & Cron Jobs").size(13.0));
        ui.add_space(4.0);
        ui.checkbox(&mut self.config.periodic_reboot, egui::RichText::new("Periodic System Reboot").size(13.0));

        if self.config.periodic_reboot {
            ui.add_space(8.0);

            ui.label(egui::RichText::new("Reboot Schedule")
                .size(12.0)
                .color(macos_colors::LABEL_SECONDARY));
            ui.add_space(2.0);
            egui::ComboBox::from_id_salt("reboot_hour")
                .selected_text(egui::RichText::new(format_reboot_schedule(&self.config.reboot_hour)).size(13.0))
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.reboot_hour, "1".to_string(), "Daily at 1:00 AM");
                    ui.selectable_value(&mut self.config.reboot_hour, "2".to_string(), "Daily at 2:00 AM");
                    ui.selectable_value(&mut self.config.reboot_hour, "3".to_string(), "Daily at 3:00 AM");
                    ui.selectable_value(&mut self.config.reboot_hour, "4".to_string(), "Daily at 4:00 AM");
                    ui.selectable_value(&mut self.config.reboot_hour, "5".to_string(), "Daily at 5:00 AM");
                    ui.selectable_value(&mut self.config.reboot_hour, "*/6".to_string(), "Every 6 hours");
                    ui.selectable_value(&mut self.config.reboot_hour, "*/12".to_string(), "Every 12 hours");
                    ui.selectable_value(&mut self.config.reboot_hour, "*/24".to_string(), "Every 24 hours");
                });
        }
    }

    fn render_output(&mut self, ui: &mut egui::Ui) {
        // Success banner
        if let Some(msg) = self.result_message.clone() {
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_premultiplied(48, 209, 88, 25))
                .stroke(egui::Stroke::new(0.5, macos_colors::SYSTEM_GREEN))
                .corner_radius(6.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Success")
                        .size(13.0)
                        .strong()
                        .color(macos_colors::SYSTEM_GREEN));
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new(&msg).size(12.0));
                });
            ui.add_space(8.0);
        }

        // Error banner
        if let Some(msg) = self.error_message.clone() {
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_premultiplied(255, 69, 58, 25))
                .stroke(egui::Stroke::new(0.5, macos_colors::SYSTEM_RED))
                .corner_radius(6.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Error")
                        .size(13.0)
                        .strong()
                        .color(macos_colors::SYSTEM_RED));
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            for line in msg.lines() {
                                let color = if line.contains("fatal:") || line.contains("FAILED!") || line.contains("ERROR") {
                                    macos_colors::SYSTEM_RED
                                } else {
                                    egui::Color32::from_rgb(220, 220, 220)
                                };
                                ui.label(egui::RichText::new(line)
                                    .font(egui::FontId::monospace(11.0))
                                    .color(color));
                            }
                        });
                });
            ui.add_space(8.0);
        }

        // Terminal header
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Output")
                .size(15.0)
                .strong()
                .color(macos_colors::LABEL_PRIMARY));
            if self.provisioning {
                ui.spinner();
            }
        });
        ui.add_space(4.0);

        // Terminal output area
        if self.provisioning || !self.output_lines.is_empty() {
            egui::Frame::new()
                .fill(macos_colors::TERMINAL_BG)
                .stroke(egui::Stroke::new(0.5, macos_colors::SEPARATOR))
                .corner_radius(6.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Provisioning Output")
                            .size(12.0)
                            .color(macos_colors::LABEL_SECONDARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let open_btn = ui.add(egui::Button::new(
                                egui::RichText::new("Open Log").size(11.0)
                            ).corner_radius(6.0));
                            if open_btn.clicked() {
                                if let Ok(repo_root) = get_repo_root() {
                                    let log_path = repo_root.join("provisioning.log");
                                    let _ = std::process::Command::new("open")
                                        .arg(&log_path)
                                        .spawn();
                                }
                            }
                            let copy_btn = ui.add(egui::Button::new(
                                egui::RichText::new("Copy Logs").size(11.0)
                            ).corner_radius(6.0));
                            if copy_btn.clicked() {
                                let all_output = self.output_lines.join("\n");
                                ui.ctx().copy_text(all_output);
                            }
                        });
                    });
                    ui.add_space(4.0);

                    egui::ScrollArea::vertical()
                        .max_height(ui.available_height() - 8.0)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            for line in &self.output_lines {
                                let (color, bold) = ansible_line_style(line);
                                let mut text = egui::RichText::new(line)
                                    .font(egui::FontId::monospace(11.0))
                                    .color(color);
                                if bold {
                                    text = text.strong();
                                }
                                ui.label(text);
                            }
                        });
                });
        } else {
            ui.label(egui::RichText::new("No output yet. Start provisioning to see results.")
                .size(12.0)
                .color(macos_colors::LABEL_TERTIARY));
        }
    }
}

impl Drop for AnsibleProvisioningApp {
    fn drop(&mut self) {
        println!("Drop trait triggered - ensuring cleanup...");
        self.cleanup();
    }
}

impl eframe::App for AnsibleProvisioningApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Application exiting - triggering cleanup...");
        self.cleanup();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for signal-based shutdown request
        if self.term_signal.load(Ordering::Relaxed) {
            println!("Signal received - initiating graceful shutdown...");
            self.cleanup();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Poll for messages
        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    ProvisioningMessage::Output(line) => {
                        self.output_lines.push(line);
                    }
                    ProvisioningMessage::Error(line) => {
                        self.output_lines.push(format!("ERROR: {}", line));
                    }
                    ProvisioningMessage::Complete(success) => {
                        self.provisioning = false;
                        if success {
                            self.result_message = Some("Provisioning completed successfully!".to_string());
                        } else {
                            // Collect recent error/fatal lines for context
                            let error_lines: Vec<&str> = self.output_lines.iter()
                                .filter(|l| l.contains("ERROR") || l.contains("fatal:") || l.contains("FAILED!") || l.contains("UNREACHABLE!"))
                                .map(|l| l.as_str())
                                .collect();
                            let detail = if error_lines.is_empty() {
                                // Show last 5 lines as context
                                let tail: Vec<&str> = self.output_lines.iter()
                                    .rev().take(5).collect::<Vec<_>>().into_iter().rev()
                                    .map(|l| l.as_str()).collect();
                                format!("Provisioning failed.\n\nLast output:\n{}", tail.join("\n"))
                            } else {
                                format!("Provisioning failed.\n\nErrors:\n{}", error_lines.join("\n"))
                            };
                            self.error_message = Some(detail);
                        }
                    }
                }
            }
        }

        // Auto-save config on changes
        let _ = save_cache(&self.config);

        // Toolbar
        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::new()
                .fill(macos_colors::TOOLBAR_BG)
                .stroke(egui::Stroke::new(0.5, macos_colors::SEPARATOR))
                .inner_margin(egui::vec2(12.0, 6.0)))
            .show(ctx, |ui| {
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    let button_text = if self.provisioning {
                        "Provisioning..."
                    } else {
                        "Start Provisioning"
                    };

                    let button = egui::Button::new(
                        egui::RichText::new(button_text).size(13.0).strong()
                    )
                    .fill(if self.provisioning {
                        macos_colors::GROUPED_BG
                    } else {
                        macos_colors::ACCENT_BLUE
                    })
                    .corner_radius(6.0)
                    .min_size(egui::vec2(140.0, 28.0));

                    let response = ui.add_enabled(!self.provisioning, button);

                    if response.clicked() {
                        self.launch_provisioning();
                    }

                    if self.provisioning {
                        ui.spinner();
                        ui.label(egui::RichText::new("Running...")
                            .size(12.0)
                            .color(macos_colors::LABEL_SECONDARY));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("Settings auto-saved")
                            .size(11.0)
                            .color(macos_colors::LABEL_TERTIARY));
                    });
                });
                ui.add_space(2.0);
            });

        // Sidebar
        egui::SidePanel::left("nav_sidebar")
            .resizable(false)
            .exact_width(180.0)
            .frame(egui::Frame::new()
                .fill(macos_colors::SIDEBAR_BG)
                .stroke(egui::Stroke::new(0.5, macos_colors::SEPARATOR))
                .inner_margin(egui::vec2(8.0, 12.0)))
            .show(ctx, |ui| {
                let sections = [
                    NavSection::Connection,
                    NavSection::Features,
                    NavSection::Security,
                    NavSection::Maintenance,
                    NavSection::Output,
                ];

                for section in sections {
                    let selected = self.selected_section == section;

                    // macOS sidebar row
                    let desired_size = egui::vec2(ui.available_width(), 26.0);
                    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

                    if selected {
                        // macOS accent selection highlight
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(5),
                            macos_colors::SELECTED_CONTENT_BG,
                        );
                    } else if response.hovered() {
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(5),
                            egui::Color32::from_rgba_premultiplied(255, 255, 255, 10),
                        );
                    }

                    ui.painter().text(
                        egui::pos2(rect.left() + 10.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        section.label(),
                        egui::FontId::proportional(13.0),
                        if selected {
                            macos_colors::LABEL_PRIMARY
                        } else {
                            macos_colors::LABEL_SECONDARY
                        },
                    );

                    if response.clicked() {
                        self.selected_section = section;
                    }
                }
            });

        // Detail pane
        egui::CentralPanel::default()
            .frame(egui::Frame::new()
                .fill(macos_colors::WINDOW_BG)
                .inner_margin(egui::vec2(20.0, 16.0)))
            .show(ctx, |ui| {
                match self.selected_section {
                    NavSection::Connection => self.render_connection(ui),
                    NavSection::Features => self.render_features(ui),
                    NavSection::Security => self.render_security(ui),
                    NavSection::Maintenance => self.render_maintenance(ui),
                    NavSection::Output => self.render_output(ui),
                }
            });

        // Request repaint for smooth updates
        if self.provisioning {
            ctx.request_repaint();
        }
    }
}

fn setup_custom_style(ctx: &egui::Context) {
    // Start from egui's built-in dark theme
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();

    // macOS dark mode panel colors
    style.visuals.panel_fill = macos_colors::WINDOW_BG;
    style.visuals.window_fill = macos_colors::WINDOW_BG;
    style.visuals.extreme_bg_color = macos_colors::TERMINAL_BG;

    // Widget fills — standard macOS grouped background tones
    style.visuals.widgets.noninteractive.bg_fill = macos_colors::GROUPED_BG;
    style.visuals.widgets.noninteractive.weak_bg_fill = egui::Color32::from_rgb(38, 38, 40);
    style.visuals.widgets.inactive.bg_fill = macos_colors::GROUPED_BG;
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(38, 38, 40);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(58, 58, 60);
    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(50, 50, 52);
    style.visuals.widgets.active.bg_fill = macos_colors::ACCENT_BLUE;
    style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(0, 112, 235);

    // macOS-standard rounding (6px — Settings/Finder style)
    let macos_rounding = egui::CornerRadius::same(6);
    style.visuals.widgets.noninteractive.corner_radius = macos_rounding;
    style.visuals.widgets.inactive.corner_radius = macos_rounding;
    style.visuals.widgets.hovered.corner_radius = macos_rounding;
    style.visuals.widgets.active.corner_radius = macos_rounding;

    // Compact macOS density
    style.spacing.item_spacing = egui::vec2(8.0, 4.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.indent = 16.0;
    style.spacing.interact_size = egui::vec2(40.0, 20.0);

    // Standard dark mode shadows
    style.visuals.popup_shadow = egui::Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 60),
    };
    style.visuals.window_shadow = egui::Shadow::NONE;

    style.animation_time = 0.15;

    ctx.set_style(style);
}

fn format_reboot_schedule(hour: &str) -> String {
    match hour {
        "1" => "Daily at 1:00 AM",
        "2" => "Daily at 2:00 AM",
        "3" => "Daily at 3:00 AM",
        "4" => "Daily at 4:00 AM",
        "5" => "Daily at 5:00 AM",
        "*/6" => "Every 6 hours",
        "*/12" => "Every 12 hours",
        "*/24" => "Every 24 hours",
        _ => "Daily at 3:00 AM",
    }.to_string()
}

fn get_cache_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".ansible_provisioning_cache.json");
    path
}

fn load_cache() -> Result<ProvisioningConfig, String> {
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

fn save_cache(config: &ProvisioningConfig) -> Result<(), String> {
    let cache_path = get_cache_path();
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&cache_path, json)
        .map_err(|e| format!("Failed to write cache: {}", e))
}

fn get_repo_root() -> Result<PathBuf, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;

    let mut current = exe_path.parent().ok_or("No parent directory")?;

    loop {
        let playbook_path = current.join("playbook.yml");
        if playbook_path.exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    std::env::current_dir()
        .map_err(|e| format!("Could not find repository root: {}", e))
}

/// Check if a line is just a timing line from profile_tasks callback
/// e.g. "Friday 20 February 2026  19:15:30 -0800 (0:00:00.008)       0:00:00.008 *******"
fn is_timing_only_line(s: &str) -> bool {
    // Lines that are ONLY timing info (no task name)
    // They match: "<weekday> <date> <time> <tz> (<delta>) <total> *****"
    (s.contains("*******") && !s.contains("TASK") && !s.contains("PLAY") && !s.contains("HANDLER"))
        || (s.starts_with("Friday ") || s.starts_with("Saturday ") || s.starts_with("Sunday ")
            || s.starts_with("Monday ") || s.starts_with("Tuesday ")
            || s.starts_with("Wednesday ") || s.starts_with("Thursday "))
            && s.contains("(0:") && s.ends_with("*******")
}

/// Returns (color, bold) matching macOS system colors for ansible output
fn ansible_line_style(line: &str) -> (egui::Color32, bool) {
    let trimmed = line.trim();

    // --- Red / errors ---
    if trimmed.starts_with("fatal:") || trimmed.contains("FAILED!") {
        return (macos_colors::SYSTEM_RED, true);
    }
    if trimmed.contains("ERROR") || trimmed.starts_with("ERROR:") {
        return (macos_colors::SYSTEM_RED, true);
    }
    if trimmed.contains("UNREACHABLE!") || trimmed.contains("unreachable=") {
        return (macos_colors::SYSTEM_RED, true);
    }

    // --- Yellow / changed ---
    if trimmed.starts_with("changed:") || trimmed.contains("changed:") {
        return (macos_colors::SYSTEM_YELLOW, false);
    }
    if trimmed.contains("[WARNING]") || trimmed.starts_with("WARNING") || trimmed.starts_with("[DEPRECATION") {
        return (macos_colors::SYSTEM_ORANGE, false);
    }

    // --- Green / ok ---
    if trimmed.starts_with("ok:") || trimmed.contains("ok:") {
        return (macos_colors::SYSTEM_GREEN, false);
    }
    if trimmed.contains("SUCCESS") || trimmed.contains("PROVISIONING COMPLETED") {
        return (macos_colors::SYSTEM_GREEN, true);
    }

    // --- Cyan / skipped ---
    if trimmed.starts_with("skipping:") || trimmed.contains("skipping:") {
        return (macos_colors::SYSTEM_CYAN, false);
    }
    if trimmed.starts_with("included:") {
        return (macos_colors::SYSTEM_CYAN, false);
    }

    // --- Bold white / structural ---
    if trimmed.starts_with("PLAY [") || trimmed.starts_with("PLAY RECAP") {
        return (macos_colors::LABEL_PRIMARY, true);
    }
    if trimmed.starts_with("TASK [") {
        return (macos_colors::ACCENT_BLUE, true);
    }
    if trimmed.starts_with("RUNNING HANDLER") {
        return (macos_colors::ACCENT_BLUE, true);
    }

    // --- Purple / recap stats ---
    if trimmed.contains("ok=") && trimmed.contains("changed=") {
        return (macos_colors::SYSTEM_PURPLE, false);
    }

    // --- Dim gray / timing & separators ---
    if trimmed.contains("0:00:") || trimmed.starts_with("====") || trimmed.starts_with("----") {
        return (macos_colors::LABEL_TERTIARY, false);
    }
    if trimmed.starts_with("Playbook run took") || trimmed.starts_with("TASKS RECAP") || trimmed.starts_with("PLAYBOOK RECAP") {
        return (macos_colors::LABEL_SECONDARY, true);
    }

    // --- Log file line ---
    if trimmed.starts_with("Log file:") {
        return (macos_colors::LABEL_TERTIARY, false);
    }

    // --- Default ---
    (egui::Color32::from_rgb(200, 200, 200), false)
}

/// Strip ANSI escape codes from a string.
/// Handles real ESC (\x1b), and literal representations (\033, \u000033, \u001b)
/// that appear in ansible's YAML-formatted output.
fn strip_ansi(s: &str) -> String {
    // First strip real ESC char sequences
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    // Then strip literal escape representations from YAML output:
    //   \u000033[1;36m  \u001b[0m  \033[1;36m
    // Catch-all: strip any remaining escape-like patterns from YAML output
    // Matches: \u000033[...m  \u001b[...m  \033[...m  and NUL-padded variants
    if let Ok(re) = regex::Regex::new(r"(?:\\u[0]*(?:1[bB]|33)|\\033|\x00+33)\[[\d;]*m") {
        result = re.replace_all(&result, "").to_string();
    }
    result
}

fn bool_to_yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

async fn run_provisioning(
    config: ProvisioningConfig,
    tx: Sender<ProvisioningMessage>,
    shutdown_signal: Arc<AtomicBool>,
    child_pid: Arc<AtomicU32>,
) -> Result<(), String> {
    let repo_root = get_repo_root()?;
    let playbook_path = repo_root.join("playbook.yml");

    if !playbook_path.exists() {
        return Err(format!("Playbook not found at: {}", playbook_path.display()));
    }

    let ssh_key_path = if config.ssh_key_path.starts_with("~/") {
        dirs::home_dir()
            .ok_or("Could not determine home directory")?
            .join(&config.ssh_key_path[2..])
    } else {
        PathBuf::from(&config.ssh_key_path)
    };

    if !ssh_key_path.exists() {
        return Err(format!("SSH key not found at: {}", ssh_key_path.display()));
    }

    let mut cmd = Command::new("ansible-playbook");
    cmd.current_dir(&repo_root);
    // Disable ANSI color codes — the GUI does its own coloring
    cmd.env("ANSIBLE_NOCOLOR", "1");
    cmd.env("ANSIBLE_FORCE_COLOR", "0");
    cmd.env("NO_COLOR", "1");
    cmd.arg("playbook.yml");
    cmd.arg("-e").arg(format!("target_ip={}", config.ip_address));
    cmd.arg("-e").arg(format!("target_user={}", config.ssh_user));
    cmd.arg("-e").arg(format!("ssh_key_path={}", ssh_key_path.display()));

    if !config.hostname.is_empty() {
        cmd.arg("-e").arg(format!("target_hostname={}", config.hostname));
    }

    cmd.arg("-e").arg(format!("prompt_enable_fail2ban={}", bool_to_yes_no(config.fail2ban)));
    cmd.arg("-e").arg(format!("prompt_install_docker={}", bool_to_yes_no(config.docker)));
    cmd.arg("-e").arg(format!("prompt_install_lemp={}", bool_to_yes_no(config.lemp)));
    cmd.arg("-e").arg(format!("prompt_enable_swap={}", bool_to_yes_no(config.swap)));
    cmd.arg("-e").arg(format!("prompt_enable_cron_jobs={}", bool_to_yes_no(config.cron_jobs)));
    cmd.arg("-e").arg(format!("prompt_install_dev_tools={}", bool_to_yes_no(config.devtools)));
    cmd.arg("-e").arg(format!("prompt_install_wordpress={}", bool_to_yes_no(config.wordpress)));
    cmd.arg("-e").arg(format!("prompt_install_certbot={}", bool_to_yes_no(config.certbot)));
    cmd.arg("-e").arg(format!("prompt_enable_periodic_reboot={}", bool_to_yes_no(config.periodic_reboot)));
    cmd.arg("-e").arg(format!("prompt_reboot_hour={}", config.reboot_hour));

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
    if config.advanced_protection {
        cmd.arg("-e").arg("enable_ssh_2fa=yes");
        cmd.arg("-e").arg("enable_backups=yes");
        cmd.arg("-e").arg("enable_usb_restrictions=yes");
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn ansible-playbook: {}", e))?;

    // Store child PID for cleanup
    if let Some(pid) = child.id() {
        child_pid.store(pid, Ordering::SeqCst);
        println!("Ansible-playbook child process spawned with PID: {}", pid);
    }

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Create log file for this run
    let log_path = repo_root.join("provisioning.log");
    let log_file = std::sync::Arc::new(std::sync::Mutex::new(
        std::fs::File::create(&log_path).ok()
    ));
    let _ = tx.send(ProvisioningMessage::Output(
        format!("Log file: {}", log_path.display())
    ));

    let tx_stdout = tx.clone();
    let tx_stderr = tx.clone();
    let shutdown_stdout = Arc::clone(&shutdown_signal);
    let shutdown_stderr = Arc::clone(&shutdown_signal);
    let log_stdout = Arc::clone(&log_file);
    let log_stderr = Arc::clone(&log_file);

    let stdout_task = tokio::spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if shutdown_stdout.load(Ordering::SeqCst) {
                break;
            }
            let clean = strip_ansi(&line);
            let trimmed = clean.trim();
            // Skip empty lines and pure timing lines (noise from profile_tasks)
            if trimmed.is_empty() {
                continue;
            }
            if is_timing_only_line(trimmed) {
                // Still log to file, but don't show in GUI
                if let Ok(mut guard) = log_stdout.lock() {
                    if let Some(ref mut f) = *guard {
                        use std::io::Write;
                        let _ = writeln!(f, "{}", clean);
                    }
                }
                continue;
            }
            // Write to log file
            if let Ok(mut guard) = log_stdout.lock() {
                if let Some(ref mut f) = *guard {
                    use std::io::Write;
                    let _ = writeln!(f, "{}", clean);
                }
            }
            let _ = tx_stdout.send(ProvisioningMessage::Output(clean));
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if shutdown_stderr.load(Ordering::SeqCst) {
                break;
            }
            let clean = strip_ansi(&line);
            let trimmed = clean.trim();
            if trimmed.is_empty() {
                continue;
            }
            // Write to log file (always)
            if let Ok(mut guard) = log_stderr.lock() {
                if let Some(ref mut f) = *guard {
                    use std::io::Write;
                    let _ = writeln!(f, "STDERR: {}", clean);
                }
            }
            // Categorize: deprecation/warning lines as Output (not Error), real errors as Error
            let is_warning = trimmed.contains("[WARNING]")
                || trimmed.contains("[DEPRECATION WARNING]")
                || trimmed.starts_with("Origin:")
                || trimmed.starts_with("Use `ansible_facts")
                || trimmed.contains("^ column");
            // Skip verbose deprecation context lines (source code snippets)
            let is_deprecation_context = trimmed.starts_with("Origin:")
                || trimmed.contains("^ column")
                || (trimmed.len() > 2 && trimmed.chars().next().map_or(false, |c| c.is_ascii_digit())
                    && trimmed.contains("  "));
            if is_deprecation_context {
                continue; // Skip noisy deprecation context lines from GUI
            }
            if is_warning {
                let _ = tx_stderr.send(ProvisioningMessage::Output(format!("WARNING: {}", clean)));
            } else {
                let _ = tx_stderr.send(ProvisioningMessage::Error(clean));
            }
        }
    });

    // Wait for child process or shutdown signal
    let status = loop {
        if shutdown_signal.load(Ordering::SeqCst) {
            println!("Shutdown signal received - killing child process...");
            let _ = child.kill().await;
            break child.wait().await
                .map_err(|e| format!("Failed to wait for process: {}", e))?;
        }

        // Poll child status
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                // Still running, sleep briefly
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                return Err(format!("Failed to check process status: {}", e));
            }
        }
    };

    // Abort tasks to prevent hanging
    stdout_task.abort();
    stderr_task.abort();

    // Give tasks a moment to finish naturally
    let _ = tokio::time::timeout(Duration::from_millis(500), async {
        let _ = stdout_task.await;
        let _ = stderr_task.await;
    }).await;

    if !status.success() {
        let exit_code = status.code().unwrap_or(-1);
        let _ = tx.send(ProvisioningMessage::Error(
            format!("Provisioning failed with exit code: {}", exit_code)
        ));
    }
    let _ = tx.send(ProvisioningMessage::Complete(status.success()));

    Ok(())
}

/// Cleans up any existing rustsible-gui processes before starting.
/// This prevents multiple instances from running and mimics macOS Force Quit behavior.
/// Includes aggressive zombie process cleanup with parent killing and process group termination.
fn cleanup_previous_instances() -> Result<(), String> {
    println!("Cleaning up previous instances...");

    // Get the current process ID to avoid killing ourselves
    let current_pid = std::process::id();

    // Initialize system with process information
    let mut system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything())
    );

    // Refresh process list
    system.refresh_processes();

    // Find all rustsible-gui processes
    let target_processes: Vec<_> = system
        .processes()
        .iter()
        .filter(|(_, process)| {
            let pid = process.pid().as_u32();
            let name = process.name();

            // Match process name and exclude current process
            pid != current_pid && name.contains("rustsible-gui")
        })
        .map(|(pid, _)| *pid)
        .collect();

    if target_processes.is_empty() {
        println!("No running instances found.");
        return Ok(());
    }

    println!("Found {} process(es) to terminate", target_processes.len());

    // Phase 0: Kill parent processes first (releases zombie children)
    #[cfg(unix)]
    {
        println!("Phase 0: Killing parent processes to release zombies...");
        system.refresh_processes();

        let mut parent_pids = std::collections::HashSet::new();

        for pid in &target_processes {
            if let Some(process) = system.process(*pid) {
                if let Some(parent_pid) = process.parent() {
                    let parent_pid_u32 = parent_pid.as_u32();

                    // Don't kill ourselves or init/launchd (PID 1)
                    if parent_pid_u32 != current_pid && parent_pid_u32 != 1 {
                        if let Some(parent_process) = system.process(parent_pid) {
                            let parent_name = parent_process.name();

                            // Only kill parent if it's related to our app (not system processes)
                            if !parent_name.contains("launchd")
                                && !parent_name.contains("init")
                                && !parent_name.contains("systemd") {
                                parent_pids.insert(parent_pid_u32);
                            }
                        }
                    }
                }
            }
        }

        for parent_pid in parent_pids {
            println!("  Killing parent PID {}...", parent_pid);
            let _ = std::process::Command::new("kill")
                .arg("-9")
                .arg(parent_pid.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .ok();
        }

        thread::sleep(Duration::from_millis(200));
    }

    // Phase 1: Graceful termination with SIGTERM (or equivalent)
    println!("Phase 1: Attempting graceful termination...");
    for pid in &target_processes {
        match system.process(*pid) {
            Some(process) => {
                println!("  Sending TERM signal to PID {}...", pid.as_u32());
                if !process.kill() {
                    println!("  Warning: Failed to send termination signal to PID {}", pid.as_u32());
                }
            }
            None => {
                println!("  Process {} already terminated", pid.as_u32());
            }
        }
    }

    // Wait for graceful shutdown
    thread::sleep(Duration::from_millis(300));

    // Phase 2: Force kill remaining processes (SIGKILL or equivalent)
    println!("Phase 2: Force killing remaining processes...");
    system.refresh_processes();

    for pid in &target_processes {
        if let Some(process) = system.process(*pid) {
            println!("  Force quitting PID {}...", pid.as_u32());

            // Kill with force - sysinfo's kill_with sends SIGKILL on Unix
            if cfg!(unix) {
                use sysinfo::Signal;
                if !process.kill_with(Signal::Kill).unwrap_or(false) {
                    println!("  Warning: Failed to force kill PID {}", pid.as_u32());
                }
            } else {
                // On Windows, regular kill is already forceful
                if !process.kill() {
                    println!("  Warning: Failed to force kill PID {}", pid.as_u32());
                }
            }
        }
    }

    // Wait for processes to fully terminate
    thread::sleep(Duration::from_millis(500));

    // Phase 3: System-wide cleanup using OS tools (Unix only)
    #[cfg(unix)]
    {
        println!("Phase 3: System-wide cleanup...");

        // Use killall -9 to forcefully kill any remaining processes
        let _ = std::process::Command::new("killall")
            .arg("-9")
            .arg("rustsible-gui")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok(); // Ignore failures gracefully

        // Use pkill -9 with pattern matching as a secondary cleanup
        let _ = std::process::Command::new("pkill")
            .arg("-9")
            .arg("-f")
            .arg("rustsible-gui")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok(); // Ignore failures gracefully

        // Short wait after system-wide cleanup
        thread::sleep(Duration::from_millis(300));

        println!("System-wide cleanup completed.");
    }

    // Phase 4: macOS-specific aggressive cleanup using launchctl
    #[cfg(target_os = "macos")]
    {
        println!("Phase 4: macOS launchctl force quit...");
        system.refresh_processes();

        for pid in &target_processes {
            if system.process(*pid).is_some() {
                let pid_str = pid.as_u32().to_string();
                println!("  Using launchctl to kill PID {}...", pid_str);

                // Try launchctl kill SIGKILL
                let _ = std::process::Command::new("launchctl")
                    .arg("kill")
                    .arg("SIGKILL")
                    .arg(format!("system/{}", pid_str))
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .ok();

                // Also try with gui domain
                let _ = std::process::Command::new("launchctl")
                    .arg("kill")
                    .arg("SIGKILL")
                    .arg(format!("gui/{}", pid_str))
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .ok();

                // Try removing the process from launchd entirely
                let _ = std::process::Command::new("launchctl")
                    .arg("remove")
                    .arg(format!("system/{}", pid_str))
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .ok();
            }
        }

        thread::sleep(Duration::from_millis(300));
        println!("macOS launchctl cleanup completed.");
    }

    // Phase 5: Nuclear option - Process group kill
    #[cfg(unix)]
    {
        println!("Phase 5: Process group termination (nuclear option)...");
        system.refresh_processes();

        let mut process_groups = std::collections::HashSet::new();

        // Collect all process group IDs
        for pid in &target_processes {
            if system.process(*pid).is_some() {
                // Get process group ID using ps command
                if let Ok(output) = std::process::Command::new("ps")
                    .arg("-o")
                    .arg("pgid=")
                    .arg("-p")
                    .arg(pid.as_u32().to_string())
                    .output()
                {
                    if let Ok(pgid_str) = String::from_utf8(output.stdout) {
                        if let Ok(pgid) = pgid_str.trim().parse::<i32>() {
                            // Don't kill our own process group or PID 1
                            if pgid > 1 && pgid != current_pid as i32 {
                                process_groups.insert(pgid);
                            }
                        }
                    }
                }
            }
        }

        // Kill entire process groups
        for pgid in process_groups {
            println!("  Killing process group {}...", pgid);

            // Use negative PID to kill entire process group
            let _ = std::process::Command::new("kill")
                .arg("-9")
                .arg(format!("-{}", pgid))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .ok();
        }

        thread::sleep(Duration::from_millis(300));
        println!("Process group termination completed.");
    }

    // Final verification
    system.refresh_processes();
    let final_check: Vec<_> = system
        .processes()
        .iter()
        .filter(|(_, process)| {
            let pid = process.pid().as_u32();
            let name = process.name();
            pid != current_pid && name.contains("rustsible-gui")
        })
        .collect();

    if !final_check.is_empty() {
        println!(
            "Warning: {} zombie process(es) still detected after aggressive cleanup",
            final_check.len()
        );

        // Log details about remaining zombies
        for (pid, process) in &final_check {
            println!(
                "  Zombie PID: {} Name: {} Status: {:?}",
                pid.as_u32(),
                process.name(),
                process.status()
            );
        }
    } else {
        println!("Cleanup completed successfully - all processes terminated.");
    }

    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    // Clean up any previous instances before starting
    if let Err(e) = cleanup_previous_instances() {
        eprintln!("Warning: Failed to clean up previous instances: {}", e);
        eprintln!("Continuing anyway...");
    }

    println!("Launching Rustsible GUI...");

    // Set up Unix signal handlers for graceful shutdown
    let term_signal = Arc::new(AtomicBool::new(false));

    #[cfg(unix)]
    {
        use signal_hook::consts::{SIGTERM, SIGINT};
        use signal_hook::flag;

        // Register SIGTERM handler (kill <pid>)
        if let Err(e) = flag::register(SIGTERM, Arc::clone(&term_signal)) {
            eprintln!("Warning: Failed to register SIGTERM handler: {}", e);
        } else {
            println!("Registered SIGTERM handler for graceful shutdown");
        }

        // Register SIGINT handler (Ctrl+C)
        if let Err(e) = flag::register(SIGINT, Arc::clone(&term_signal)) {
            eprintln!("Warning: Failed to register SIGINT handler: {}", e);
        } else {
            println!("Registered SIGINT handler for graceful shutdown");
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([860.0, 580.0])
            .with_min_inner_size([700.0, 450.0])
            .with_title("Rustsible"),
        ..Default::default()
    };

    let term_signal_clone = Arc::clone(&term_signal);
    eframe::run_native(
        "Rustsible",
        options,
        Box::new(move |cc| Ok(Box::new(AnsibleProvisioningApp::new(cc, term_signal_clone)))),
    )
}
