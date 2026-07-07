use crate::metadata::ThemeMetadata;
use crate::themes::catppuccin::mocha::CatppuccinMocha;
use egui::{
  Color32,
  Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
  id: "catppuccin_latte",
  name: "Catppuccin Latte",
  description: "Soothing pastel light theme with light background",
  author: "Catppuccin (adapted for AsterIDE)",
  version: "1.0.0",
  is_dark: false,
  family: "Catppuccin",
};

pub struct CatppuccinLatte;

impl CatppuccinLatte {
  pub const ROSEWATER: Color32 = Color32::from_rgb(220, 138, 120);
  pub const FLAMINGO: Color32 = Color32::from_rgb(221, 120, 120);
  pub const PINK: Color32 = Color32::from_rgb(234, 118, 140);
  pub const MAUVE: Color32 = Color32::from_rgb(136, 57, 239);
  pub const RED: Color32 = Color32::from_rgb(210, 15, 57);
  pub const MAROON: Color32 = Color32::from_rgb(230, 69, 83);
  pub const PEACH: Color32 = Color32::from_rgb(254, 100, 11);
  pub const YELLOW: Color32 = Color32::from_rgb(223, 142, 29);
  pub const GREEN: Color32 = Color32::from_rgb(64, 160, 43);
  pub const TEAL: Color32 = Color32::from_rgb(23, 146, 153);
  pub const SKY: Color32 = Color32::from_rgb(4, 165, 229);
  pub const SAPPHIRE: Color32 = Color32::from_rgb(32, 74, 135);
  pub const BLUE: Color32 = Color32::from_rgb(30, 102, 245);
  pub const LAVENDER: Color32 = Color32::from_rgb(114, 135, 253);
  pub const TEXT: Color32 = Color32::from_rgb(76, 79, 105);
  pub const SUBTEXT1: Color32 = Color32::from_rgb(92, 95, 119);
  pub const SUBTEXT0: Color32 = Color32::from_rgb(108, 111, 133);
  pub const OVERLAY2: Color32 = Color32::from_rgb(124, 127, 147);
  pub const OVERLAY1: Color32 = Color32::from_rgb(140, 143, 161);
  pub const OVERLAY0: Color32 = Color32::from_rgb(156, 160, 176);
  pub const SURFACE2: Color32 = Color32::from_rgb(181, 189, 204);
  pub const SURFACE1: Color32 = Color32::from_rgb(205, 214, 230);
  pub const SURFACE0: Color32 = Color32::from_rgb(230, 239, 254);
  pub const BASE: Color32 = Color32::from_rgb(239, 241, 245);
  pub const MANTLE: Color32 = Color32::from_rgb(230, 233, 239);
  pub const CRUST: Color32 = Color32::from_rgb(220, 224, 232);

  pub fn metadata() -> ThemeMetadata {
    METADATA
  }

  pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
    let mut visuals = Visuals::light();
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
