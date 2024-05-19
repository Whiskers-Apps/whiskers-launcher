// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod windows;

use commands::settings::{
    add_search_engine, add_to_blacklist, delete_search_engine, export_theme, get_blacklisted_apps,
    get_new_search_engine_id, get_settings, get_theme_from_file, get_whitelisted_apps, is_wayland,
    remove_from_blacklist, update_search_engine, write_settings,
};
use windows::open_settings_window;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_settings_window,
            get_settings,
            write_settings,
            is_wayland,
            get_blacklisted_apps,
            get_whitelisted_apps,
            add_to_blacklist,
            remove_from_blacklist,
            get_new_search_engine_id,
            add_search_engine,
            update_search_engine,
            delete_search_engine,
            get_theme_from_file,
            export_theme
        ])
        .run(tauri::generate_context!())
        .expect("Error running app");
}
