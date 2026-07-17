use serde::{
    Deserialize,
    Serialize,
};

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
}

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
        }
    }
}

impl TryFrom<&iced::Theme> for ThemeSetting {
    type Error = ();

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
            iced::Theme::Custom(_) => Err(()),
        }
    }
}
