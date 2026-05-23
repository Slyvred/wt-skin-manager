use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct InstalledSkin {
    pub lang_group: i32,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    pub version: String,
    pub game_dir: String,
    pub installed_skins: Vec<InstalledSkin>,
}

impl Config {
    pub fn get_path() -> Result<PathBuf, String> {
        ProjectDirs::from("com", "slyvred", "wt-skin-manager")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.json"))
            .ok_or_else(|| "Failed to get config path".to_string())
    }

    pub fn load_from_file() -> Result<Self, String> {
        let config_path = Self::get_path()?;

        let content = std::fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config: {e}"))?;

        let config: Config =
            serde_json::from_str(&content).map_err(|e| format!("JSON error: {e}"))?;

        if config.version != "1.1.0".to_string() {
            return Err("Config is too old !".into());
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_path()?;

        if let Some(parent_dir) = config_path.parent() {
            std::fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create config directory: {e}"))?;
        }

        let config_str = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {e}"))?;

        std::fs::write(&config_path, config_str)
            .map_err(|e| format!("Failed to write config file: {e}"))?;

        Ok(())
    }
}
