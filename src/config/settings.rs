use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub refresh_rate: u64,
    pub show_gpu: bool,
    pub show_temperature: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            refresh_rate: 1000,
            show_gpu: true,
            show_temperature: true,
        }
    }
}

impl Settings {
    pub fn get_config_path() -> PathBuf {
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let mut path = PathBuf::from(user_profile);
            path.push("Desktop");
            path.push("sysinfo.toml");
            path
        } else {
            // Fallback to current directory if USERPROFILE is not found
            PathBuf::from("sysinfo.toml")
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_config_path();
        
        if !path.exists() {
            let default_settings = Settings::default();
            default_settings.save()?;
            return Ok(default_settings);
        }

        let content = fs::read_to_string(&path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path();
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
