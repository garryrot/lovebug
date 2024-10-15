
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{info, Level};

pub static SETTINGS_PATH: &str = "Data\\F4SE\\Plugins";
pub static SETTINGS_FILE: &str = "Lovebug.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModSettings {
    pub version: u32,
    pub log_level: LogLevel
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl ModSettings {
    pub fn default() -> Self {
        Self {
            version: 0,
            log_level: LogLevel::Debug,
        }
    }

    pub fn try_read_or_default() -> ModSettings {
        let path = [SETTINGS_PATH, SETTINGS_FILE].iter().collect::<PathBuf>();
    
        match fs::read_to_string(path) {
            Ok(settings_json) => match serde_json::from_str::<ModSettings>(&settings_json) {
                Ok(settings) => settings,
                Err(_err) => ModSettings::default(),
            },
            Err(err) => {
                info!("Settings path '{}' could not be opened. Error: {}. Using default configuration.", SETTINGS_PATH, err);
                ModSettings::default()
            }
        }
    }    
}

impl From<LogLevel> for Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}
