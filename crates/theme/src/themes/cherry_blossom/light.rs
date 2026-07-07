use crate::metadata::ThemeMetadata;
use crate::themes::cherry_blossom::dark::CherryBlossomDark;
use egui::{
  Color32,
  Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
  id: "cherry_blossom_light",
  name: "Cherry Blossom Light",
  description: "A light theme with soft pink accents inspired by cherry blossoms, the default theme for AsterIDE.",
  author: "AsterIDE",
  version: "1.0.0",
  is_dark: false,
  family: "Cherry Blossom",
};

pub struct CherryBlossomLight;

impl CherryBlossomLight {
  pub const PINK_50: Color32 = Color32::from_rgb(255, 250, 252);
  pub const PINK_100: Color32 = Color32::from_rgb(255, 240, 248);
  pub const PINK_200: Color32 = Color32::from_rgb(255, 225, 240);
  pub const PINK_300: Color32 = Color32::from_rgb(255, 200, 225);
  pub const PINK_400: Color32 = Color32::from_rgb(255, 170, 205);
  pub const PINK_500: Color32 = Color32::from_rgb(255, 140, 185);
  pub const PINK_600: Color32 = Color32::from_rgb(235, 110, 160);
  pub const PINK_700: Color32 = Color32::from_rgb(210, 85, 135);
  pub const PINK_800: Color32 = Color32::from_rgb(180, 65, 110);
  pub const PINK_900: Color32 = Color32::from_rgb(145, 50, 85);

  pub const BG_LIGHTEST: Color32 = Color32::from_rgb(255, 252, 254);
  pub const BG_LIGHT: Color32 = Color32::from_rgb(255, 248, 252);
  pub const BG_MID: Color32 = Color32::from_rgb(255, 240, 248);
  pub const BG_DARK: Color32 = Color32::from_rgb(245, 230, 240);
  pub const BG_DARKER: Color32 = Color32::from_rgb(235, 220, 230);

  pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(80, 40, 60);
  pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(130, 80, 105);
  pub const TEXT_MUTED: Color32 = Color32::from_rgb(170, 120, 145);

  pub const ACCENT_PINK: Color32 = Color32::from_rgb(220, 80, 140);
  pub const ACCENT_HOT: Color32 = Color32::from_rgb(255, 90, 150);
  pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(255, 180, 210);

  pub fn metadata() -> ThemeMetadata {
    METADATA
  }

  pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
    let mut visuals = Visuals::light();
    let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

    visuals.window_fill = Self::BG_LIGHT;
    visuals.panel_fill = Self::BG_LIGHT;
    visuals.window_stroke = egui::Stroke::new(1.0, Self::BG_DARK);
    visuals.window_corner_radius = radius;
    visuals.menu_corner_radius = radius;

    visuals.widgets.noninteractive.corner_radius = radius;
    visuals.widgets.noninteractive.bg_fill = Self::BG_MID;
    visuals.widgets.inactive.corner_radius = radius;
    visuals.widgets.inactive.bg_fill = Self::BG_DARK;
    visuals.widgets.hovered.corner_radius = radius;
    visuals.widgets.hovered.bg_fill = Self::BG_DARKER;
    visuals.widgets.active.corner_radius = radius;
    visuals.widgets.active.bg_fill = Self::PINK_400;
    visuals.widgets.open.corner_radius = radius;
    visuals.widgets.open.bg_fill = Self::PINK_500;

    visuals.selection.bg_fill = Self::PINK_400;
    visuals.selection.stroke = egui::Stroke::new(1.0, Self::PINK_700);

    visuals.override_text_color = Some(Self::TEXT_PRIMARY);
    visuals.hyperlink_color = Self::ACCENT_PINK;

    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT_PRIMARY);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::PINK_700);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::PINK_50);

    ctx.set_visuals(visuals);
    CherryBlossomDark::apply_fonts(ctx);
  }
}
