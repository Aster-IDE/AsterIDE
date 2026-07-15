use crate::metadata::ThemeMetadata;
use crate::themes::rose_pine::base::RosePine;
use egui::{
    Color32,
    Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
    id: "rose_pine_dawn",
    name: "Rose Pine Dawn",
    description: "All natural pine, faux fur and a bit of soho vibes for the classy minimalist (dawn variant)",
    author: "Rose Pine (adapted for AsterIDE)",
    version: "1.0.0",
    is_dark: false,
    family: "Rose Pine",
};

pub struct RosePineDawn;

impl RosePineDawn {
    pub const BASE: Color32 = Color32::from_rgb(250, 245, 238);
    pub const SURFACE: Color32 = Color32::from_rgb(255, 251, 245);
    pub const OVERLAY: Color32 = Color32::from_rgb(242, 237, 232);
    pub const MUTED: Color32 = Color32::from_rgb(152, 147, 165);
    pub const SUBTLE: Color32 = Color32::from_rgb(121, 112, 122);
    pub const TEXT: Color32 = Color32::from_rgb(87, 82, 91);
    pub const LOVE: Color32 = Color32::from_rgb(180, 99, 122);
    pub const GOLD: Color32 = Color32::from_rgb(234, 157, 52);
    pub const ROSE: Color32 = Color32::from_rgb(215, 130, 126);
    pub const PINE: Color32 = Color32::from_rgb(40, 105, 131);
    pub const FOAM: Color32 = Color32::from_rgb(86, 148, 159);
    pub const IRIS: Color32 = Color32::from_rgb(144, 122, 169);
    pub const HIGHLIGHT_LOW: Color32 = Color32::from_rgb(244, 239, 234);
    pub const HIGHLIGHT_MED: Color32 = Color32::from_rgb(223, 218, 213);
    pub const HIGHLIGHT_HIGH: Color32 = Color32::from_rgb(206, 202, 197);

    pub fn metadata() -> ThemeMetadata {
        METADATA
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::light();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::HIGHLIGHT_MED);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::OVERLAY;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::HIGHLIGHT_MED;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::ROSE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::LOVE;

        visuals.selection.bg_fill = Self::ROSE;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::LOVE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::IRIS;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::PINE);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        RosePine::apply_fonts(ctx);
    }
}
