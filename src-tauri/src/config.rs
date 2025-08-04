//! `config.json` management commands
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub install_directory: Option<String>,
}

fn get_config_path(app_handle: &tauri::AppHandle) -> crate::Result<PathBuf> {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| crate::Error::Config("Could not resolve app data directory".into()))?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path.join("config.json"))
}

fn read_config(path: &PathBuf) -> crate::Result<AppConfig> {
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(path)?;
    let config: AppConfig = serde_json::from_str(&content)?;
    Ok(config)
}

#[tauri::command]
pub async fn get_config(app_handle: tauri::AppHandle) -> crate::Result<AppConfig> {
    let path = get_config_path(&app_handle)?;
    read_config(&path)
}

#[tauri::command]
pub async fn save_config(app_handle: tauri::AppHandle, config: AppConfig) -> crate::Result<()> {
    let path = get_config_path(&app_handle)?;
    let content = serde_json::to_string_pretty(&config)?;
    fs::write(path, content)?;
    Ok(())
}
