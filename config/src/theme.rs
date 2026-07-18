use iced::{Color, theme::Palette};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeOption {
    pub key: String,
    pub label: String,
    pub theme: iced::Theme,
}

impl std::fmt::Display for ThemeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeSetting {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
    CherryBlossomLight,
    CherryBlossomDark,
    RosePine,
    RosePineDawn,
    RosePineMoon,
}

impl ThemeSetting {
    pub fn all() -> &'static [ThemeSetting] {
        use ThemeSetting::*;
        &[
            Light,
            Dark,
            Dracula,
            Nord,
            SolarizedLight,
            SolarizedDark,
            GruvboxLight,
            GruvboxDark,
            CatppuccinLatte,
            CatppuccinFrappe,
            CatppuccinMacchiato,
            CatppuccinMocha,
            TokyoNight,
            TokyoNightStorm,
            TokyoNightLight,
            KanagawaWave,
            KanagawaDragon,
            KanagawaLotus,
            Moonfly,
            Nightfly,
            Oxocarbon,
            Ferra,
            CherryBlossomLight,
            CherryBlossomDark,
            RosePine,
            RosePineDawn,
            RosePineMoon,
        ]
    }

    pub fn iced_all() -> Vec<ThemeOption> {
        Self::all()
            .iter()
            .map(|t| ThemeOption {
                key: t.key().to_string(),
                label: t.display_name().to_string(),
                theme: (*t).into(),
            })
            .collect()
    }

    pub fn key(&self) -> &'static str {
        use ThemeSetting::*;
        match self {
            Light => "light",
            Dark => "dark",
            Dracula => "dracula",
            Nord => "nord",
            SolarizedLight => "solarized_light",
            SolarizedDark => "solarized_dark",
            GruvboxLight => "gruvbox_light",
            GruvboxDark => "gruvbox_dark",
            CatppuccinLatte => "catppuccin_latte",
            CatppuccinFrappe => "catppuccin_frappe",
            CatppuccinMacchiato => "catppuccin_macchiato",
            CatppuccinMocha => "catppuccin_mocha",
            TokyoNight => "tokyo_night",
            TokyoNightStorm => "tokyo_night_storm",
            TokyoNightLight => "tokyo_night_light",
            KanagawaWave => "kanagawa_wave",
            KanagawaDragon => "kanagawa_dragon",
            KanagawaLotus => "kanagawa_lotus",
            Moonfly => "moonfly",
            Nightfly => "nightfly",
            Oxocarbon => "oxocarbon",
            Ferra => "ferra",
            CherryBlossomLight => "cherry_blossom_light",
            CherryBlossomDark => "cherry_blossom_dark",
            RosePine => "rose_pine",
            RosePineDawn => "rose_pine_dawn",
            RosePineMoon => "rose_pine_moon",
        }
    }

    pub fn display_name(&self) -> &'static str {
        use ThemeSetting::*;
        match self {
            Light => "Light",
            Dark => "Dark",
            Dracula => "Dracula",
            Nord => "Nord",
            SolarizedLight => "Solarized Light",
            SolarizedDark => "Solarized Dark",
            GruvboxLight => "Gruvbox Light",
            GruvboxDark => "Gruvbox Dark",
            CatppuccinLatte => "Catppuccin Latte",
            CatppuccinFrappe => "Catppuccin Frappé",
            CatppuccinMacchiato => "Catppuccin Macchiato",
            CatppuccinMocha => "Catppuccin Mocha",
            TokyoNight => "Tokyo Night",
            TokyoNightStorm => "Tokyo Night Storm",
            TokyoNightLight => "Tokyo Night Light",
            KanagawaWave => "Kanagawa Wave",
            KanagawaDragon => "Kanagawa Dragon",
            KanagawaLotus => "Kanagawa Lotus",
            Moonfly => "Moonfly",
            Nightfly => "Nightfly",
            Oxocarbon => "Oxocarbon",
            Ferra => "Ferra",
            CherryBlossomLight => "Cherry Blossom Light",
            CherryBlossomDark => "Cherry Blossom Dark",
            RosePine => "Rosé Pine",
            RosePineDawn => "Rosé Pine Dawn",
            RosePineMoon => "Rosé Pine Moon",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::all().iter().copied().find(|t| t.key() == key)
    }
}

const CHERRY_BLOSSOM_LIGHT: Palette = Palette {
    background: Color::from_rgba8(245, 220, 230, 1.0),
    text: Color::from_rgba8(80, 40, 60, 1.0),
    primary: Color::from_rgba8(220, 80, 140, 1.0),
    success: Color::from_rgba8(0, 255, 0, 1.0),
    warning: Color::from_rgba8(255, 255, 0, 1.0),
    danger: Color::from_rgba8(255, 0, 0, 1.0),
};

const CHERRY_BLOSSOM_DARK: Palette = Palette {
    background: Color::from_rgba8(35, 20, 28, 1.0),
    text: Color::from_rgba8(235, 235, 245, 1.0),
    primary: Color::from_rgba8(235, 130, 180, 1.0),
    success: Color::from_rgba8(0, 255, 0, 1.0),
    warning: Color::from_rgba8(255, 255, 0, 1.0),
    danger: Color::from_rgba8(255, 0, 0, 1.0),
};

const ROSE_PINE: Palette = Palette {
    background: Color::from_rgba8(25, 23, 36, 1.0),
    text: Color::from_rgba8(224, 222, 244, 1.0),
    primary: Color::from_rgba8(196, 167, 231, 1.0),
    success: Color::from_rgba8(49, 116, 143, 1.0),
    warning: Color::from_rgba8(246, 193, 119, 1.0),
    danger: Color::from_rgba8(235, 111, 146, 1.0),
};

