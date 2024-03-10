// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pages;

use pages::search::*;
use pages::settings_page::*;

use enigo::MouseControllable;
use std::env;
use tauri::{Manager, PhysicalPosition, RunEvent, WindowBuilder, WindowEvent, WindowUrl};
use whiskers_launcher_rs::settings;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            get_whitelisted_apps,
            add_to_blacklist,
            get_blacklisted_apps,
            export_theme,
            import_theme,
            get_search_results,
            open_app,
            open_url,
            get_user_extensions,
            get_extensions_default_values,
            run_extension_action,
            open_extension_dialog,
            get_extension_dialog_action,
            close_extension_dialog,
            clone_extension,
            uninstall_extension,
            get_cached_themes_store,
            get_cached_extensions_store,
            cache_themes,
            apply_store_theme,
            cache_extensions,
            index_extensions,
            get_os,
            open_extension
        ])
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments.iter().any(|arg| arg == "--settings");

            if open_settings {
                WindowBuilder::new(app, "settings", WindowUrl::App("settings".into()))
                    .title("Settings")
                    .build()
                    .expect("Error creating settings window");

                let main_window = app.get_window("main").unwrap();
                main_window.close().expect("Error closing search window");

                return Ok(());
            }

            let (cursor_x, cursor_y) = enigo::Enigo::new().mouse_location();

            let main_window = app.get_window("main").unwrap();

            main_window
                .set_position(PhysicalPosition {
                    x: cursor_x,
                    y: cursor_y,
                })
                .unwrap();

            main_window.set_always_on_top(true).unwrap();
            main_window.show().unwrap();
            main_window.set_focus().unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .build(tauri::generate_context!())
        .unwrap()
        .run(|app, e| match e {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::Focused(focused) => {
                            if !focused {
                                let settings = settings::get_settings().unwrap();
                                if settings.hide_on_blur {
                                    let window = app.get_window("main").unwrap();
                                    window.close().unwrap();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
}
