use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultSection,
    #[serde(default)]
    pub profile: HashMap<String, Profile>,
    #[serde(default)]
    pub server: HashMap<String, Server>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultSection {
    pub output: String,
    pub color: bool,
}

impl Default for DefaultSection {
    fn default() -> Self {
        Self {
            output: "plain".to_string(),
            color: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Profile {
    pub base_url: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Server {
    pub host: String,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub key: Option<String>,
    pub ssl_dir: Option<String>,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".tooler")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(toml::from_str(&content)?)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, toml::to_string_pretty(config)?)?;
    Ok(())
}
