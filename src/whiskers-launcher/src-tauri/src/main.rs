// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod features;
use std::env;
use std::process::exit;

//#[cfg(target_os = "windows")]
//{}

use features::actions::*;
use features::extensions::*;
use features::form::*;
use features::search::*;
use features::settings::*;
use features::window_managing::*;

use serde::Serialize;
use tauri::CustomMenuItem;
use tauri::GlobalShortcutManager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::{Manager, RunEvent, WindowEvent};
use tokio::time::sleep;
use whiskers_launcher_core::features::core::apps::index_apps;
use whiskers_launcher_core::features::core::extensions::index_extensions;
use whiskers_launcher_core::features::core::settings::get_settings;
use whiskers_launcher_core::utils::on_linux;
use whiskers_launcher_core::utils::on_wayland;

#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("refresh-apps", "Refresh Apps"))
        .add_item(CustomMenuItem::new("restart", "Restart"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

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
            run_extension_action,
            update_extension
        ])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    open_search_window(&app);
                }
                "settings" => {
                    open_settings_window(&app);
                }
                "refresh-apps" => {
                    tokio::spawn(async move {
                        index_apps();
                    });
                }
                "restart" => {
                    tauri::api::process::restart(&app.env());
                }
                "quit" => {
                    exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            app.get_window("main")
                .expect("Error getting main window")
                .close()
                .expect("Error closing main window");

            tokio::spawn(async move {
                let _ = get_settings();
                index_extensions();

                loop {
                    {
                        index_apps();
                    }

                    sleep(tokio::time::Duration::from_secs(300)).await;
                }
            });

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
                                    let window = &app.get_window("main").unwrap();
                                    window.close().unwrap();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            RunEvent::Ready => {
                if on_linux() && on_wayland() {
                    return;
                }

                let owned_app = app.to_owned();

                let settings = get_settings();
                let first_key = settings.first_key;
                let second_key = settings.second_key;
                let third_key = settings.third_key;

                let shortcut = match second_key {
                    Some(second_key) => {
                        format!("{}+{}+{}", &first_key, &second_key, &third_key)
                    }
                    None => format!("{}+{}", &first_key, &third_key),
                };

                owned_app
                    .global_shortcut_manager()
                    .register(&shortcut, move || {
                        open_search_window(&owned_app);
                    })
                    .expect("Error registering shortcut");
            }
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
