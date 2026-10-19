use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, Stroke, StrokeKind};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::config::{load_cache, save_cache, ProvisioningConfig};
use crate::provisioning::run_provisioning;
use crate::style::{setup_theme, theme_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavSection {
    Connection,
    Features,
    Security,
    Maintenance,
    Output,
}

impl NavSection {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            NavSection::Connection => "GATEWAY",
            NavSection::Features => "ASSETS",
            NavSection::Security => "SHIELD",
            NavSection::Maintenance => "NUCLEUS",
            NavSection::Output => "STREAM",
        }
    }

    pub(crate) fn icon(&self) -> &'static str {
        match self {
            NavSection::Connection => "🌐",
            NavSection::Features => "📦",
            NavSection::Security => "🔒",
            NavSection::Maintenance => "🛠",
            NavSection::Output => "📝",
        }
    }
}

pub(crate) enum ProvisioningMessage {
    Output(String),
    Error(String),
    Complete(bool),
}

pub(crate) struct RustsibleProvisioningApp {
    pub(crate) config: ProvisioningConfig,
    pub(crate) provisioning: bool,
    pub(crate) testing_connection: bool,
    pub(crate) output_lines: Vec<String>,
    pub(crate) result_message: Option<String>,
    pub(crate) error_message: Option<String>,
    pub(crate) connection_test_result: Option<String>,
    pub(crate) rx: Option<Receiver<ProvisioningMessage>>,
    pub(crate) thread_handle: Option<thread::JoinHandle<()>>,
    pub(crate) shutdown_signal: Arc<AtomicBool>,
    pub(crate) child_pid: Arc<AtomicU32>,
    pub(crate) term_signal: Arc<AtomicBool>,
    pub(crate) selected_section: NavSection,

    // Performance optimization fields
    pub(crate) config_needs_save: bool,
    pub(crate) last_save_time: Instant,
}

impl Default for RustsibleProvisioningApp {
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
            config_needs_save: false,
            last_save_time: Instant::now(),
        }
    }
}

