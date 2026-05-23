use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

/// Walk up from cwd until we find a .tooler.toml file.
pub fn find_config() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join(".tooler.toml");
        if candidate.exists() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Load the nearest .tooler.toml and return (config, project_root).
pub fn load() -> Result<(ProjectConfig, PathBuf)> {
    match find_config() {
        Some(path) => {
            let content = std::fs::read_to_string(&path)?;
            let config = toml::from_str(&content)?;
            let root = path.parent().unwrap_or(&path).to_path_buf();
            Ok((config, root))
        }
        None => Ok((ProjectConfig::default(), std::env::current_dir()?)),
    }
}
