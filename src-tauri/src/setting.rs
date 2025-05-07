use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_config_dir;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub username: String,
    pub port: u16,
    pub enable_feature: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            username: "user".into(),
            port: 25565,
            enable_feature: true,
        }
    }
}

fn get_setting_path() -> Result<PathBuf, String> {
    app_config_dir(&tauri::Config::default())
        .map(|mut path| {
            path.push("setting.toml");
            path
        })
        .ok_or("Cannot determine config directory".to_string())
}

/// Tạo file setting.toml nếu chưa tồn tại
#[tauri::command]
pub fn create_default_settings() -> Result<(), String> {
    let path = get_setting_path()?;

    if !path.exists() {
        let default = AppSettings::default();
        let toml_str = toml::to_string_pretty(&default)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        let mut file = fs::File::create(&path)
            .map_err(|e| format!("Failed to create setting.toml: {}", e))?;
        file.write_all(toml_str.as_bytes())
            .map_err(|e| format!("Failed to write to setting.toml: {}", e))?;
    }

    Ok(())
}

/// Đọc setting hiện tại
#[tauri::command]
pub fn get_settings() -> Result<AppSettings, String> {
    let path = get_setting_path()?;

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read setting.toml: {}", e))?;

    toml::from_str(&content)
        .map_err(|e| format!("Failed to parse setting.toml: {}", e))
}
