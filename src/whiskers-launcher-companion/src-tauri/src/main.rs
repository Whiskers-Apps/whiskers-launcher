// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    process::exit,
};

use indexing::index_apps;
use serde::Serialize;
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu,
};
use tokio::time::sleep;
use whiskers_launcher_rs::api::settings::get_settings;
use windows::{open_launcher_window, open_settings_window};

mod windows;
mod indexing;

#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("restart", "Restart"))
        .add_item(CustomMenuItem::new("stop", "Stop"));

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    open_launcher_window();
                }
                "settings" => {
                    open_settings_window();
                }
                "restart" => {
                    tauri::api::process::restart(&app.env());
                }
                "stop" => {
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
              loop{
                {
                  index_apps();
                }

                // Reindex apps every 5 minutes
                sleep(tokio::time::Duration::from_secs(300)).await;
              }
            });

            Ok(())
        })
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
        .expect("")
        .run(|app, e| match e {
            RunEvent::Ready => {
                let display_server = env::var("XDG_SESSION_TYPE").unwrap_or(String::from(""));

                if display_server != "wayland" {
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

                    app.clone()
                        .global_shortcut_manager()
                        .register(&shortcut, move || {
                            open_launcher_window();
                        })
                        .unwrap();
                }
            }
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
