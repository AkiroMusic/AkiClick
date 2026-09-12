use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const MIN_INTERVAL_MS: u32 = 1;
pub const MAX_INTERVAL_MS: u32 = 60_000;
pub const MAX_CLICK_COUNT: u32 = 999_999;

/// Settings persisted as `shubiao.ini`.
///
/// Field names follow the legacy 鼠标连点器 ("REF") format for compatibility —
/// they are misleading but MUST NOT be renamed while REF compatibility is a
/// documented feature:
/// - `left`  holds the START hotkey VK code
/// - `right` holds the STOP hotkey VK code
/// - `stop`  holds the EXIT hotkey VK code
/// - `clickstate` / `showstate` are unused; kept only so existing files and
///   REF-written files keep parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub mode: u32,
    pub freq: u32,
    pub clicktimes: u32,
    pub clickstate: u32,
    pub showstate: u32,
    pub left: u32,
    pub right: u32,
    pub stop: u32,
    // AkiClick UI preferences
    pub theme: String,
    pub lang: String,
    pub always_on_top: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: 0,
            freq: 700,
            clicktimes: 100,
            clickstate: 1,
            showstate: 0,
            left: 120,  // F9 — start
            right: 121, // F10 — stop
            stop: 122,  // F11 — exit
            theme: "dark".into(),
            lang: "en".into(),
            always_on_top: 1,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let mut config: Config = serde_ini::from_str(&content)?;
            config.sanitize();
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
        // Write-then-rename so a crash mid-write can't corrupt the config.
        let tmp = path.with_extension("ini.tmp");
        fs::write(&tmp, content)?;
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    /// Clamp every field into its valid range; repairs hand-edited or
    /// partially corrupted INI files instead of rejecting the whole file.
    pub fn sanitize(&mut self) {
        if self.mode > 2 {
            self.mode = 0;
        }
        self.freq = self.freq.clamp(MIN_INTERVAL_MS, MAX_INTERVAL_MS);
        self.clicktimes = self.clicktimes.min(MAX_CLICK_COUNT);
        self.clickstate = u32::from(self.clickstate > 0);
        self.showstate = u32::from(self.showstate > 0);
        if !crate::hotkeys::is_known_vk(self.left) {
            self.left = 120; // F9
        }
        if !crate::hotkeys::is_known_vk(self.right) {
            self.right = 121; // F10
        }
        if !crate::hotkeys::is_known_vk(self.stop) {
            self.stop = 122; // F11
        }
        if self.theme != "light" && self.theme != "dark" {
            self.theme = "dark".into();
        }
        if self.lang != "en" && self.lang != "zh" {
            self.lang = "en".into();
        }
        self.always_on_top = u32::from(self.always_on_top > 0);
    }

    // Legacy INI keys have misleading names (see struct docs); these accessors
    // give the real meaning so call sites don't have to remember the mapping.
    pub fn start_vk(&self) -> u32 {
        self.left
    }
    pub fn stop_vk(&self) -> u32 {
        self.right
    }
    pub fn exit_vk(&self) -> u32 {
        self.stop
    }
    pub fn interval_ms(&self) -> u32 {
        self.freq
    }
    pub fn click_count(&self) -> u32 {
        self.clicktimes
    }
    pub fn always_on_top_enabled(&self) -> bool {
        self.always_on_top == 1
    }

    pub fn config_path() -> Result<PathBuf> {
        // Portable mode: config already lives next to the exe
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let portable_path = exe_dir.join("shubiao.ini");
                if portable_path.exists() {
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
}
