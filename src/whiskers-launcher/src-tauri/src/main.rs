// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod windows;

use commands::{search::*, settings::*};
use serde::Serialize;
use tauri::Manager;
use windows::open_settings_window;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_settings_window,
            get_settings,
            get_os,
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
            export_theme,
            get_results,
            run_action,
            clone_extension,
            get_extensions,
            open_extension_dir,
            index_extensions,
            get_dialog_request,
            run_dialog_action,
            remove_extension,
            get_extensions_store,
            write_extensions_store,
            get_themes_store,
            write_themes_store
        ])
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            #[derive(Clone, Serialize)]
            struct PluginPayload {
                args: Vec<String>,
                cwd: String,
            }

            app.emit_all("single-instance", PluginPayload { args: argv, cwd })
                .unwrap();
        }))
        .run(tauri::generate_context!())
        .expect("Error running app");
}
