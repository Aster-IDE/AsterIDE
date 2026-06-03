use crate::metadata::ThemeMetadata;
use crate::themes::catppuccin::mocha::CatppuccinMocha;
use egui::{Color32, Visuals};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "catppuccin_frappe",
    name: "Catppuccin Frappé",
    description: "Soothing pastel dark theme with frappe background",
    author: "Catppuccin (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: true,
    family: "Catppuccin",
};

pub struct CatppuccinFrappé;

impl CatppuccinFrappé {
    pub const ROSEWATER: Color32 = Color32::from_rgb(242, 213, 207);
    pub const FLAMINGO: Color32 = Color32::from_rgb(238, 190, 190);
    pub const PINK: Color32 = Color32::from_rgb(244, 219, 234);
    pub const MAUVE: Color32 = Color32::from_rgb(202, 158, 230);
    pub const RED: Color32 = Color32::from_rgb(231, 130, 132);
    pub const MAROON: Color32 = Color32::from_rgb(234, 153, 156);
    pub const PEACH: Color32 = Color32::from_rgb(239, 159, 118);
    pub const YELLOW: Color32 = Color32::from_rgb(238, 205, 136);
    pub const GREEN: Color32 = Color32::from_rgb(166, 209, 137);
    pub const TEAL: Color32 = Color32::from_rgb(143, 188, 187);
    pub const SKY: Color32 = Color32::from_rgb(133, 193, 220);
    pub const SAPPHIRE: Color32 = Color32::from_rgb(133, 193, 220);
    pub const BLUE: Color32 = Color32::from_rgb(103, 132, 235);
    pub const LAVENDER: Color32 = Color32::from_rgb(177, 169, 245);
    pub const TEXT: Color32 = Color32::from_rgb(198, 212, 236);
    pub const SUBTEXT1: Color32 = Color32::from_rgb(179, 191, 216);
    pub const SUBTEXT0: Color32 = Color32::from_rgb(161, 171, 196);
    pub const OVERLAY2: Color32 = Color32::from_rgb(147, 153, 178);
    pub const OVERLAY1: Color32 = Color32::from_rgb(129, 135, 161);
    pub const OVERLAY0: Color32 = Color32::from_rgb(111, 118, 143);
    pub const SURFACE2: Color32 = Color32::from_rgb(86, 91, 120);
    pub const SURFACE1: Color32 = Color32::from_rgb(69, 74, 102);
    pub const SURFACE0: Color32 = Color32::from_rgb(48, 52, 70);
    pub const BASE: Color32 = Color32::from_rgb(30, 32, 48);
    pub const MANTLE: Color32 = Color32::from_rgb(24, 26, 41);
    pub const CRUST: Color32 = Color32::from_rgb(17, 19, 32);

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
