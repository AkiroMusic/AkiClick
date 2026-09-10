use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mode: u32,
    pub freq: u32,
    pub clicktimes: u32,
    pub clickstate: u32,
    pub showstate: u32,
    pub left: u32,
    pub right: u32,
    pub stop: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: 0,
            freq: 700,
            clicktimes: 100,
            clickstate: 1,
            showstate: 0,
            left: 120,  // F9
            right: 121, // F10
            stop: 122,  // F11
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: Config = serde_ini::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_ini::to_string(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        // Check for portable mode first (config next to exe)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let portable_path = exe_dir.join("shubiao.ini");
                if portable_path.exists() {
                    return Ok(portable_path);
                }
                // Check if we're in a development environment
                if exe_dir.join("Cargo.toml").exists() || exe_dir.join("target").exists() {
                    return Ok(portable_path);
                }
            }
        }

        // Use app data directory - standard Windows path
        let appdata = std::env::var("APPDATA")
            .or_else(|_| std::env::var("LOCALAPPDATA"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Fallback to current dir
                PathBuf::from(".")
            });
        let config_path = appdata.join("AkiClick").join("shubiao.ini");
        Ok(config_path)
    }

    pub fn is_portable() -> bool {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                return exe_dir.join("shubiao.ini").exists() 
                    || exe_dir.join("Cargo.toml").exists()
                    || exe_dir.join("target").exists();
            }
        }
        false
    }
}