// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod windows;

use commands::settings::{
    add_to_blacklist, get_blacklisted_apps, get_settings, get_whitelisted_apps, is_wayland,
    remove_from_blacklist, write_settings,
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
        ])
        .run(tauri::generate_context!())
        .expect("Error running app");
}
