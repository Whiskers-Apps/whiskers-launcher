// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod windows;

use std::env;

use commands::{search::*, settings::*};
use enigo::{Enigo, Mouse, Settings};
use serde::Serialize;
use tauri::{Manager, PhysicalPosition, RunEvent, WindowBuilder, WindowEvent, WindowUrl};
use whiskers_launcher_rs::api::settings;
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
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments.iter().any(|arg| arg == "--settings" || arg == "-s");

            let main_window = app
                .handle()
                .to_owned()
                .get_window("main")
                .expect("Error getting main window");

            if open_settings {
                WindowBuilder::new(app, "settings", WindowUrl::App("settings".into()))
                    .title("Settings")
                    .build()
                    .expect("Error creating settings window");

                main_window.close().expect("Error closing window");

                return Ok(());
            }

            // Opens the window in the monitor where the cursor is
            if !is_wayland() {
                let monitors = main_window
                    .available_monitors()
                    .expect("Error getting monitors");

                let (cursor_x, _cursor_y) = Enigo::new(&Settings::default())
                    .expect("Error initializing enigo")
                    .location()
                    .expect("Error getting cursor location");

                let mut monitors_x: Vec<i32> = monitors
                    .iter()
                    .map(|monitor| monitor.position().x)
                    .collect();

                monitors_x.sort_by(|a, b| b.cmp(a));

                for monitor_x in monitors_x {
                    if monitor_x <= cursor_x {
                        main_window
                            .set_position(PhysicalPosition::new(monitor_x, 0))
                            .expect("Error moving window");

                        main_window.center().expect("Error centering window");

                        break;
                    }
                }
            }

            main_window.set_fullscreen(true).unwrap();
            main_window.maximize().expect("Error maximizing window");
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
                                let settings = settings::get_settings();

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
