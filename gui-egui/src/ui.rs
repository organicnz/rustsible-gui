use crate::app::RustsibleProvisioningApp;
use crate::provisioning::format_reboot_schedule;
use crate::style::{ansible_line_style, crystal_card, theme_colors};
use eframe::egui::{self, CornerRadius, RichText};

impl RustsibleProvisioningApp {
    pub(crate) fn render_header(&self, ui: &mut egui::Ui, title: &str, subtitle: &str) {
        ui.vertical(|ui| {
            ui.add_space(12.0);
            ui.label(
                RichText::new(title)
                    .size(38.0)
                    .strong()
                    .color(theme_colors::TEXT_BRIGHT)
                    .extra_letter_spacing(-0.8),
            );
            ui.label(
                RichText::new(subtitle)
                    .size(15.0)
                    .color(theme_colors::ACCENT_LIGHT)
                    .extra_letter_spacing(0.5),
            );
        });
        ui.add_space(36.0);
    }

    pub(crate) fn render_connection(&mut self, ui: &mut egui::Ui) {
        self.render_header(
            ui,
            "Core Gateway",
            "Establish secure infrastructure synchronization.",
        );

        crystal_card(ui, |ui: &mut egui::Ui| {
            egui::Grid::new("conn_grid")
                .spacing([32.0, 24.0])
                .show(ui, |ui| {
                    let field_width = 500.0;
                    ui.label(
                        RichText::new("PROTOCOL HOST")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.config.ip_address)
                                .desired_width(field_width),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.end_row();

                    ui.label(
                        RichText::new("IDENTITY")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.config.ssh_user)
                                .desired_width(field_width),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.end_row();

                    ui.label(
                        RichText::new("PASSWORD")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.config.connection_password)
                                .desired_width(field_width)
                                .password(true),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.end_row();

                    ui.label(
                        RichText::new("RSA ARCHIVE")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                egui::TextEdit::singleline(&mut self.config.ssh_key_path)
                                    .desired_width(field_width - 100.0),
                            )
                            .changed()
                        {
                            self.mark_config_changed();
                        }
                        if ui.button("BROWSE").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.config.ssh_key_path = path.to_string_lossy().to_string();
                                self.mark_config_changed();
                            }
                        }
                    });
                    ui.end_row();

                    ui.label(
                        RichText::new("KEY PASSPHRASE")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.config.ssh_key_passphrase)
                                .desired_width(field_width)
                                .password(true),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.end_row();

                    ui.label(
                        RichText::new("ALIAS")
                            .strong()
                            .color(theme_colors::TEXT_LOW)
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.config.hostname)
                                .desired_width(field_width),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.end_row();
                });

            ui.add_space(32.0);
            ui.horizontal(|ui| {
                let btn_text = if self.testing_connection {
                    "PROBING..."
                } else {
                    "VALIDATE GATEWAY"
                };
                let btn = egui::Button::new(
                    RichText::new(btn_text)
                        .strong()
                        .color(theme_colors::ACCENT_LIGHT),
                )
                .min_size(egui::vec2(220.0, 44.0));

                if ui
                    .add_enabled(!self.testing_connection && !self.provisioning, btn)
                    .clicked()
                {
                    self.test_connection();
                }

                if let Some(result) = &self.connection_test_result {
                    let color = if result == "Verified" {
                        theme_colors::SUCCESS
                    } else {
                        theme_colors::ERROR
                    };
                    ui.label(
                        RichText::new(format!("❯ {}", result))
                            .color(color)
                            .strong()
                            .size(16.0),
                    );
                }
            });
        });

        ui.add_space(40.0);

        crystal_card(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.config.create_user, "").changed() {
                    self.mark_config_changed();
                }
                ui.label(
                    RichText::new("DEPLOY SYSTEM IDENTITY")
                        .size(18.0)
                        .strong()
                        .color(theme_colors::TEXT_BRIGHT),
                );
            });

            if self.config.create_user {
                ui.add_space(24.0);
                ui.indent("user_indent", |ui| {
                    egui::Grid::new("user_grid")
                        .spacing([32.0, 20.0])
                        .show(ui, |ui| {
                            let field_width = 460.0;
                            ui.label(
                                RichText::new("UID")
                                    .strong()
                                    .color(theme_colors::TEXT_LOW)
                                    .size(12.0),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.config.added_user)
                                        .desired_width(field_width),
                                )
                                .changed()
                            {
                                self.mark_config_changed();
                            }
                            ui.end_row();

                            ui.label(
                                RichText::new("PASSCODE")
                                    .strong()
                                    .color(theme_colors::TEXT_LOW)
                                    .size(12.0),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.config.user_password)
                                        .desired_width(field_width)
                                        .password(true),
                                )
                                .changed()
                            {
                                self.mark_config_changed();
                            }
                            ui.end_row();
                        });
                });
            }
        });
    }

    pub(crate) fn render_features(&mut self, ui: &mut egui::Ui) {
        self.render_header(
            ui,
            "Resource Assets",
            "Deploy high-performance environment clusters.",
        );

        crystal_card(ui, |ui: &mut egui::Ui| {
            ui.label(
                RichText::new("CLUSTER CORE")
                    .small()
                    .strong()
                    .color(theme_colors::TEXT_LOW)
                    .extra_letter_spacing(1.5),
            );
            ui.add_space(16.0);
            if ui
                .checkbox(
                    &mut self.config.docker,
                    RichText::new("Docker Orchestration Matrix").size(17.0),
                )
                .changed()
            {
                self.mark_config_changed();
            }
            ui.add_space(20.0);

            if ui
                .checkbox(
                    &mut self.config.lemp,
                    RichText::new("LEMP High-Performance Stack").size(17.0),
                )
                .changed()
            {
                self.mark_config_changed();
            }
            if self.config.lemp {
                ui.add_space(12.0);
                ui.indent("lemp_indent", |ui: &mut egui::Ui| {
                    if ui
                        .checkbox(&mut self.config.wordpress, "WordPress Platinum Engine")
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    if ui
                        .checkbox(&mut self.config.certbot, "Quantum SSL Hardening")
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                });
            }
        });

        ui.add_space(40.0);

        crystal_card(ui, |ui: &mut egui::Ui| {
            ui.label(
                RichText::new("DEVELOPER ENVIRONMENT")
                    .small()
                    .strong()
                    .color(theme_colors::TEXT_LOW)
                    .extra_letter_spacing(1.5),
            );
            ui.add_space(16.0);
            if ui
                .checkbox(
                    &mut self.config.devtools,
                    RichText::new("Integrated IDE Subsystem")
                        .size(17.0)
                        .strong(),
                )
                .changed()
            {
                self.mark_config_changed();
            }

            if self.config.devtools {
                ui.add_space(20.0);
                ui.indent("dev_indent", |ui: &mut egui::Ui| {
                    ui.columns(2, |cols| {
                        cols[0].vertical(|ui| {
                            ui.label(
                                RichText::new("Core Tools & AI")
                                    .strong()
                                    .color(theme_colors::TEXT_MED),
                            );
                            ui.add_space(8.0);
                            let mut c = false;
                            c |= ui
                                .checkbox(&mut self.config.install_neovim, "Neovim Terminal IDE")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_micro, "Micro Text Editor")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_zsh, "Zsh Shell")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_fish, "Fish Shell")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_starship, "Starship Prompt")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_tmux, "tmux Multiplexer")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_nodejs, "Node.js 24 LTS")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_github_cli, "GitHub CLI (gh)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_claude_code, "Claude Code")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_gemini, "Gemini AI CLI")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_kiro, "Kiro Tool")
                                .changed();

                            ui.add_space(16.0);
                            ui.label(
                                RichText::new("File & Process Mgmt")
                                    .strong()
                                    .color(theme_colors::TEXT_MED),
                            );
                            ui.add_space(8.0);
                            c |= ui
                                .checkbox(&mut self.config.install_btop, "btop (System Monitor)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_htop, "htop (Process Viewer)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_ranger, "ranger (File Manager)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_duf, "duf (Disk Usage)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_ncdu, "ncdu (Disk Analyzer)")
                                .changed();

                            if c {
                                self.mark_config_changed();
                            }
                        });

                        cols[1].vertical(|ui| {
                            ui.label(
                                RichText::new("Modern Utilities")
                                    .strong()
                                    .color(theme_colors::TEXT_MED),
                            );
                            ui.add_space(8.0);
                            let mut c = false;
                            c |= ui
                                .checkbox(&mut self.config.install_ripgrep, "ripgrep (rg)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_fd, "fd-find (fd)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_fzf, "fzf (Fuzzy Finder)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_zoxide, "zoxide (Smarter cd)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_direnv, "direnv (Env Switcher)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_lnav, "lnav (Log Navigator)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_tldr, "tldr (Simplified Man)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_lazygit, "lazygit (Git TUI)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_uv, "uv (Fast Python PM)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_jq, "jq (JSON Processor)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_gping, "gping (Visual Ping)")
                                .changed();
                            c |= ui
                                .checkbox(&mut self.config.install_nmap, "nmap (Network Audit)")
                                .changed();
                            c |= ui
                                .checkbox(
                                    &mut self.config.install_autossh,
                                    "autossh (SSH Persistence)",
                                )
                                .changed();

                            if c {
                                self.mark_config_changed();
                            }
                        });
                    });
                });
            }
        });
    }

    pub(crate) fn render_security(&mut self, ui: &mut egui::Ui) {
        self.render_header(
            ui,
            "Shield Perimeter",
            "Activate multi-layered system hardening protocols.",
        );

        ui.columns(2, |cols| {
            crystal_card(&mut cols[0], |ui: &mut egui::Ui| {
                ui.label(
                    RichText::new("THREAT MITIGATION")
                        .small()
                        .strong()
                        .color(theme_colors::TEXT_LOW)
                        .extra_letter_spacing(1.0),
                );
                ui.add_space(16.0);
                let mut c = false;
                c |= ui
                    .checkbox(&mut self.config.fail2ban, "Fail2ban Shield")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.apparmor, "AppArmor MAC")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.rootkit_detection, "Malware Scan")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.suricata, "Network IDS")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.disable_ipv6, "Disable IPv6 Protocol")
                    .changed();
                if c {
                    self.mark_config_changed();
                }
            });

            crystal_card(&mut cols[1], |ui: &mut egui::Ui| {
                ui.label(
                    RichText::new("ACCESS CONTROL")
                        .small()
                        .strong()
                        .color(theme_colors::TEXT_LOW)
                        .extra_letter_spacing(1.0),
                );
                ui.add_space(16.0);
                let mut c = false;
                c |= ui
                    .checkbox(&mut self.config.ssh_2fa_totp, "2FA: Authenticator")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.ssh_2fa_fido2, "2FA: FIDO2 Keys")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.ssh_2fa_duo, "2FA: Duo Push")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.usb_restrictions, "USB Hardware Lock")
                    .changed();
                c |= ui
                    .checkbox(&mut self.config.backups, "Automated System Backups")
                    .changed();
                if c {
                    self.mark_config_changed();
                }
            });
        });

        ui.add_space(40.0);

        crystal_card(ui, |ui: &mut egui::Ui| {
            ui.label(
                RichText::new("NUCLEUS HARDENING")
                    .small()
                    .strong()
                    .color(theme_colors::TEXT_LOW)
                    .extra_letter_spacing(1.5),
            );
            ui.add_space(20.0);
            ui.horizontal(|ui: &mut egui::Ui| {
                let mut c = false;
                c |= ui
                    .checkbox(&mut self.config.system_hardening, "Kernel Tuning")
                    .changed();
                ui.add_space(32.0);
                c |= ui
                    .checkbox(&mut self.config.file_integrity, "AIDE Sync")
                    .changed();
                ui.add_space(32.0);
                c |= ui
                    .checkbox(&mut self.config.audit_logging, "Auditd Stream")
                    .changed();
                if c {
                    self.mark_config_changed();
                }
            });
        });
    }

    pub(crate) fn render_maintenance(&mut self, ui: &mut egui::Ui) {
        self.render_header(
            ui,
            "System Nucleus",
            "Optimize resource allocation and cycle management.",
        );

        crystal_card(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.vertical(|ui: &mut egui::Ui| {
                    if ui
                        .checkbox(
                            &mut self.config.swap,
                            RichText::new("Intelligent Swap").size(17.0),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.label(
                        RichText::new("Automated RAM paging")
                            .small()
                            .color(theme_colors::TEXT_LOW),
                    );
                });
                ui.add_space(80.0);
                ui.vertical(|ui: &mut egui::Ui| {
                    if ui
                        .checkbox(
                            &mut self.config.cron_jobs,
                            RichText::new("Automated Ops").size(17.0),
                        )
                        .changed()
                    {
                        self.mark_config_changed();
                    }
                    ui.label(
                        RichText::new("Security patch cycles")
                            .small()
                            .color(theme_colors::TEXT_LOW),
                    );
                });
            });
        });

        ui.add_space(40.0);

        crystal_card(ui, |ui: &mut egui::Ui| {
            if ui
                .checkbox(
                    &mut self.config.periodic_reboot,
                    RichText::new("Scheduled Refresh Cycles")
                        .size(17.0)
                        .strong(),
                )
                .changed()
            {
                self.mark_config_changed();
            }

            if self.config.periodic_reboot {
                ui.add_space(24.0);
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label(
                        RichText::new("PROTOCOL FREQUENCY")
                            .color(theme_colors::TEXT_LOW)
                            .strong()
                            .size(12.0)
                            .extra_letter_spacing(1.0),
                    );
                    let combo_response = egui::ComboBox::from_id_salt("reboot_hour")
                        .selected_text(format_reboot_schedule(&self.config.reboot_hour))
                        .width(280.0)
                        .show_ui(ui, |ui| {
                            let mut changed = false;
                            changed |= ui
                                .selectable_value(
                                    &mut self.config.reboot_hour,
                                    "1".into(),
                                    "01:00 Standard",
                                )
                                .changed();
                            changed |= ui
                                .selectable_value(
                                    &mut self.config.reboot_hour,
                                    "3".into(),
                                    "03:00 Standard",
                                )
                                .changed();
                            changed |= ui
                                .selectable_value(
                                    &mut self.config.reboot_hour,
                                    "5".into(),
                                    "05:00 Standard",
                                )
                                .changed();
                            changed |= ui
                                .selectable_value(
                                    &mut self.config.reboot_hour,
                                    "*/6".into(),
                                    "Interval: 6 Hours",
                                )
                                .changed();
                            changed |= ui
                                .selectable_value(
                                    &mut self.config.reboot_hour,
                                    "*/12".into(),
                                    "Interval: 12 Hours",
                                )
                                .changed();
                            changed
                        });
                    if combo_response.inner.unwrap_or(false) {
                        self.mark_config_changed();
                    }
                });
            }
        });
    }

    pub(crate) fn render_output(&mut self, ui: &mut egui::Ui) {
        if let Some(msg) = self.result_message.clone() {
            ui.label(
                RichText::new(format!("❯ SUCCESS: {}", msg))
                    .color(theme_colors::SUCCESS)
                    .strong()
                    .size(22.0),
            );
            ui.add_space(24.0);
        }
        if let Some(msg) = self.error_message.clone() {
            ui.label(
                RichText::new(format!("❯ INTERRUPT: {}", msg))
                    .color(theme_colors::ERROR)
                    .strong()
                    .size(22.0),
            );
            ui.add_space(24.0);
        }

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("System Stream")
                    .size(26.0)
                    .strong()
                    .color(theme_colors::TEXT_BRIGHT),
            );
            if self.provisioning {
                ui.spinner();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("PURGE LOG").clicked() {
                    self.output_lines.clear();
                    self.result_message = None;
                    self.error_message = None;
                }
                if !self.output_lines.is_empty() && ui.button("COPY LOG").clicked() {
                    let full_log = self.output_lines.join("\n");
                    ui.ctx().copy_text(full_log);
                }
            });
        });

        ui.add_space(24.0);
        egui::Frame::NONE
            .fill(theme_colors::TERMINAL_BG)
            .corner_radius(CornerRadius::same(20))
            .inner_margin(28.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 20.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        if self.output_lines.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("SYSTEM READY FOR INITIALIZATION")
                                        .color(theme_colors::TEXT_LOW)
                                        .strong()
                                        .extra_letter_spacing(2.0),
                                );
                            });
                        } else {
                            // Virtualized rendering for performance with large output
                            let text_style = egui::TextStyle::Monospace;
                            let row_height = ui.text_style_height(&text_style);
                            let num_rows = self.output_lines.len();

                            ui.set_min_width(ui.available_width());

                            egui::ScrollArea::vertical().show_rows(
                                ui,
                                row_height,
                                num_rows,
                                |ui, row_range| {
                                    for row_idx in row_range {
                                        if let Some(line) = self.output_lines.get(row_idx) {
                                            let (color, bold) = ansible_line_style(line);
                                            let mut text = RichText::new(line)
                                                .font(egui::FontId::monospace(14.0))
                                                .color(color);
                                            if bold {
                                                text = text.strong();
                                            }
                                            ui.label(text);
                                        }
                                    }
                                },
                            );
                        }
                    });
            });
    }
}
