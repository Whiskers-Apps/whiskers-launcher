// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use constants::Settings;
use data_structs::{Action, OpenAppAction, SimpleKLResult};
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, PhysicalPosition, PhysicalSize, RunEvent,
    SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent,
};
pub mod constants;
pub mod data_structs;

#[tauri::command]
fn get_results() -> Result<String, String> {
    let mut results: Vec<SimpleKLResult> = Vec::new();

    let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let local_apps_path = &format!("{home_path}/.local/share/applications");
    let apps_path = "/usr/share/applications";

    let local_apps_entries = fs::read_dir(local_apps_path).unwrap();
    let apps_entries = fs::read_dir(apps_path).unwrap();

    for local_app_entry in local_apps_entries {
        let file_path = local_app_entry.unwrap().path();
        let file = File::open(&file_path).unwrap();
        let reader = io::BufReader::new(file);
        let mut app_name = String::from("");
        let mut app_icon = String::from("");

        for line in reader.lines() {
            let line_content = line.unwrap();

            if line_content.starts_with("Name=") {
                let split_line: Vec<&str> = line_content.splitn(2, "Name=").collect();
                app_name = split_line[1].to_string();
            }
            /*
            if line_content.starts_with("Exec="){
                let split_line: Vec<&str> = line_content.splitn(2, "Exec=").collect();

            }
            */
            if line_content.starts_with("Icon=") {
                let split_line: Vec<&str> = line_content.splitn(2, "Icon=").collect();
                app_icon = split_line[1].to_string();
            }
        }

        let action = OpenAppAction {
            exec_path: "".to_string(),
        };

        results.push(SimpleKLResult {
            icon: Some(app_icon),
            title: None,
            description: None,
            text: Some(app_name),
            action: Action::OpenApp(action),
        });
    }

    for app_entry in apps_entries {
        let file_path = app_entry.unwrap().path();
        let file = File::open(&file_path).unwrap();
        let reader = io::BufReader::new(file);
        let mut app_name = String::from("");
        let mut app_icon = String::from("");

        for line in reader.lines() {
            let line_content = line.unwrap();

            if line_content.starts_with("Name=") {
                let split_line: Vec<&str> = line_content.splitn(2, "Name=").collect();
                app_name = split_line[1].to_string();
            }
            /*
            if line_content.starts_with("Exec="){
                let split_line: Vec<&str> = line_content.splitn(2, "Exec=").collect();

            }
            */
            if line_content.starts_with("Icon=") {
                let split_line: Vec<&str> = line_content.splitn(2, "Icon=").collect();
                app_icon = split_line[1].to_string();
            }
        }

        let action = OpenAppAction {
            exec_path: "".to_string(),
        };

        /* 
        results.push(SimpleKLResult {
            icon: Some(app_icon),
            title: None,
            description: None,
            text: Some(app_name),
            action: Action::OpenApp(action),
        });
        */

        let mut result = SimpleKLResult::new();
    
    }

    return Ok( serde_json::to_string(&results).unwrap());
}

#[tauri::command]
fn get_current_settings() -> String {
    Settings::init_settings();
    return serde_json::to_string(&Settings::current_settings()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn update_settings(settings_json: String) -> Result<(), String> {
    Settings::init_settings();
    return Settings::update(settings_json);
}

#[tauri::command()]
fn hide_window(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command()]
fn show_window(window: tauri::Window) {
    window.hide().unwrap();
}

fn main() {
    //Init Tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle_visibility", "Hide/Show"))
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_results,
            get_current_settings,
            update_settings,
            hide_window,
            show_window
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let screen = main_window.current_monitor().unwrap().unwrap();
            let screen_position = screen.position();
            let screen_size = PhysicalSize::<i32> {
                width: screen.size().width as i32,
                height: screen.size().height as i32,
            };
            let window_size = PhysicalSize::<i32> {
                width: main_window.outer_size().unwrap().width as i32,
                height: main_window.outer_size().unwrap().height as i32,
            };

            let new_position = PhysicalPosition {
                x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
                y: screen_position.y + 100,
            };

            main_window.set_position(new_position).unwrap();
            main_window.set_always_on_top(true).unwrap();
            main_window.show().unwrap();

            /*
            if main_window.is_visible().unwrap() {
                main_window.hide().unwrap();
            } else {
                main_window.show().unwrap();
            }
            */

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle_visibility" => {
                    let main_window = app.get_window("main").unwrap();
                    let screen = main_window.current_monitor().unwrap().unwrap();
                    let screen_position = screen.position();
                    let screen_size = PhysicalSize::<i32> {
                        width: screen.size().width as i32,
                        height: screen.size().height as i32,
                    };
                    let window_size = PhysicalSize::<i32> {
                        width: main_window.outer_size().unwrap().width as i32,
                        height: main_window.outer_size().unwrap().height as i32,
                    };

                    let new_position = PhysicalPosition {
                        x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
                        y: screen_position.y + 100,
                    };

                    main_window.set_position(new_position).unwrap();
                    main_window.set_always_on_top(true).unwrap();

                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                        main_window.emit("focus_box", "").unwrap();
                    }
                }
                "settings" => {
                    tauri::WindowBuilder::new(
                        app,
                        "settings",
                        tauri::WindowUrl::App("/settings".parse().unwrap()),
                    )
                    .build()
                    .unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_positioner::init())
        .build(tauri::generate_context!())
        .expect("")
        .run(|app, e| match e {
            RunEvent::Ready => {
                let launch_app = app.clone();

                app.clone()
                    .global_shortcut_manager()
                    .register(&Settings::launch_shortcut(), move || {
                        let main_window = launch_app.get_window("main").unwrap();
                        let screen = main_window.current_monitor().unwrap().unwrap();
                        let screen_position = screen.position();
                        let screen_size = PhysicalSize::<i32> {
                            width: screen.size().width as i32,
                            height: screen.size().height as i32,
                        };
                        let window_size = PhysicalSize::<i32> {
                            width: main_window.outer_size().unwrap().width as i32,
                            height: main_window.outer_size().unwrap().height as i32,
                        };

                        let new_position = PhysicalPosition {
                            x: screen_position.x
                                + ((screen_size.width / 2) - (window_size.width / 2)),
                            y: screen_position.y + 100,
                        };

                        main_window.set_position(new_position).unwrap();
                        main_window.set_always_on_top(true).unwrap();
                        main_window.show().unwrap();
                        main_window.emit("focus_box", "").unwrap();
                    })
                    .unwrap();
            }
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::Focused(focused) => {
                            //Hides the window if the user clicks outside
                            if !focused {
                                let window = app.get_window("main").unwrap();
                                window.hide().unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
            RunEvent::Resumed => {}
            _ => {}
        });
}
