use crate::metadata::ThemeMetadata;
use egui::{
  Color32,
  FontId,
  TextStyle,
  Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
  id: "gruvbox_dark",
  name: "Gruvbox Dark",
  description: "Retro groove color scheme with dark background",
  author: "Gruvbox (adapted for AsterIDE)",
  version: "1.0.0",
  is_dark: true,
  family: "Gruvbox",
};

pub struct GruvboxDark;

impl GruvboxDark {
  pub const BG_HARD: Color32 = Color32::from_rgb(29, 32, 33);
  pub const BG: Color32 = Color32::from_rgb(40, 40, 40);
  pub const BG_SOFT: Color32 = Color32::from_rgb(50, 48, 47);
  pub const BG1: Color32 = Color32::from_rgb(60, 56, 54);
  pub const BG2: Color32 = Color32::from_rgb(80, 73, 69);
  pub const BG3: Color32 = Color32::from_rgb(102, 92, 84);
  pub const BG4: Color32 = Color32::from_rgb(124, 111, 100);
  pub const FG: Color32 = Color32::from_rgb(235, 219, 178);
  pub const FG0: Color32 = Color32::from_rgb(213, 196, 161);
  pub const FG1: Color32 = Color32::from_rgb(189, 174, 147);
  pub const FG2: Color32 = Color32::from_rgb(168, 153, 132);
  pub const FG3: Color32 = Color32::from_rgb(146, 130, 109);
  pub const FG4: Color32 = Color32::from_rgb(124, 111, 100);
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
    let mut visuals = Visuals::dark();
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
