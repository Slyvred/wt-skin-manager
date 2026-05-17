use serde::{Deserialize, Serialize};
use std;
use std::path::Path;
// use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    pub version: String,
    pub game_dir: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let config_path = Path::new(path);

        let content = std::fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config: {e}"))?;

        let config: Config =
            serde_json::from_str(&content).map_err(|e| format!("JSON error: {e}"))?;

        Ok(config)
    }
    pub fn save(&self) -> Result<(), String> {
        let config_str =
            serde_json::to_string(self).map_err(|e| format!("Failed to serialize config: {e}"))?;

        std::fs::write("./config.json", config_str)
            .map_err(|e| format!("Failed to write config file: {e}"))?;

        Ok(())
    }
}
