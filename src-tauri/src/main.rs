// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod db;
mod domain;

use commands::{
    browse_browser_db_file, browse_db_file, cleanup_old_dbs, copy_browser_db_to_app, get_config,
    list_history, open_db_directory, set_browser_db_path, set_db_path, set_top_sites_count,
    stats_overview, validate_db_path,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_history,
            stats_overview,
            get_config,
            set_db_path,
            validate_db_path,
            browse_db_file,
            browse_browser_db_file,
            copy_browser_db_to_app,
            set_browser_db_path,
            open_db_directory,
            cleanup_old_dbs,
            set_top_sites_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
