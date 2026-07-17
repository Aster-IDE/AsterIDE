use iced::{Color, theme::Palette};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
                iced::Theme::custom("Cherry Blossom Light".to_string(), CHERRY_BLOSSOM_LIGHT)
            }
            ThemeSetting::CherryBlossomDark => {
                iced::Theme::custom("Cherry Blossom Dark".to_string(), CHERRY_BLOSSOM_DARK)
            }
            ThemeSetting::RosePine => iced::Theme::custom("Rose Pine".to_string(), ROSE_PINE),
            ThemeSetting::RosePineMoon => {
                iced::Theme::custom("Rose Pine Moon".to_string(), ROSE_PINE_MOON)
            }
            ThemeSetting::RosePineDawn => {
                iced::Theme::custom("Rose Pine Dawn".to_string(), ROSE_PINE_DAWN)
            }
        }
    }
}

impl TryFrom<&iced::Theme> for ThemeSetting {
    type Error = String;

    fn try_from(theme: &iced::Theme) -> Result<Self, Self::Error> {
        match theme {
            iced::Theme::Light => Ok(ThemeSetting::Light),
            iced::Theme::Dark => Ok(ThemeSetting::Dark),
            iced::Theme::Dracula => Ok(ThemeSetting::Dracula),
            iced::Theme::Nord => Ok(ThemeSetting::Nord),
            iced::Theme::SolarizedLight => Ok(ThemeSetting::SolarizedLight),
            iced::Theme::SolarizedDark => Ok(ThemeSetting::SolarizedDark),
            iced::Theme::GruvboxLight => Ok(ThemeSetting::GruvboxLight),
            iced::Theme::GruvboxDark => Ok(ThemeSetting::GruvboxDark),
            iced::Theme::CatppuccinLatte => Ok(ThemeSetting::CatppuccinLatte),
            iced::Theme::CatppuccinFrappe => Ok(ThemeSetting::CatppuccinFrappe),
            iced::Theme::CatppuccinMacchiato => Ok(ThemeSetting::CatppuccinMacchiato),
            iced::Theme::CatppuccinMocha => Ok(ThemeSetting::CatppuccinMocha),
            iced::Theme::TokyoNight => Ok(ThemeSetting::TokyoNight),
            iced::Theme::TokyoNightStorm => Ok(ThemeSetting::TokyoNightStorm),
            iced::Theme::TokyoNightLight => Ok(ThemeSetting::TokyoNightLight),
            iced::Theme::KanagawaWave => Ok(ThemeSetting::KanagawaWave),
            iced::Theme::KanagawaDragon => Ok(ThemeSetting::KanagawaDragon),
            iced::Theme::KanagawaLotus => Ok(ThemeSetting::KanagawaLotus),
            iced::Theme::Moonfly => Ok(ThemeSetting::Moonfly),
            iced::Theme::Nightfly => Ok(ThemeSetting::Nightfly),
            iced::Theme::Oxocarbon => Ok(ThemeSetting::Oxocarbon),
            iced::Theme::Ferra => Ok(ThemeSetting::Ferra),
            iced::Theme::Custom(_) => match theme.to_string().as_str() {
                "Cherry Blossom Light" => Ok(ThemeSetting::CherryBlossomLight),
                "Cherry Blossom Dark" => Ok(ThemeSetting::CherryBlossomDark),
                "Rose Pine" => Ok(ThemeSetting::RosePine),
                "Rose Pine Moon" => Ok(ThemeSetting::RosePineMoon),
                "Rose Pine Dawn" => Ok(ThemeSetting::RosePineDawn),
                _ => Err(format!("Unknown custom theme name: {}", theme.to_string())),
            },
        }
    }
}
