use crate::metadata::ThemeMetadata;
use egui::{
  Color32,
  FontId,
  TextStyle,
  Visuals,
};

pub const METADATA: ThemeMetadata = ThemeMetadata {
  id: "catppuccin_mocha",
  name: "Catppuccin Mocha",
  description: "Soothing pastel dark theme with dark background",
  author: "Catppuccin (adapted for AsterIDE)",
  version: "1.0.0",
  is_dark: true,
  family: "Catppuccin",
};

pub struct CatppuccinMocha;

impl CatppuccinMocha {
  pub const ROSEWATER: Color32 = Color32::from_rgb(245, 224, 220);
  pub const FLAMINGO: Color32 = Color32::from_rgb(242, 205, 205);
  pub const PINK: Color32 = Color32::from_rgb(245, 194, 231);
  pub const MAUVE: Color32 = Color32::from_rgb(203, 166, 247);
  pub const RED: Color32 = Color32::from_rgb(243, 139, 168);
  pub const MAROON: Color32 = Color32::from_rgb(235, 160, 172);
  pub const PEACH: Color32 = Color32::from_rgb(250, 179, 135);
  pub const YELLOW: Color32 = Color32::from_rgb(249, 226, 175);
  pub const GREEN: Color32 = Color32::from_rgb(166, 227, 161);
  pub const TEAL: Color32 = Color32::from_rgb(148, 226, 213);
  pub const SKY: Color32 = Color32::from_rgb(137, 220, 235);
  pub const SAPPHIRE: Color32 = Color32::from_rgb(138, 173, 244);
  pub const BLUE: Color32 = Color32::from_rgb(114, 135, 253);
  pub const LAVENDER: Color32 = Color32::from_rgb(180, 190, 254);
  pub const TEXT: Color32 = Color32::from_rgb(198, 208, 245);
  pub const SUBTEXT1: Color32 = Color32::from_rgb(181, 191, 226);
  pub const SUBTEXT0: Color32 = Color32::from_rgb(165, 173, 206);
  pub const OVERLAY2: Color32 = Color32::from_rgb(148, 156, 187);
  pub const OVERLAY1: Color32 = Color32::from_rgb(131, 139, 167);
  pub const OVERLAY0: Color32 = Color32::from_rgb(114, 120, 147);
  pub const SURFACE2: Color32 = Color32::from_rgb(88, 91, 112);
  pub const SURFACE1: Color32 = Color32::from_rgb(69, 71, 90);
  pub const SURFACE0: Color32 = Color32::from_rgb(49, 50, 68);
  pub const BASE: Color32 = Color32::from_rgb(30, 30, 46);
  pub const MANTLE: Color32 = Color32::from_rgb(24, 24, 37);
  pub const CRUST: Color32 = Color32::from_rgb(17, 17, 27);

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
