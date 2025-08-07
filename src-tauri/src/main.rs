// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod database;
mod error;
mod models;
mod services;
mod state;

use tauri::Manager;

pub use error::{Error, Result};
pub use state::AppState;

fn main() {
    // Initialize API configuration
    config::init_api_config();
    
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                // Get the path to the app's data directory
                let app_data_dir = handle
                    .path_resolver()
                    .app_data_dir()
                    .expect("failed to resolve app data dir");

                let db_path = app_data_dir.join("library.db");

                // Initialize the database
                let db_pool = database::init(&db_path)
                    .await
                    .expect("failed to initialize database");

                // Store the database pool in the app's state
                handle.manage(AppState { db: db_pool });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::library::get_games,
            commands::library::get_game_details,
            commands::library::add_game_manually,
            commands::library::remove_game,
            commands::library::refresh_metadata,
            commands::installer::install_game,
            commands::installer::launch_game,
            config::get_config,
            config::save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
