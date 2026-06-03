use crate::metadata::ThemeMetadata;
use egui::{Color32, FontId, TextStyle, Visuals};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "nord_dark",
    name: "Nord Dark",
    description: "An arctic, north-bluish color palette for editors and terminals",
    author: "Nord (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: true,
    family: "Nord",
};

pub struct NordDark;

impl NordDark {
    pub const NORD0: Color32 = Color32::from_rgb(46, 52, 64);
    pub const NORD1: Color32 = Color32::from_rgb(59, 66, 82);
    pub const NORD2: Color32 = Color32::from_rgb(67, 76, 94);
    pub const NORD3: Color32 = Color32::from_rgb(76, 86, 106);
    pub const NORD4: Color32 = Color32::from_rgb(94, 129, 172);
    pub const NORD5: Color32 = Color32::from_rgb(129, 161, 193);
    pub const NORD6: Color32 = Color32::from_rgb(143, 188, 187);
    pub const NORD7: Color32 = Color32::from_rgb(163, 190, 140);
    pub const NORD8: Color32 = Color32::from_rgb(191, 97, 106);
    pub const NORD9: Color32 = Color32::from_rgb(208, 135, 112);
    pub const NORD10: Color32 = Color32::from_rgb(235, 203, 139);
    pub const NORD11: Color32 = Color32::from_rgb(236, 239, 244);
    pub const NORD13: Color32 = Color32::from_rgb(216, 222, 233);
    pub const NORD14: Color32 = Color32::from_rgb(229, 233, 240);
    pub const NORD15: Color32 = Color32::from_rgb(236, 240, 241);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::dark();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::NORD0;
        visuals.panel_fill = Self::NORD0;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::NORD1);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::NORD1;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::NORD2;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::NORD3;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::NORD8;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::NORD9;

        visuals.selection.bg_fill = Self::NORD4;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::NORD6);

        visuals.override_text_color = Some(Self::NORD6);
        visuals.hyperlink_color = Self::NORD4;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::NORD6);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::NORD7);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::NORD0);

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
