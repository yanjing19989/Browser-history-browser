// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod domain;
mod commands;
mod config;

use commands::{list_history, stats_overview, get_config, set_db_path, validate_db_path, browse_db_file};

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
