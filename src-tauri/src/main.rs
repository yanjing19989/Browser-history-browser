// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod db;
mod domain;

use commands::{
    browse_db_file, get_config, list_history, set_db_path, stats_overview, validate_db_path,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_history,
            stats_overview,
            get_config,
            set_db_path,
            validate_db_path,
            browse_db_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
