pub mod spec;
mod theme;
use std::{env, path::PathBuf, sync::OnceLock};

/// Configuration ring, holds all configuration that is in a config file.
static INSTANCE: OnceLock<spec::Config> = OnceLock::new();

fn config_home() -> PathBuf {
    if cfg!(target_family = "unix") {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let home_var: PathBuf = env::var("HOME").unwrap().into();
                home_var.join(".config")
            })
            .join("asteride")
    } else {
        let home_var: PathBuf = env::var("LOCALAPPDATA").unwrap().into();
        home_var.join("asteride")
    }
}

/// Goes around and initializes vital stuff for config to
/// be read and written
pub fn init_ring(
    // used to override a path, e.g -c / --config
    override_default_path: Option<PathBuf>,
) {
    let maybe_file = override_default_path.unwrap_or_else(config_home);
    let config_path = if maybe_file.is_dir() {
        maybe_file.join("config.toml")
    } else {
        maybe_file.to_path_buf()
    };

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

    INSTANCE.set(config).unwrap();
}

pub fn get() -> &'static spec::Config {
    INSTANCE
        .get()
        .expect("init_ring wasn't called before access")
}
