use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_erebus_url")]
    pub erebus_url: String,
}

fn default_erebus_url() -> String {
    "http://localhost:3000".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            erebus_url: default_erebus_url(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let contents = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".rokha")
            .join("config.toml")
    }
}