const ROSE_PINE_MOON: Palette = Palette {
    background: Color::from_rgba8(35, 33, 54, 1.0),
    text: Color::from_rgba8(224, 222, 244, 1.0),
    primary: Color::from_rgba8(196, 167, 231, 1.0),
    success: Color::from_rgba8(62, 143, 176, 1.0),
    warning: Color::from_rgba8(246, 193, 119, 1.0),
    danger: Color::from_rgba8(235, 111, 146, 1.0),
};

const ROSE_PINE_DAWN: Palette = Palette {
    background: Color::from_rgba8(250, 244, 237, 1.0),
    text: Color::from_rgba8(70, 66, 79, 1.0),
    primary: Color::from_rgba8(144, 122, 169, 1.0),
    success: Color::from_rgba8(40, 105, 131, 1.0),
    warning: Color::from_rgba8(234, 157, 52, 1.0),
    danger: Color::from_rgba8(180, 99, 122, 1.0),
};

impl From<ThemeSetting> for iced::Theme {
    fn from(setting: ThemeSetting) -> Self {
        match setting {
            ThemeSetting::Light => iced::Theme::Light,
            ThemeSetting::Dark => iced::Theme::Dark,
            ThemeSetting::Dracula => iced::Theme::Dracula,
            ThemeSetting::Nord => iced::Theme::Nord,
            ThemeSetting::SolarizedLight => iced::Theme::SolarizedLight,
            ThemeSetting::SolarizedDark => iced::Theme::SolarizedDark,
            ThemeSetting::GruvboxLight => iced::Theme::GruvboxLight,
            ThemeSetting::GruvboxDark => iced::Theme::GruvboxDark,
            ThemeSetting::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            ThemeSetting::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            ThemeSetting::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            ThemeSetting::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            ThemeSetting::TokyoNight => iced::Theme::TokyoNight,
            ThemeSetting::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            ThemeSetting::TokyoNightLight => iced::Theme::TokyoNightLight,
            ThemeSetting::KanagawaWave => iced::Theme::KanagawaWave,
            ThemeSetting::KanagawaDragon => iced::Theme::KanagawaDragon,
            ThemeSetting::KanagawaLotus => iced::Theme::KanagawaLotus,
            ThemeSetting::Moonfly => iced::Theme::Moonfly,
            ThemeSetting::Nightfly => iced::Theme::Nightfly,
            ThemeSetting::Oxocarbon => iced::Theme::Oxocarbon,
            ThemeSetting::Ferra => iced::Theme::Ferra,
            ThemeSetting::CherryBlossomLight => {
                iced::Theme::custom(setting.display_name().to_string(), CHERRY_BLOSSOM_LIGHT)
            }
            ThemeSetting::CherryBlossomDark => {
                iced::Theme::custom(setting.display_name().to_string(), CHERRY_BLOSSOM_DARK)
            }
            ThemeSetting::RosePine => {
                iced::Theme::custom(setting.display_name().to_string(), ROSE_PINE)
            }
            ThemeSetting::RosePineMoon => {
                iced::Theme::custom(setting.display_name().to_string(), ROSE_PINE_MOON)
            }
            ThemeSetting::RosePineDawn => {
                iced::Theme::custom(setting.display_name().to_string(), ROSE_PINE_DAWN)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CustomThemeSpec {
    pub display_name: String,
    #[serde(with = "hex_color")]
    pub background: Color,
    #[serde(with = "hex_color")]
    pub text: Color,
    #[serde(with = "hex_color")]
    pub primary: Color,
    #[serde(with = "hex_color")]
    pub success: Color,
    #[serde(with = "hex_color")]
    pub warning: Color,
    #[serde(with = "hex_color")]
    pub danger: Color,
}

impl CustomThemeSpec {
    pub fn to_iced_theme(&self) -> iced::Theme {
        iced::Theme::custom(
            self.display_name.clone(),
            Palette {
                background: self.background,
                text: self.text,
                primary: self.primary,
                success: self.success,
                warning: self.warning,
                danger: self.danger,
            },
        )
    }
}

mod hex_color {
    use iced::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(color: &Color, s: S) -> Result<S::Ok, S::Error> {
        let r = (color.r * 255.0).round() as u8;
        let g = (color.g * 255.0).round() as u8;
        let b = (color.b * 255.0).round() as u8;
        let a = (color.a * 255.0).round() as u8;
        if a == 255 {
            format!("#{r:02X}{g:02X}{b:02X}").serialize(s)
        } else {
            format!("#{r:02X}{g:02X}{b:02X}{a:02X}").serialize(s)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Color, D::Error> {
        let raw = String::deserialize(d)?;
        let hex = raw.trim().trim_start_matches('#');
        let bytes = |i: usize| -> Result<u8, D::Error> {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| serde::de::Error::custom(format!("bad hex in {raw:?}: {e}")))
        };
        match hex.len() {
            6 => Ok(Color::from_rgba8(bytes(0)?, bytes(2)?, bytes(4)?, 1.0)),
            8 => {
                let a = bytes(6)? as f32 / 255.0;
                Ok(Color::from_rgba8(bytes(0)?, bytes(2)?, bytes(4)?, a))
            }
            _ => Err(serde::de::Error::custom(format!(
                "expected #RRGGBB or #RRGGBBAA, got {raw:?}"
            ))),
        }
    }
}
