use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const DEFAULT_BASE_URL: &str = "https://api.rokha.ai";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Remote Rokha API base URL. Default `https://api.rokha.ai`.
    /// Override with `ro config set-base-url <url>` or `ROKHA_BASE_URL` env.
    #[serde(default = "default_base_url")]
    pub erebus_url: String,
}

fn default_base_url() -> String {
    DEFAULT_BASE_URL.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            erebus_url: default_base_url(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        // Env override wins. Useful for `ROKHA_BASE_URL=http://localhost:3000 ro chat ...`
        // during local server dev.
        if let Ok(url) = std::env::var("ROKHA_BASE_URL") {
            if !url.is_empty() {
                return Self { erebus_url: url };
            }
        }
        let path = Self::config_path();
        if path.exists() {
            let contents = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<PathBuf> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let body = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(&path, body)?;
        Ok(path)
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".rokha")
            .join("config.toml")
    }
}
