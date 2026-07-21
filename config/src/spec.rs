/*
 * This file is part of AsterIDE.
 *
 * Copyright (c) 2026 AsterIDE
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::theme::{CustomThemeSpec, ThemeOption, ThemeSetting};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Public surface for the config.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub appearance: Appearance,
}

impl Config {
    pub fn new() -> Self {
        Self {
            appearance: Appearance {
                theme: "dark".to_string(),
                scale: 1.0,
                custom_themes: HashMap::new(),
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
                    theme: a.theme.unwrap_or_else(|| defaults.appearance.theme.clone()),
                    scale: a.scale.unwrap_or(defaults.appearance.scale),
                    custom_themes: a.custom_themes.unwrap_or_default(),
                })
                .unwrap_or(defaults.appearance),
        }
    }
}

/// For writing config back out to disk — only include what differs
/// from defaults if you want a "sparse" file, or just mirror everything.
impl From<&Config> for InnerConfig {
    fn from(config: &Config) -> Self {
        Self {
            appearance: Some(InnerAppearance {
                theme: Some(config.appearance.theme.clone()),
                scale: Some(config.appearance.scale),
                custom_themes: Some(config.appearance.custom_themes.clone()),
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
    pub theme: String,
    pub scale: f32,
    pub custom_themes: HashMap<String, CustomThemeSpec>,
}

impl Appearance {
    pub fn resolve_theme(&self) -> iced::Theme {
        if let Some(builtin) = ThemeSetting::from_key(&self.theme) {
            return builtin.into();
        }
        if let Some(custom) = self.custom_themes.get(&self.theme) {
            return custom.to_iced_theme();
        }
        tracing::error!(
            title = "Unknown theme key",
            description = format!(
                "{:?} was not found under appearance.custom_themes.{}",
                self.theme, self.theme
            )
        );
        std::process::exit(1);
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct InnerAppearance {
    theme: Option<String>,
    scale: Option<f32>,
    custom_themes: Option<HashMap<String, CustomThemeSpec>>,
}

pub fn theme_options(config: &Config) -> Vec<ThemeOption> {
    let mut options = ThemeSetting::iced_all();

    options.extend(
        config
            .appearance
            .custom_themes
            .iter()
            .map(|(key, spec)| ThemeOption {
                key: key.clone(),
                label: spec.display_name.clone(),
                theme: spec.to_iced_theme(),
            }),
    );

    options
}
