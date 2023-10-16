// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Imports only used in windows
#[cfg(target_os = "windows")]
use{
    std::os::windows::process::CommandExt,
    simple_kl_rs::others::FLAG_NO_WINDOW,
    simple_kl_rs::paths::get_local_dir,
    env
};


use std::path::Path;
use std::thread;
use std::{
    fs::{self, File},
    io::Write,
    process::{exit, Command},
};

#[cfg(target_os = "linux")]
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};

#[cfg(target_os = "linux")]
use functions::get_app_icon;

use simple_kl_rs::{
    extensions::init_extensions,
    paths::{get_apps_index_path, get_temp_directory},
};
use simple_kl_rs::settings::{get_settings, init_settings, self};
use structs::App;
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu,
};

use tokio::time::sleep;

pub mod functions;
pub mod structs;

/// Searches for all apps and saves them in a apps json file.
/// It reduces the time the app needs to get all the apps over and over again
fn index_apps() {
    
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let mut apps: Vec<App> = Vec::new();
        let mut ids: Vec<String> = Vec::new();

        //Gets All Apps
        for path in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                    if !ids.contains(&entry.appid.to_string()) && !entry.no_display() {
                        ids.push(entry.appid.to_string());

                        if let Some(entry_type) = entry.type_() {
                            if entry_type == "Application" && !entry.no_display() {}
                        }

                        if entry.type_().unwrap() == "Application" && !entry.no_display() {
                            let icon = entry.icon().unwrap().to_string();

                            apps.push(App {
                                icon_path: get_app_icon(icon)
                                    .unwrap_or(Path::new("").to_owned())
                                    .into_os_string()
                                    .to_str()
                                    .unwrap()
                                    .to_owned(),
                                exec_path: path.clone().into_os_string().into_string().unwrap(),
                                name: entry.name(None).unwrap().to_string(),
                            });
                        }
                    }
                }
            }
        }

        //Saves the apps in a temporary file
        let index_path = &get_apps_index_path().unwrap();
        let temp_folder = &get_temp_directory().unwrap();

        if !Path::new(temp_folder).exists() {
            fs::create_dir_all(temp_folder).unwrap();
        }

        let index_yml = serde_yaml::to_string(&apps).expect("Error writing apps file");

        let mut index_file = File::create(index_path).expect("Error creating index file");

        index_file
            .write_all(index_yml.as_bytes())
            .expect("Error writing apps to index");

        index_file.flush().expect("Error closing index file");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {

        let mut script_path = get_local_dir().unwrap();
        script_path.push("resources\\ps-scripts\\index-apps.ps1");

        let script_content = fs::read_to_string(&script_path).unwrap();
        powershell_script::run(&script_content).expect("Error running index script");
    }
}


#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("sync_apps", "Sync Apps"))
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("reload", "Reload"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    thread::spawn(move||{
                        if cfg!(target_os = "linux"){
                            Command::new("sh")
                                .arg("-c")
                                .arg("WEBKIT_DISABLE_COMPOSITING_MODE=1 simple-keyboard-launcher")
                                .output()
                                .expect("Error opening app");
                        }

                        if cfg!(target_os="windows"){

                            Command::new("cmd")
                                .arg("/C")
                                .arg("start simple-keyboard-launcher")
                                .current_dir(Path::new("C:\\Users\\lighttigerxiv\\Documents\\Development\\Simple-Keyboard-Launcher\\simple-kl\\src-tauri\\target\\release"))
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
                        if cfg!(target_os = "linux"){
                            Command::new("sh")
                                .arg("-c")
                                .arg("WEBKIT_DISABLE_COMPOSITING_MODE=1 simple-keyboard-launcher --settings")
                                .output()
                                .expect("Error opening app");
                        }

                        #[cfg(target_os = "windows")]
                        if cfg!(target_os="windows"){

                            Command::new("cmd")
                                .arg("/C")
                                .arg("start simple-keyboard-launcher --settings")
                                .current_dir(Path::new("C:\\Users\\lighttigerxiv\\Documents\\Development\\Simple-Keyboard-Launcher\\simple-kl\\src-tauri\\target\\release"))
                                .creation_flags(FLAG_NO_WINDOW)
                                .spawn()
                                .expect("Error opening app");
                        }  
                    });
                }
                "reload" => {

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

            init_settings();
            init_extensions();
            settings::update_auto_start();

            //Re-indexes the apps every 2 minutes
            tokio::spawn(async move {
                loop {
                    {
                        index_apps();
                    }
                    sleep(tokio::time::Duration::from_secs(120)).await;
                }
            });


            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            #[derive(Clone, serde::Serialize)]
            struct PluginPayload {
                args: Vec<String>,
                cwd: String,
            }

            app.emit_all("single-instance", PluginPayload { args: argv, cwd }).unwrap();
        }))
        .build(tauri::generate_context!())
        .expect("")
        .run(|app, e| match e {
            RunEvent::Ready => {

                let general_settings = get_settings().general;
                let first_key = general_settings.first_key;
                let second_key = general_settings.second_key;
                let third_key = general_settings.third_key;

                let shortcut = match second_key == "-" {
                    true=> format!("{}+{}", &first_key, &third_key),
                    false=> format!("{}+{}+{}", &first_key, &second_key, &third_key)
                };

                app.clone()
                    .global_shortcut_manager()
                    .register(&shortcut, move || {

                        thread::spawn(move||{
                            if cfg!(target_os = "linux"){
                                Command::new("sh")
                                    .arg("-c")
                                    .arg("WEBKIT_DISABLE_COMPOSITING_MODE=1 simple-keyboard-launcher")
                                    .output()
                                    .expect("Error opening app");
                            }

                            #[cfg(target_os = "windows")]
                            if cfg!(target_os="windows"){

                                Command::new("cmd")
                                    .arg("/C")
                                    .arg("start simple-keyboard-launcher")
                                    .current_dir(Path::new("C:\\Users\\lighttigerxiv\\Documents\\Development\\Simple-Keyboard-Launcher\\simple-kl\\src-tauri\\target\\release"))
                                    .creation_flags(FLAG_NO_WINDOW)
                                    .spawn()
                                    .expect("Error opening app");
                            }  
                        });
                    }).unwrap();
            }
            _ => {}
        })
}
