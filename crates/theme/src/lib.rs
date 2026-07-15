use egui::Color32;
use serde::{
    Deserialize,
    Serialize,
};
use std::sync::RwLock;

pub mod metadata;
pub mod themes;

pub use metadata::ThemeMetadata;
pub use themes::{
    CatppuccinFrappé,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    CherryBlossomDark,
    CherryBlossomLight,
    DraculaDark,
    GruvboxDark,
    GruvboxLight,
    NordDark,
    RosePine,
    RosePineDawn,
    RosePineMoon,
};

static CURRENT_THEME_COLORS: RwLock<ThemeColors> = RwLock::new(ThemeColors::cherry_blossom_dark());

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub bg_darkest: Color32,
    pub bg_dark: Color32,
    pub bg_mid: Color32,
    pub bg_light: Color32,
    pub bg_lighter: Color32,
    pub border: Color32,

    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,

    pub accent_primary: Color32,
    pub accent_hot: Color32,
    pub accent_light: Color32,
}

impl ThemeColors {
    pub const fn cherry_blossom_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 20, 28),
            bg_dark: Color32::from_rgb(45, 28, 38),
            bg_mid: Color32::from_rgb(55, 35, 45),
            bg_light: Color32::from_rgb(70, 45, 58),
            bg_lighter: Color32::from_rgb(85, 55, 70),
            border: Color32::from_rgb(85, 55, 70),
            text_primary: Color32::from_rgb(255, 235, 245),
            text_secondary: Color32::from_rgb(200, 160, 180),
            text_muted: Color32::from_rgb(150, 110, 130),
            accent_primary: Color32::from_rgb(255, 130, 180),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 200, 220),
        }
    }

    pub const fn cherry_blossom_light() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(245, 220, 230),
            bg_dark: Color32::from_rgb(255, 248, 252),
            bg_mid: Color32::from_rgb(255, 240, 248),
            bg_light: Color32::from_rgb(245, 230, 240),
            bg_lighter: Color32::from_rgb(235, 220, 230),
            border: Color32::from_rgb(235, 220, 230),
            text_primary: Color32::from_rgb(80, 40, 60),
            text_secondary: Color32::from_rgb(130, 80, 105),
            text_muted: Color32::from_rgb(170, 120, 145),
            accent_primary: Color32::from_rgb(220, 80, 140),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 180, 210),
        }
    }

    pub const fn rose_pine() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(31, 29, 46),
            bg_dark: Color32::from_rgb(38, 35, 58),
            bg_mid: Color32::from_rgb(33, 32, 46),
            bg_light: Color32::from_rgb(49, 46, 73),
            bg_lighter: Color32::from_rgb(64, 61, 82),
            border: Color32::from_rgb(82, 79, 103),
            text_primary: Color32::from_rgb(224, 222, 244),
            text_secondary: Color32::from_rgb(144, 140, 170),
            text_muted: Color32::from_rgb(110, 106, 134),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }

    pub const fn rose_pine_moon() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 33, 54),
            bg_dark: Color32::from_rgb(42, 39, 63),
            bg_mid: Color32::from_rgb(57, 53, 82),
            bg_light: Color32::from_rgb(42, 40, 62),
            bg_lighter: Color32::from_rgb(68, 65, 90),
            border: Color32::from_rgb(86, 82, 110),
            text_primary: Color32::from_rgb(224, 222, 244),
            text_secondary: Color32::from_rgb(144, 140, 170),
            text_muted: Color32::from_rgb(110, 106, 134),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }

    pub const fn rose_pine_dawn() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(250, 244, 237),
            bg_dark: Color32::from_rgb(255, 250, 243),
            bg_mid: Color32::from_rgb(242, 233, 222),
            bg_light: Color32::from_rgb(223, 218, 211),
            bg_lighter: Color32::from_rgb(206, 202, 195),
            border: Color32::from_rgb(189, 185, 177),
            text_primary: Color32::from_rgb(87, 82, 91),
            text_secondary: Color32::from_rgb(121, 112, 122),
            text_muted: Color32::from_rgb(152, 147, 165),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }

    pub const fn dracula_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(28, 30, 40),
            bg_dark: Color32::from_rgb(40, 42, 54),
            bg_mid: Color32::from_rgb(68, 71, 90),
            bg_light: Color32::from_rgb(98, 114, 164),
            bg_lighter: Color32::from_rgb(150, 200, 220),
            border: Color32::from_rgb(68, 71, 90),
            text_primary: Color32::from_rgb(229, 231, 235),
            text_secondary: Color32::from_rgb(180, 160, 240),
            text_muted: Color32::from_rgb(98, 114, 164),
            accent_primary: Color32::from_rgb(180, 160, 240),
            accent_hot: Color32::from_rgb(235, 100, 100),
            accent_light: Color32::from_rgb(120, 220, 140),
        }
    }

    pub const fn nord_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(46, 52, 64),
            bg_dark: Color32::from_rgb(59, 66, 82),
            bg_mid: Color32::from_rgb(67, 76, 94),
            bg_light: Color32::from_rgb(76, 86, 106),
            bg_lighter: Color32::from_rgb(94, 129, 172),
            border: Color32::from_rgb(59, 66, 82),
            text_primary: Color32::from_rgb(216, 222, 233),
            text_secondary: Color32::from_rgb(143, 188, 187),
            text_muted: Color32::from_rgb(76, 86, 106),
            accent_primary: Color32::from_rgb(136, 192, 208),
            accent_hot: Color32::from_rgb(191, 97, 106),
            accent_light: Color32::from_rgb(163, 190, 140),
        }
    }

    pub const fn gruvbox_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(29, 32, 33),
            bg_dark: Color32::from_rgb(40, 40, 40),
            bg_mid: Color32::from_rgb(50, 48, 47),
            bg_light: Color32::from_rgb(60, 56, 54),
            bg_lighter: Color32::from_rgb(80, 73, 69),
            border: Color32::from_rgb(60, 56, 54),
            text_primary: Color32::from_rgb(235, 219, 178),
            text_secondary: Color32::from_rgb(168, 153, 132),
            text_muted: Color32::from_rgb(102, 92, 84),
            accent_primary: Color32::from_rgb(215, 153, 33),
            accent_hot: Color32::from_rgb(204, 36, 29),
            accent_light: Color32::from_rgb(152, 195, 121),
        }
    }

    pub const fn gruvbox_light() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(251, 241, 199),
            bg_dark: Color32::from_rgb(250, 240, 230),
            bg_mid: Color32::from_rgb(242, 229, 212),
            bg_light: Color32::from_rgb(235, 219, 178),
            bg_lighter: Color32::from_rgb(214, 204, 187),
            border: Color32::from_rgb(235, 219, 178),
            text_primary: Color32::from_rgb(60, 56, 54),
            text_secondary: Color32::from_rgb(124, 111, 100),
            text_muted: Color32::from_rgb(168, 153, 132),
            accent_primary: Color32::from_rgb(215, 153, 33),
            accent_hot: Color32::from_rgb(204, 36, 29),
            accent_light: Color32::from_rgb(152, 195, 121),
        }
    }

    pub const fn catppuccin_frappe() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(17, 19, 32),
            bg_dark: Color32::from_rgb(24, 26, 41),
            bg_mid: Color32::from_rgb(30, 32, 48),
            bg_light: Color32::from_rgb(48, 52, 70),
            bg_lighter: Color32::from_rgb(69, 74, 102),
            border: Color32::from_rgb(48, 52, 70),
            text_primary: Color32::from_rgb(198, 212, 236),
            text_secondary: Color32::from_rgb(147, 153, 178),
            text_muted: Color32::from_rgb(111, 118, 143),
            accent_primary: Color32::from_rgb(202, 158, 230),
            accent_hot: Color32::from_rgb(231, 130, 132),
            accent_light: Color32::from_rgb(177, 169, 245),
        }
    }

    pub const fn catppuccin_latte() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(220, 224, 232),
            bg_dark: Color32::from_rgb(230, 233, 239),
            bg_mid: Color32::from_rgb(239, 241, 245),
            bg_light: Color32::from_rgb(230, 239, 254),
            bg_lighter: Color32::from_rgb(205, 214, 230),
            border: Color32::from_rgb(230, 239, 254),
            text_primary: Color32::from_rgb(76, 79, 105),
            text_secondary: Color32::from_rgb(124, 127, 147),
            text_muted: Color32::from_rgb(156, 160, 176),
            accent_primary: Color32::from_rgb(136, 57, 239),
            accent_hot: Color32::from_rgb(210, 15, 57),
            accent_light: Color32::from_rgb(114, 135, 253),
        }
    }

    pub const fn catppuccin_macchiato() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(24, 26, 38),
            bg_dark: Color32::from_rgb(30, 32, 48),
            bg_mid: Color32::from_rgb(36, 39, 59),
            bg_light: Color32::from_rgb(49, 55, 89),
            bg_lighter: Color32::from_rgb(71, 78, 120),
            border: Color32::from_rgb(49, 55, 89),
            text_primary: Color32::from_rgb(202, 213, 249),
            text_secondary: Color32::from_rgb(148, 156, 197),
            text_muted: Color32::from_rgb(114, 122, 161),
            accent_primary: Color32::from_rgb(198, 160, 236),
            accent_hot: Color32::from_rgb(237, 135, 150),
            accent_light: Color32::from_rgb(183, 189, 248),
        }
    }

    pub const fn catppuccin_mocha() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(17, 17, 27),
            bg_dark: Color32::from_rgb(24, 24, 37),
            bg_mid: Color32::from_rgb(30, 30, 46),
            bg_light: Color32::from_rgb(49, 50, 68),
            bg_lighter: Color32::from_rgb(69, 71, 90),
            border: Color32::from_rgb(49, 50, 68),
            text_primary: Color32::from_rgb(198, 208, 245),
            text_secondary: Color32::from_rgb(148, 156, 187),
            text_muted: Color32::from_rgb(114, 120, 147),
            accent_primary: Color32::from_rgb(203, 166, 247),
            accent_hot: Color32::from_rgb(243, 139, 168),
            accent_light: Color32::from_rgb(180, 190, 254),
        }
    }

    pub fn for_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::CherryBlossomDark => Self::cherry_blossom_dark(),
            ThemeVariant::CherryBlossomLight => Self::cherry_blossom_light(),
            ThemeVariant::RosePine => Self::rose_pine(),
            ThemeVariant::RosePineMoon => Self::rose_pine_moon(),
            ThemeVariant::RosePineDawn => Self::rose_pine_dawn(),
            ThemeVariant::DraculaDark => Self::dracula_dark(),
            ThemeVariant::NordDark => Self::nord_dark(),
            ThemeVariant::GruvboxDark => Self::gruvbox_dark(),
            ThemeVariant::GruvboxLight => Self::gruvbox_light(),
            ThemeVariant::CatppuccinFrappé => Self::catppuccin_frappe(),
            ThemeVariant::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeVariant::CatppuccinMacchiato => Self::catppuccin_macchiato(),
            ThemeVariant::CatppuccinMocha => Self::catppuccin_mocha(),
        }
    }
}

