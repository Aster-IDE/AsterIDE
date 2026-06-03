use crate::metadata::ThemeMetadata;
use egui::{Color32, FontId, TextStyle, Visuals};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "dracula_dark",
    name: "Dracula Dark",
    description: "A dark theme for Dracula, a color scheme for terminals and editors",
    author: "Dracula (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: true,
    family: "Dracula",
};

pub struct DraculaDark;

impl DraculaDark {
    pub const BACKGROUND: Color32 = Color32::from_rgb(28, 30, 40);
    pub const CURRENT_LINE: Color32 = Color32::from_rgb(40, 42, 54);
    pub const SELECTION: Color32 = Color32::from_rgb(68, 71, 90);
    pub const COMMENT: Color32 = Color32::from_rgb(98, 114, 164);
    pub const FOREGROUND: Color32 = Color32::from_rgb(229, 231, 235);
    pub const RED: Color32 = Color32::from_rgb(235, 100, 100);
    pub const ORANGE: Color32 = Color32::from_rgb(235, 160, 120);
    pub const YELLOW: Color32 = Color32::from_rgb(230, 220, 150);
    pub const GREEN: Color32 = Color32::from_rgb(120, 220, 140);
    pub const PURPLE: Color32 = Color32::from_rgb(180, 160, 240);
    pub const CYAN: Color32 = Color32::from_rgb(150, 200, 220);
    pub const PINK: Color32 = Color32::from_rgb(220, 140, 190);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::dark();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BACKGROUND;
        visuals.panel_fill = Self::BACKGROUND;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::CURRENT_LINE);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::CURRENT_LINE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::CURRENT_LINE;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::SELECTION;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::PURPLE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::PINK;

        visuals.selection.bg_fill = Self::SELECTION;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::PURPLE);

        visuals.override_text_color = Some(Self::FOREGROUND);
        visuals.hyperlink_color = Self::CYAN;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::FOREGROUND);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::CYAN);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BACKGROUND);

        ctx.set_visuals(visuals);
        Self::apply_fonts(ctx);
    }

    pub fn apply_fonts(ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(20.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(13.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(13.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(11.0, egui::FontFamily::Proportional),
        );
        ctx.set_global_style(style);
    }
}
