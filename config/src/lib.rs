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

pub mod spec;
pub mod theme;
use std::{
    env,
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// Configuration ring, holds all configuration that is in a config file.
static INSTANCE: RwLock<Option<Arc<spec::Config>>> = RwLock::new(None);

fn config_home() -> PathBuf {
    if cfg!(target_family = "unix") {
        std::env::var_os("XDG_CONFIG_HOME")
            .map_or_else(|| {
                let home_var: PathBuf = env::var("HOME").unwrap().into();
                home_var.join(".config")
            }, PathBuf::from)
            .join("asteride")
    } else {
        let home_var: PathBuf = env::var("LOCALAPPDATA").unwrap().into();
        home_var.join("asteride")
    }
}

fn resolve_config_path(override_path: Option<PathBuf>) -> PathBuf {
    let result = match override_path {
        Some(path) if path.is_dir() => path.join("config.toml"),
        Some(path) => path,
        None => config_home().join("config.toml"),
    };

    tracing::debug!("Resolved config path: {result:?}");
    result
}

/// Goes around and initializes vital stuff for config to
/// be read and written
pub fn init_ring(
    // used to override a path, e.g -c / --config
    override_default_path: Option<PathBuf>,
) {
    if let Some(path) = &override_default_path {
        tracing::info!("Using {path:?}");
    }

    let config_path = resolve_config_path(override_default_path);

    let inner_config = if config_path.is_file() {
        let raw = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|e| panic!("failed to read config at {config_path:?}: {e}"));
        toml::from_str(&raw)
            .unwrap_or_else(|e| panic!("failed to parse config at {config_path:?}: {e}"))
    } else {
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create config directory");
        }

        let default = spec::InnerConfig::default();
        let toml_str =
            toml::to_string_pretty(&default).expect("failed to serialize default config");
        std::fs::write(&config_path, toml_str).expect("failed to write default config");
        default
    };

    let config: spec::Config = inner_config.into();

    let mut instance_writer = INSTANCE.write().unwrap();
    *instance_writer = Some(Arc::new(config));
    tracing::debug!("Config ring filled with: {instance_writer:#?}");
}

pub fn get() -> std::sync::Arc<spec::Config> {
    INSTANCE
        .read()
        .unwrap()
        .as_ref()
        .expect("init_ring wasn't called before access")
        .clone()
}