pub fn current_theme_colors() -> ThemeColors {
    CURRENT_THEME_COLORS
        .read()
        .map(|c| *c)
        .unwrap_or_else(|_| ThemeColors::cherry_blossom_dark())
}

pub fn set_current_theme_colors(colors: ThemeColors) {
    if let Ok(mut guard) = CURRENT_THEME_COLORS.write() {
        *guard = colors;
    }
}

pub fn update_current_theme(variant: ThemeVariant) {
    set_current_theme_colors(ThemeColors::for_variant(variant));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeVariant {
    CherryBlossomDark,
    CherryBlossomLight,
    RosePine,
    RosePineMoon,
    RosePineDawn,
    DraculaDark,
    NordDark,
    GruvboxDark,
    GruvboxLight,
    CatppuccinFrappé,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::CherryBlossomDark => "Dark",
            ThemeVariant::CherryBlossomLight => "Light",
            ThemeVariant::RosePine => "Rose Pine",
            ThemeVariant::RosePineMoon => "Rose Pine Moon",
            ThemeVariant::RosePineDawn => "Rose Pine Dawn",
            ThemeVariant::DraculaDark => "Dracula Dark",
            ThemeVariant::NordDark => "Nord Dark",
            ThemeVariant::GruvboxDark => "Gruvbox Dark",
            ThemeVariant::GruvboxLight => "Gruvbox Light",
            ThemeVariant::CatppuccinFrappé => "Catppuccin Frappé",
            ThemeVariant::CatppuccinLatte => "Catppuccin Latte",
            ThemeVariant::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
        }
    }

    pub fn apply(&self, ctx: &egui::Context, corner_roundness: f32) {
        match self {
            ThemeVariant::CherryBlossomDark => {
                themes::CherryBlossomDark::apply(ctx, corner_roundness)
            }
            ThemeVariant::CherryBlossomLight => {
                themes::CherryBlossomLight::apply(ctx, corner_roundness)
            }
            ThemeVariant::RosePine => themes::RosePine::apply(ctx, corner_roundness),
            ThemeVariant::RosePineMoon => themes::RosePineMoon::apply(ctx, corner_roundness),
            ThemeVariant::RosePineDawn => themes::RosePineDawn::apply(ctx, corner_roundness),
            ThemeVariant::DraculaDark => themes::DraculaDark::apply(ctx, corner_roundness),
            ThemeVariant::NordDark => themes::NordDark::apply(ctx, corner_roundness),
            ThemeVariant::GruvboxDark => themes::GruvboxDark::apply(ctx, corner_roundness),
            ThemeVariant::GruvboxLight => themes::GruvboxLight::apply(ctx, corner_roundness),
            ThemeVariant::CatppuccinFrappé => {
                themes::CatppuccinFrappé::apply(ctx, corner_roundness)
            }
            ThemeVariant::CatppuccinLatte => themes::CatppuccinLatte::apply(ctx, corner_roundness),
            ThemeVariant::CatppuccinMacchiato => {
                themes::CatppuccinMacchiato::apply(ctx, corner_roundness)
            }
            ThemeVariant::CatppuccinMocha => themes::CatppuccinMocha::apply(ctx, corner_roundness),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeFamily {
    CherryBlossom,
    RosePine,
    Dracula,
    Nord,
    Gruvbox,
    Catppuccin,
}

impl ThemeFamily {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeFamily::CherryBlossom => "Cherry Blossom",
            ThemeFamily::RosePine => "Rose Pine",
            ThemeFamily::Dracula => "Dracula",
            ThemeFamily::Nord => "Nord",
            ThemeFamily::Gruvbox => "Gruvbox",
            ThemeFamily::Catppuccin => "Catppuccin",
        }
    }

    pub fn variants(&self) -> &'static [ThemeVariant] {
        match self {
            ThemeFamily::CherryBlossom => &[
                ThemeVariant::CherryBlossomDark,
                ThemeVariant::CherryBlossomLight,
            ],
            ThemeFamily::RosePine => &[
                ThemeVariant::RosePine,
                ThemeVariant::RosePineMoon,
                ThemeVariant::RosePineDawn,
            ],
            ThemeFamily::Dracula => &[ThemeVariant::DraculaDark],
            ThemeFamily::Nord => &[ThemeVariant::NordDark],
            ThemeFamily::Gruvbox => &[ThemeVariant::GruvboxDark, ThemeVariant::GruvboxLight],
            ThemeFamily::Catppuccin => &[
                ThemeVariant::CatppuccinFrappé,
                ThemeVariant::CatppuccinLatte,
                ThemeVariant::CatppuccinMacchiato,
                ThemeVariant::CatppuccinMocha,
            ],
        }
    }

    pub fn default_variant(&self) -> ThemeVariant {
        match self {
            ThemeFamily::CherryBlossom => ThemeVariant::CherryBlossomDark,
            ThemeFamily::RosePine => ThemeVariant::RosePine,
            ThemeFamily::Dracula => ThemeVariant::DraculaDark,
            ThemeFamily::Nord => ThemeVariant::NordDark,
            ThemeFamily::Gruvbox => ThemeVariant::GruvboxDark,
            ThemeFamily::Catppuccin => ThemeVariant::CatppuccinMocha,
        }
    }
}

