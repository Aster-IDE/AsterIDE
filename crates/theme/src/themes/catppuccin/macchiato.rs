use crate::metadata::ThemeMetadata;
use crate::themes::catppuccin::mocha::CatppuccinMocha;
use egui::{
    Color32,
    Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "catppuccin_macchiato",
    name: "Catppuccin Macchiato",
    description: "Soothing pastel dark theme with macchiato background",
    author: "Catppuccin (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: true,
    family: "Catppuccin",
};

pub struct CatppuccinMacchiato;

impl CatppuccinMacchiato {
    pub const ROSEWATER: Color32 = Color32::from_rgb(244, 219, 214);
    pub const FLAMINGO: Color32 = Color32::from_rgb(240, 200, 198);
    pub const PINK: Color32 = Color32::from_rgb(245, 224, 220);
    pub const MAUVE: Color32 = Color32::from_rgb(198, 160, 236);
    pub const RED: Color32 = Color32::from_rgb(237, 135, 150);
    pub const MAROON: Color32 = Color32::from_rgb(238, 168, 175);
    pub const PEACH: Color32 = Color32::from_rgb(245, 169, 127);
    pub const YELLOW: Color32 = Color32::from_rgb(238, 214, 175);
    pub const GREEN: Color32 = Color32::from_rgb(166, 218, 149);
    pub const TEAL: Color32 = Color32::from_rgb(150, 205, 201);
    pub const SKY: Color32 = Color32::from_rgb(137, 220, 235);
    pub const SAPPHIRE: Color32 = Color32::from_rgb(138, 173, 244);
    pub const BLUE: Color32 = Color32::from_rgb(110, 147, 255);
    pub const LAVENDER: Color32 = Color32::from_rgb(183, 189, 248);
    pub const TEXT: Color32 = Color32::from_rgb(202, 213, 249);
    pub const SUBTEXT1: Color32 = Color32::from_rgb(184, 194, 232);
    pub const SUBTEXT0: Color32 = Color32::from_rgb(165, 175, 214);
    pub const OVERLAY2: Color32 = Color32::from_rgb(148, 156, 197);
    pub const OVERLAY1: Color32 = Color32::from_rgb(131, 139, 179);
    pub const OVERLAY0: Color32 = Color32::from_rgb(114, 122, 161);
    pub const SURFACE2: Color32 = Color32::from_rgb(88, 95, 138);
    pub const SURFACE1: Color32 = Color32::from_rgb(71, 78, 120);
    pub const SURFACE0: Color32 = Color32::from_rgb(49, 55, 89);
    pub const BASE: Color32 = Color32::from_rgb(36, 39, 59);
    pub const MANTLE: Color32 = Color32::from_rgb(30, 32, 48);
    pub const CRUST: Color32 = Color32::from_rgb(24, 26, 38);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::dark();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::MANTLE);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE0;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::SURFACE1;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::SURFACE2;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::MAUVE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::LAVENDER;

        visuals.selection.bg_fill = Self::SURFACE2;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::MAUVE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::LAVENDER;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::LAVENDER);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        CatppuccinMocha::apply_fonts(ctx);
    }
}
