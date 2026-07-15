use crate::metadata::ThemeMetadata;
use crate::themes::rose_pine::base::RosePine;
use egui::{
    Color32,
    Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "rose_pine_moon",
    name: "Rose Pine Moon",
    description: "All natural pine, faux fur and a bit of soho vibes for the classy minimalist (dark variant)",
    author: "Rose Pine (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: true,
    family: "Rose Pine",
};

pub struct RosePineMoon;

impl RosePineMoon {
    pub const BASE: Color32 = Color32::from_rgb(35, 33, 54);
    pub const SURFACE: Color32 = Color32::from_rgb(42, 40, 62);
    pub const OVERLAY: Color32 = Color32::from_rgb(57, 53, 82);
    pub const MUTED: Color32 = Color32::from_rgb(110, 106, 134);
    pub const SUBTLE: Color32 = Color32::from_rgb(144, 140, 170);
    pub const TEXT: Color32 = Color32::from_rgb(224, 222, 244);
    pub const LOVE: Color32 = Color32::from_rgb(235, 111, 146);
    pub const GOLD: Color32 = Color32::from_rgb(246, 193, 119);
    pub const ROSE: Color32 = Color32::from_rgb(235, 188, 186);
    pub const PINE: Color32 = Color32::from_rgb(62, 143, 176);
    pub const FOAM: Color32 = Color32::from_rgb(156, 207, 216);
    pub const IRIS: Color32 = Color32::from_rgb(196, 167, 231);
    pub const HIGHLIGHT_LOW: Color32 = Color32::from_rgb(42, 40, 62);
    pub const HIGHLIGHT_MED: Color32 = Color32::from_rgb(68, 65, 90);
    pub const HIGHLIGHT_HIGH: Color32 = Color32::from_rgb(82, 79, 103);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::dark();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::OVERLAY);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::OVERLAY;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::HIGHLIGHT_MED;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::LOVE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::IRIS;

        visuals.selection.bg_fill = Self::LOVE;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::ROSE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::FOAM;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::ROSE);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        RosePine::apply_fonts(ctx);
    }
}
