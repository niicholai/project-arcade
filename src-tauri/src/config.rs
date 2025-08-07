use std::sync::OnceLock;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use crate::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub install_path: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug)]
pub struct ApiConfig {
    pub igdb_client_id: String,
    pub igdb_client_secret: String,
    pub giant_bomb_api_key: String,
}

static API_CONFIG: OnceLock<ApiConfig> = OnceLock::new();

pub fn init_api_config() {
    let _ = API_CONFIG.set(ApiConfig {
        igdb_client_id: String::from("sv7yvq6pvsvc2kanwmaqxwmpe7d7ql"),
        igdb_client_secret: String::from("3pu41ddsqlf1lakehfbb211dtz38e2"),
        giant_bomb_api_key: String::from("6646a4b9c58d6f5eafddefb1213f2d4e35da2882"),
    });
}

pub fn get_api_config() -> &'static ApiConfig {
    API_CONFIG.get().expect("API config not initialized")
}

#[tauri::command]
pub async fn get_config(app_handle: AppHandle) -> Result<Config> {
    let path = app_handle.path_resolver().app_config_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;

    std::fs::create_dir_all(&path)?;
    let config_path = path.join("config.json");

    if !config_path.exists() {
        let default_config = Config {
            install_path: None,
            theme: None,
        };
        let config_json = serde_json::to_string_pretty(&default_config)?;
        std::fs::write(&config_path, config_json)?;
        Ok(default_config)
    } else {
        let config_str = std::fs::read_to_string(&config_path)?;
        let config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}

#[tauri::command]
pub async fn save_config(app_handle: AppHandle, config: Config) -> Result<()> {
    let path = app_handle.path_resolver().app_config_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;

    std::fs::create_dir_all(&path)?;
    let config_path = path.join("config.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    std::fs::write(&config_path, config_json)?;
    Ok(())
}