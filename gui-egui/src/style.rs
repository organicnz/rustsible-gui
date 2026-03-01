use eframe::egui::{self, Color32, Visuals, CornerRadius, Stroke, Margin};

pub mod macos_v26_colors {
    use super::Color32;

    // --- Platinum v26 "Onyx Crystal" Palette ---
    pub const APP_BG: Color32 = Color32::from_rgb(10, 10, 12);
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(30, 30, 32);
    
    // Glassmorphism - Refractive surfaces
    pub const GLASS_SURFACE: Color32 = Color32::from_rgba_premultiplied(40, 40, 45, 200);
    pub const GLASS_BORDER: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 15);
    
    // Accents - "Retina Blue"
    pub const ACCENT: Color32 = Color32::from_rgb(0, 122, 255);
    pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(100, 180, 255);
    pub const SELECTION_BG: Color32 = Color32::from_rgb(0, 88, 208);
    
    // Typography
    pub const TEXT_BRIGHT: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT_MED: Color32 = Color32::from_rgb(160, 160, 165);
    pub const TEXT_LOW: Color32 = Color32::from_rgb(100, 100, 105);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);

    // Status
    pub const SUCCESS: Color32 = Color32::from_rgb(48, 209, 88);
    pub const WARNING: Color32 = Color32::from_rgb(255, 159, 10);
    pub const ERROR: Color32 = Color32::from_rgb(255, 69, 58);
    pub const TERMINAL_BG: Color32 = Color32::from_rgb(5, 5, 5);
}

pub fn setup_macos_v26_style(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    
    visuals.window_fill = macos_v26_colors::APP_BG;
    visuals.panel_fill = macos_v26_colors::SIDEBAR_BG;
    
    let standard_radius = CornerRadius::same(12);
    
    visuals.widgets.noninteractive.bg_fill = macos_v26_colors::GLASS_SURFACE;
    visuals.widgets.noninteractive.corner_radius = standard_radius;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, macos_v26_colors::TEXT_LOW);

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 55);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, macos_v26_colors::GLASS_BORDER);
    
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 65);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    
    visuals.widgets.active.bg_fill = macos_v26_colors::ACCENT;
    visuals.widgets.active.corner_radius = CornerRadius::same(8);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    
    visuals.selection.bg_fill = macos_v26_colors::SELECTION_BG;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.window_margin = Margin::same(24);
    style.spacing.indent = 24.0;
    
    ctx.set_style(style);
}

/// Renders a high-fidelity macOS v26 platinum card
pub fn crystal_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::NONE
        .fill(macos_v26_colors::GLASS_SURFACE)
        .corner_radius(CornerRadius::same(16))
        .stroke(Stroke::new(1.0, macos_v26_colors::GLASS_BORDER))
        .inner_margin(24.0)
        .show(ui, add_contents);
}

pub fn ansible_line_style(line: &str) -> (Color32, bool) {
    let trimmed = line.trim();
    if trimmed.contains("FAILED") || trimmed.contains("fatal:") || trimmed.contains("ERROR") {
        return (macos_v26_colors::ERROR, true);
    }
    if trimmed.contains("changed:") {
        return (macos_v26_colors::WARNING, false);
    }
    if trimmed.contains("ok:") || trimmed.contains("SUCCESS") {
        return (macos_v26_colors::SUCCESS, false);
    }
    if trimmed.starts_with("TASK") || trimmed.starts_with("PLAY") {
        return (macos_v26_colors::TEXT_PRIMARY, true);
    }
    (macos_v26_colors::TEXT_MED, false)
}