impl RustsibleProvisioningApp {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>, term_signal: Arc<AtomicBool>) -> Self {
        setup_theme(&cc.egui_ctx);
        let mut app = Self::default();
        app.term_signal = term_signal;
        app
    }

    pub(crate) fn mark_config_changed(&mut self) {
        self.config_needs_save = true;
    }

    pub(crate) fn launch_provisioning(&mut self) {
        // Clean up any previous state first
        self.cleanup();

        // Force an immediate save before deploying
        if self.config_needs_save {
            let _ = save_cache(&self.config);
            self.config_needs_save = false;
        }

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

        let handle = std::thread::spawn(move || {
            let rt_res = tokio::runtime::Runtime::new();
            match rt_res {
                Ok(rt) => {
                    rt.block_on(async {
                        let _ = tx.send(ProvisioningMessage::Output(
                            "🚀 Initializing provisioning...".into(),
                        ));

                        match run_provisioning(
                            config,
                            tx.clone(),
                            shutdown_signal.clone(),
                            child_pid.clone(),
                        )
                        .await
                        {
                            Ok(_) => {
                                // run_provisioning already sends success/failure messages
                                // and Complete(bool) via the channel — nothing to add here.
                            }
                            Err(e) => {
                                // Setup-level failure (e.g. ansible-playbook not found)
                                let _ = tx.send(ProvisioningMessage::Error(format!(
                                    "❌ Provisioning failed: {}",
                                    e
                                )));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                            }
                        }
                    });

                    // Graceful shutdown with timeout
                    rt.shutdown_timeout(Duration::from_secs(5));
                }
                Err(e) => {
                    let _ = tx.send(ProvisioningMessage::Error(format!(
                        "Failed to create Tokio runtime: {}",
                        e
                    )));
                    let _ = tx.send(ProvisioningMessage::Complete(false));
                }
            }
        });

        self.thread_handle = Some(handle);
    }

    pub(crate) fn test_connection(&mut self) {
        self.testing_connection = true;
        self.connection_test_result = None;

        // Force an immediate save before testing connection
        if self.config_needs_save {
            let _ = save_cache(&self.config);
            self.config_needs_save = false;
        }

        let config = self.config.clone();
        let (tx, rx) = channel();
        self.rx = Some(rx);

        let _handle = std::thread::spawn(move || {
            let rt_res = tokio::runtime::Runtime::new();
            if let Ok(rt) = rt_res {
                rt.block_on(async {
                    let key_path = if config.ssh_key_path.starts_with("~/") {
                        dirs::home_dir().unwrap().join(&config.ssh_key_path[2..])
                    } else {
                        std::path::PathBuf::from(&config.ssh_key_path)
                    };

                    if !key_path.exists() {
                        let _ = tx.send(ProvisioningMessage::Error(format!(
                            "Key not found: {}",
                            key_path.display()
                        )));
                        let _ = tx.send(ProvisioningMessage::Complete(false));
                        return;
                    }

                    let mut auth_sock: Option<String> = None;
                    let mut agent_pid: Option<String> = None;

                    if !config.ssh_key_passphrase.is_empty() {
                        match crate::provisioning::setup_ssh_agent(
                            &key_path,
                            &config.ssh_key_passphrase,
                        )
                        .await
                        {
                            Ok((socket, pid)) => {
                                auth_sock = Some(socket);
                                agent_pid = Some(pid);
                            }
                            Err(e) => {
                                let _ = tx.send(ProvisioningMessage::Error(format!(
                                    "Agent Error: {}",
                                    e
                                )));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                                return;
                            }
                        }
                    }

                    let mut cmd = tokio::process::Command::new("ssh");
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
                                let _ = tx.send(ProvisioningMessage::Error(format!(
                                    "SSH Denied: {}",
                                    stderr.trim()
                                )));
                                let _ = tx.send(ProvisioningMessage::Complete(false));
                            }
                        }
                        Err(e) => {
                            let _ =
                                tx.send(ProvisioningMessage::Error(format!("Exec Failed: {}", e)));
                            let _ = tx.send(ProvisioningMessage::Complete(false));
                        }
                    };

                    if let Some(pid) = agent_pid {
                        let _ = std::process::Command::new("kill")
                            .arg("-9")
                            .arg(pid)
                            .status();
                    }
                });
            }
        });
    }

    pub(crate) fn cleanup(&mut self) {
        // Signal shutdown
        self.shutdown_signal.store(true, Ordering::SeqCst);

        // Asynchronously kill child process if running, rather than blocking the UI loop
        let pid = self.child_pid.load(Ordering::SeqCst);
        if pid != 0 {
            #[cfg(unix)]
            {
                std::thread::spawn(move || {
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
                });
            }
            self.child_pid.store(0, Ordering::SeqCst);
        }

        // Wait for thread to finish - avoid join() as it blocks UI on app close
        if let Some(_handle) = self.thread_handle.take() {
            // Because we spawn a detached async drop essentially
            // we'll just let the OS clean the thread since we're exiting.
        }

        // Clear receiver
        self.rx = None;
    }
}

impl Drop for RustsibleProvisioningApp {
    fn drop(&mut self) {
        self.cleanup();
        if self.config_needs_save {
            let _ = save_cache(&self.config);
        }
    }
}

