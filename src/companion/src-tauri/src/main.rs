// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env, process::{exit, Command}, thread
};

// Imports only used in windows
#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, whiskers_launcher_rs::others::FLAG_NO_WINDOW};

pub mod indexing;
pub mod init;

use indexing::index_apps;
use serde::Serialize;
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu,
};
use tokio::time::sleep;
use whiskers_launcher_rs::{
    extensions::index_extensions,
    paths::get_local_dir,
    settings::{get_settings, index_settings},
};

pub mod functions;

#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("sync_apps", "Sync Apps"))
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("restart", "Restart"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    thread::spawn(move || {
                        if cfg!(target_os = "linux") {
                            Command::new("sh")
                                .arg("-c")
                                .arg("whiskers-launcher")
                                .output()
                                .expect("Error opening app");
                        }

                        #[cfg(target_os = "windows")]
                        if cfg!(target_os = "windows") {
                            Command::new("cmd")
                                .arg("/C")
                                .arg("start whiskers-launcher")
                                .current_dir(get_local_dir().unwrap())
                                .creation_flags(FLAG_NO_WINDOW)
                                .spawn()
                                .expect("Error opening app");
                        }
                    });
                }
                "sync_apps" => {
                    tokio::spawn(async {
                        index_apps();
                    });
                }
                "settings" => {
                    thread::spawn(|| {
                        if cfg!(target_os = "linux") {
                            Command::new("sh")
                                .arg("-c")
                                .arg("whiskers-launcher --settings")
                                .output()
                                .expect("Error opening app");
                        }

                        #[cfg(target_os = "windows")]
                        if cfg!(target_os = "windows") {
                            Command::new("cmd")
                                .arg("/C")
                                .arg("start whiskers-launcher --settings")
                                .current_dir(get_local_dir().unwrap())
                                .creation_flags(FLAG_NO_WINDOW)
                                .spawn()
                                .expect("Error opening app");
                        }
                    });
                }
                "restart" => {
                    let app_clone = app.to_owned();
                    tauri::api::process::restart(&app_clone.env());
                }
                "quit" => {
                    exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            app.get_window("main").unwrap().hide().unwrap();

            index_settings().unwrap();
            index_extensions().unwrap();

            //Re-indexes the apps every minute
            tokio::spawn(async move {
                loop {
                    {
                        index_apps();
                    }
                    sleep(tokio::time::Duration::from_secs(60)).await;
                }
            });

            
            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
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
                let settings = get_settings().unwrap();
                let first_key = settings.launch_first_key;
                let second_key = settings.launch_second_key;
                let third_key = settings.launch_third_key;

                let shortcut = match second_key {
                    Some(second_key) => format!("{}+{}+{}", &first_key, &second_key, &third_key),
                    None => format!("{}+{}", &first_key, &third_key),
                };

                let server = env::var("XDG_SESSION_TYPE").unwrap_or("".to_string());

                if server != "wayland"{
                    app.clone()
                    .global_shortcut_manager()
                    .register(&shortcut, move || {
                        thread::spawn(move || {
                            if cfg!(target_os = "linux") {
                                Command::new("sh")
                                    .arg("-c")
                                    .arg("whiskers-launcher")
                                    .output()
                                    .expect("Error opening app");
                            }

                            #[cfg(target_os = "windows")]
                            if cfg!(target_os = "windows") {
                                Command::new("cmd")
                                    .arg("/C")
                                    .arg("start whiskers-launcher")
                                    .current_dir(get_local_dir().unwrap())
                                    .creation_flags(FLAG_NO_WINDOW)
                                    .spawn()
                                    .expect("Error opening app");
                            }
                        });
                    })
                    .unwrap();
                }
            }
            _ => {}
        })
}
