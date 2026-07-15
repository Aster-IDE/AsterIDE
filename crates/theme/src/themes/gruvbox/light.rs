use crate::metadata::ThemeMetadata;
use crate::themes::gruvbox::dark::GruvboxDark;
use egui::{
    Color32,
    Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "gruvbox_light",
    name: "Gruvbox Light",
    description: "Retro groove color scheme with light background",
    author: "Gruvbox (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: false,
    family: "Gruvbox",
};

pub struct GruvboxLight;

impl GruvboxLight {
    pub const BG_HARD: Color32 = Color32::from_rgb(251, 241, 199);
    pub const BG: Color32 = Color32::from_rgb(250, 240, 230);
    pub const BG_SOFT: Color32 = Color32::from_rgb(242, 229, 212);
    pub const BG1: Color32 = Color32::from_rgb(235, 219, 178);
    pub const BG2: Color32 = Color32::from_rgb(214, 204, 187);
    pub const BG3: Color32 = Color32::from_rgb(189, 174, 147);
    pub const BG4: Color32 = Color32::from_rgb(168, 153, 132);
    pub const FG: Color32 = Color32::from_rgb(60, 56, 54);
    pub const FG0: Color32 = Color32::from_rgb(80, 73, 69);
    pub const FG1: Color32 = Color32::from_rgb(102, 92, 84);
    pub const FG2: Color32 = Color32::from_rgb(124, 111, 100);
    pub const FG3: Color32 = Color32::from_rgb(146, 130, 109);
    pub const FG4: Color32 = Color32::from_rgb(168, 153, 132);
    pub const RED: Color32 = Color32::from_rgb(204, 36, 29);
    pub const GREEN: Color32 = Color32::from_rgb(152, 195, 121);
    pub const YELLOW: Color32 = Color32::from_rgb(215, 153, 33);
    pub const BLUE: Color32 = Color32::from_rgb(69, 133, 136);
    pub const PURPLE: Color32 = Color32::from_rgb(177, 98, 134);
    pub const AQUA: Color32 = Color32::from_rgb(104, 157, 106);
    pub const ORANGE: Color32 = Color32::from_rgb(214, 93, 14);
    pub const GRAY: Color32 = Color32::from_rgb(146, 130, 109);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::light();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BG;
        visuals.panel_fill = Self::BG;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::BG1);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::BG_SOFT;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::BG1;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::BG2;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::RED;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::ORANGE;

        visuals.selection.bg_fill = Self::BG2;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::BLUE);

        visuals.override_text_color = Some(Self::FG);
        visuals.hyperlink_color = Self::BLUE;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::FG);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::GREEN);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BG_HARD);

        ctx.set_visuals(visuals);
        GruvboxDark::apply_fonts(ctx);
    }
}
