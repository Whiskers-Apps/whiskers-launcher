// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod features;
use std::env;

use features::actions::*;
use features::search::*;
use features::settings::*;
use features::window_managing::*;
use features::form::*;

use enigo::{Enigo, Mouse, Settings};
use serde::Serialize;
use tauri::{Manager, RunEvent, WindowBuilder, WindowEvent};
use whiskers_launcher_core::features::core::settings::get_settings;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_open_settings_window,
            run_get_settings,
            run_get_os,
            run_write_settings,
            run_on_wayland,
            run_get_blacklisted_apps,
            run_get_whitelisted_apps,
            run_add_to_blacklist,
            run_remove_from_blacklist,
            run_get_new_search_engine_id,
            run_add_search_engine,
            run_update_search_engine,
            run_delete_search_engine,
            run_get_theme_from_file,
            run_export_theme,
            run_get_search_results,
            run_action,
            run_clone_extension,
            run_get_extensions,
            run_open_extension_dir,
            run_index_extensions,
            run_remove_extension,
            run_get_extensions_store,
            run_write_extensions_store,
            run_get_themes_store,
            run_write_themes_store,
            run_wallpaper_exists,
            run_get_form_request,
            run_write_form_response,
            run_extension_action
        ])
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments
                .iter()
                .any(|arg| arg == "--settings" || arg == "-s");

            let main_window = app
                .handle()
                .to_owned()
                .get_window("main")
                .expect("Error getting main window");

            if open_settings {
                let app_clone = app.handle().to_owned();

                let window = WindowBuilder::new(
                    &app_clone,
                    "settings",
                    tauri::WindowUrl::App("settings".into()),
                )
                .title("Settings")
                .inner_size(1200.0, 700.0);

                window.build().expect("Error opening settings window");

                return Ok(());
            }

            // Opens the window in the monitor where the cursor is and centers it

            let monitors = main_window
                .available_monitors()
                .expect("Error getting monitors");

            let (cursor_x, _cursor_y) = Enigo::new(&Settings::default())
                .expect("Error initializing enigo")
                .location()
                .expect("Error getting cursor location");

            println!("Cursor: {} {}", cursor_x, _cursor_y);

            for monitor in monitors.iter() {
                let settings = get_settings();
                
                if settings.wallpaper.is_some() {
                    main_window.set_fullscreen(true).unwrap();
                    main_window.maximize().expect("Error maximizing window");
                }

                main_window.show().unwrap();
            }

            main_window.show().expect("Error showing window");

            Ok(())
        })
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
        .build(tauri::generate_context!())
        .expect("Error building app")
        .run(|app, e| match e {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::Focused(focused) => {
                            if !focused {
                                let settings = get_settings();

                                if settings.hide_on_blur && settings.wallpaper.is_some() {
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
