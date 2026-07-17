use serde::{
    Deserialize,
    Serialize,
};

/// Public surface for the config.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub appearance: Appearance,
}

impl Config {
    pub fn new() -> Self {
        Self {
            appearance: Appearance {
                theme: iced::Theme::GruvboxLight,
                scale: 1.0,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Fills in any missing fields from `inner` using `Config::default()`.
impl From<InnerConfig> for Config {
    fn from(inner: InnerConfig) -> Self {
        let defaults = Config::default();
        Self {
            appearance: inner
                .appearance
                .map(|a| Appearance {
                    theme: a
                        .theme
                        .map(iced::Theme::from)
                        .unwrap_or(defaults.appearance.theme.clone()),
                    scale: a.scale.unwrap_or(defaults.appearance.scale.clone()),
                })
                .unwrap_or(defaults.appearance.clone()),
        }
    }
}

/// For writing config back out to disk — only include what differs
/// from defaults if you want a "sparse" file, or just mirror everything.
impl From<&Config> for InnerConfig {
    fn from(config: &Config) -> Self {
        Self {
            appearance: Some(InnerAppearance {
                theme: crate::theme::ThemeSetting::try_from(&config.appearance.theme).ok(),
                scale: Some(config.appearance.scale),
            }),
        }
    }
}

/// Reader to config file
#[derive(Deserialize, Serialize, Default)]
pub(crate) struct InnerConfig {
    appearance: Option<InnerAppearance>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Appearance {
    /// Inner to `iced::Theme` until I feel like adding custom themes
    pub theme: iced::Theme,
    pub scale: f32,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct InnerAppearance {
    theme: Option<crate::theme::ThemeSetting>,
    scale: Option<f32>,
}