pub struct ThemeManager {
    pub current_family: ThemeFamily,
    pub current_variant: ThemeVariant,
    pub corner_roundness: f32,
    pub show_family_dropdown: bool,
    pub show_variant_dropdown: bool,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_family: ThemeFamily::CherryBlossom,
            current_variant: ThemeVariant::CherryBlossomDark,
            corner_roundness: 8.0,
            show_family_dropdown: false,
            show_variant_dropdown: false,
        }
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&self, ctx: &egui::Context) {
        self.current_variant.apply(ctx, self.corner_roundness);
    }

    pub fn set_family(&mut self, family: ThemeFamily) {
        self.current_family = family;
        self.current_variant = family.default_variant();
    }

    pub fn set_variant(&mut self, variant: ThemeVariant) {
        self.current_variant = variant;
        self.current_family = match variant {
            ThemeVariant::CherryBlossomDark | ThemeVariant::CherryBlossomLight => {
                ThemeFamily::CherryBlossom
            }
            ThemeVariant::RosePine | ThemeVariant::RosePineMoon | ThemeVariant::RosePineDawn => {
                ThemeFamily::RosePine
            }
            ThemeVariant::DraculaDark => ThemeFamily::Dracula,
            ThemeVariant::NordDark => ThemeFamily::Nord,
            ThemeVariant::GruvboxDark | ThemeVariant::GruvboxLight => ThemeFamily::Gruvbox,
            ThemeVariant::CatppuccinFrappé
            | ThemeVariant::CatppuccinLatte
            | ThemeVariant::CatppuccinMacchiato
            | ThemeVariant::CatppuccinMocha => ThemeFamily::Catppuccin,
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Theme:");

        egui::ComboBox::from_id_salt("theme_family")
            .selected_text(self.current_family.name())
            .show_ui(ui, |ui| {
                for family in [
                    ThemeFamily::CherryBlossom,
                    ThemeFamily::RosePine,
                    ThemeFamily::Dracula,
                    ThemeFamily::Nord,
                    ThemeFamily::Gruvbox,
                    ThemeFamily::Catppuccin,
                ] {
                    if ui
                        .selectable_label(self.current_family == family, family.name())
                        .clicked()
                    {
                        self.set_family(family);
                    }
                }
            });

        ui.add_space(8.0);

        egui::ComboBox::from_id_salt("theme_variant")
            .selected_text(self.current_variant.name())
            .show_ui(ui, |ui| {
                for &variant in self.current_family.variants() {
                    if ui
                        .selectable_label(self.current_variant == variant, variant.name())
                        .clicked()
                    {
                        self.current_variant = variant;
                    }
                }
            });

        ui.add_space(8.0);

        ui.label("Corner Roundness:");
        ui.add(egui::Slider::new(&mut self.corner_roundness, 0.0..=20.0));

        if ui.button("Apply Theme").clicked() {
            self.apply(ui.ctx());
        }
    }

    pub fn all_families() -> &'static [ThemeFamily] {
        &[
            ThemeFamily::CherryBlossom,
            ThemeFamily::RosePine,
            ThemeFamily::Dracula,
            ThemeFamily::Nord,
            ThemeFamily::Gruvbox,
            ThemeFamily::Catppuccin,
        ]
    }
}

pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}

pub struct CherryBlossomTheme;

impl CherryBlossomTheme {
    pub fn bg_darkest() -> Color32 {
        current_theme_colors().bg_darkest
    }
    pub fn bg_dark() -> Color32 {
        current_theme_colors().bg_dark
    }
    pub fn bg_mid() -> Color32 {
        current_theme_colors().bg_mid
    }
    pub fn bg_light() -> Color32 {
        current_theme_colors().bg_light
    }
    pub fn bg_lighter() -> Color32 {
        current_theme_colors().bg_lighter
    }
    pub fn border_pink() -> Color32 {
        current_theme_colors().border
    }
    pub fn text_primary() -> Color32 {
        current_theme_colors().text_primary
    }
    pub fn text_secondary() -> Color32 {
        current_theme_colors().text_secondary
    }
    pub fn text_muted() -> Color32 {
        current_theme_colors().text_muted
    }
    pub fn accent_pink() -> Color32 {
        current_theme_colors().accent_primary
    }
    pub fn accent_hot() -> Color32 {
        current_theme_colors().accent_hot
    }
    pub fn accent_light() -> Color32 {
        current_theme_colors().accent_light
    }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let colors = current_theme_colors();

        let is_dark = colors.bg_darkest.r() < 128;
        let mut visuals = if is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = colors.bg_dark;
        visuals.panel_fill = colors.bg_dark;
        visuals.window_stroke = egui::Stroke::new(1.0, colors.bg_light);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = colors.bg_mid;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = colors.bg_light;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = colors.bg_lighter;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = colors.accent_hot;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = colors.accent_primary;

        visuals.selection.bg_fill = colors.accent_primary;
        visuals.selection.stroke = egui::Stroke::new(1.0, colors.accent_light);

        visuals.override_text_color = Some(colors.text_primary);
        visuals.hyperlink_color = colors.accent_primary;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, colors.accent_light);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, colors.bg_darkest);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.global_style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(20.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(13.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(11.0, egui::FontFamily::Proportional),
        );
        ctx.set_global_style(style);
    }
}

pub fn apply_theme_from_settings(
    ctx: &egui::Context,
    variant: ThemeVariant,
    corner_roundness: f32,
) {
    update_current_theme(variant);
    CherryBlossomTheme::apply(ctx, corner_roundness);
}