impl eframe::App for RustsibleProvisioningApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.cleanup();
        if self.config_needs_save {
            let _ = save_cache(&self.config);
            self.config_needs_save = false;
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.term_signal.load(Ordering::Relaxed) {
            self.cleanup();
            if self.config_needs_save {
                let _ = save_cache(&self.config);
                self.config_needs_save = false;
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                if self.testing_connection {
                    match msg {
                        ProvisioningMessage::Error(e) => {
                            self.connection_test_result = Some(e);
                            self.testing_connection = false;
                        }
                        ProvisioningMessage::Complete(s) => {
                            self.connection_test_result = Some(if s {
                                "Verified".into()
                            } else {
                                "Failed".into()
                            });
                            self.testing_connection = false;
                        }
                        _ => {}
                    }
                } else {
                    match msg {
                        ProvisioningMessage::Output(line) => self.output_lines.push(line),
                        ProvisioningMessage::Error(line) => {
                            self.output_lines.push(format!("❯ ERR: {}", line))
                        }
                        ProvisioningMessage::Complete(success) => {
                            self.provisioning = false;
                            if success {
                                self.result_message = Some("INITIALIZATION COMPLETE".into());
                            } else {
                                self.error_message = Some("SYNC INTERRUPTED".into());
                            }
                        }
                    }
                }
            }
        }

        egui::SidePanel::left("v26_sidebar")
            .frame(
                egui::Frame::new()
                    .fill(theme_colors::SIDEBAR_BG)
                    .inner_margin(Margin::same(32)),
            )
            .exact_width(300.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.label(
                    RichText::new("RUSTSIBLE")
                        .size(28.0)
                        .strong()
                        .color(theme_colors::TEXT_BRIGHT)
                        .extra_letter_spacing(3.0),
                );

                ui.add_space(56.0);

                let sections = [
                    NavSection::Connection,
                    NavSection::Features,
                    NavSection::Security,
                    NavSection::Maintenance,
                    NavSection::Output,
                ];

                for section in sections {
                    let selected = self.selected_section == section;

                    let bg = if selected {
                        theme_colors::GLASS_SURFACE
                    } else {
                        Color32::TRANSPARENT
                    };
                    let stroke = if selected {
                        Stroke::new(1.0, theme_colors::ACCENT.linear_multiply(0.5))
                    } else {
                        Stroke::NONE
                    };

                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 48.0),
                        egui::Sense::click(),
                    );

                    if response.hovered() && !selected {
                        ui.painter().rect_filled(
                            rect,
                            CornerRadius::same(12),
                            Color32::from_rgba_premultiplied(255, 255, 255, 8),
                        );
                    }

                    ui.painter()
                        .rect(rect, CornerRadius::same(12), bg, stroke, StrokeKind::Inside);

                    ui.painter().text(
                        egui::pos2(rect.left() + 24.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        format!("{}  {}", section.icon(), section.label()),
                        if selected {
                            egui::FontId::proportional(16.0)
                        } else {
                            egui::FontId::proportional(15.0)
                        },
                        if selected {
                            theme_colors::TEXT_BRIGHT
                        } else {
                            theme_colors::TEXT_MED
                        },
                    );

                    if response.clicked() {
                        self.selected_section = section;
                        // Always save when switching tabs just to ensure consistency
                        if self.config_needs_save {
                            let _ = save_cache(&self.config);
                            self.config_needs_save = false;
                        }
                    }
                    ui.add_space(14.0);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(32.0);
                    let deploy_text = if self.provisioning {
                        "SYNCING..."
                    } else {
                        "INITIATE DEPLOY"
                    };
                    let deploy_btn = egui::Button::new(
                        RichText::new(deploy_text)
                            .size(16.0)
                            .strong()
                            .color(Color32::BLACK),
                    )
                    .fill(if self.provisioning {
                        Color32::from_rgb(255, 100, 50)
                    } else {
                        theme_colors::ACCENT
                    })
                    .corner_radius(CornerRadius::same(14))
                    .min_size(egui::vec2(ui.available_width(), 54.0));

                    if ui.add_enabled(!self.provisioning, deploy_btn).clicked() {
                        self.launch_provisioning();
                    }

                    if self.provisioning && ui.button("TERMINATE").clicked() {
                        self.cleanup();
                        self.provisioning = false;
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

        // Debounced saving mechanism
        if self.config_needs_save && self.last_save_time.elapsed() > Duration::from_millis(1500) {
            let _ = save_cache(&self.config);
            self.config_needs_save = false;
            self.last_save_time = Instant::now();
        }

        if self.provisioning || self.testing_connection {
            ctx.request_repaint();
        }
        // We occasionally request repaint if we have pending unsaved strokes?
        // No, egui will repaint on user input, and user input changes config_needs_save
        // the timer check will naturally hit. Just to be safe we can let it sleep until next input.
    }
}
