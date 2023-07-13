// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use simple_kl_rs::{
    actions::{ExtensionAction, OpenApp, OpenInBrowser, ResultAction},
    extensions::{ExtensionManifest, Parameters},
    paths::{
        get_apps_index_path, get_extension_parameters_path, get_extension_path,
        get_extension_results_path, get_extensions_path,
    },
    results::{IconWithTextResult, SimpleKLResult, TextResult},
    settings::Settings,
};
use structs::structs::AppIndex;

use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    process::Command,
};

use tauri::{Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent};
pub mod extensions;
pub mod structs;

#[tauri::command(rename_all = "snake_case")]
fn get_results(search_text: String) -> Result<String, String> {
    let split_search: Vec<&str> = search_text.split_whitespace().collect();
    let mut keyword = String::from("");
    let mut search_words = String::from("");
    let search_options = Settings::search_options();

    for (index, word) in split_search.iter().enumerate() {
        if index == 0 {
            keyword = word.to_string();
        } else {
            search_words = search_words + word + " "
        }
    }

    search_words = String::from(search_words.trim_end());

    let settings = Settings::current_settings();
    let extensions = settings.extensions;

    for extension in extensions {
        if extension.keyword == keyword {
            return Ok(
                serde_json::to_string(&get_extension_results(extension.id, search_words)).unwrap(),
            );
        }
    }

    for search_option in search_options {
        if search_option.keyword == keyword {
            let url = search_option.query.replace("%s", &search_words);

            let result: Vec<SimpleKLResult> = vec![SimpleKLResult::Text(TextResult {
                text: format!("Search in {} for {}", search_option.name, search_words),
                action: ResultAction::OpenInBrowser(OpenInBrowser { url }),
            })];

            return Ok(serde_json::to_string(&result).unwrap());
        }
    }

    return Ok(serde_json::to_string(&get_apps_results(&search_text)).unwrap());
}

fn get_apps_results(search_text: &str) -> Vec<SimpleKLResult> {
    let mut results: Vec<SimpleKLResult> = Vec::new();

    let indexed_apps_json = fs::read_to_string(get_apps_index_path()).unwrap();
    let apps: Vec<AppIndex> = serde_json::from_str(&indexed_apps_json).unwrap();

    for app in apps {
        if SkimMatcherV2::default()
            .fuzzy_match(&app.name, search_text)
            .is_some()
        {
            let action = OpenApp::new(app.desktop_path);

            results.push(SimpleKLResult::IconWithText(IconWithTextResult {
                icon: app.icon_path,
                text: app.name,
                action: ResultAction::OpenApp(action),
            }));
        }
    }

    return results;
}

fn get_extension_results(id: String, search_text: String) -> Vec<SimpleKLResult> {
    let mut parameters_file = File::create(&get_extension_parameters_path()).unwrap();
    let parameters = Parameters::new_get_results(search_text);
    let parameters_json =
        serde_json::to_string(&parameters).expect("Error converting parameters json");

    parameters_file
        .write_all(&parameters_json.as_bytes())
        .unwrap();
    parameters_file.flush().unwrap();

    if let Ok(folders) = fs::read_dir(&get_extensions_path()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.json", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();

                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    if manifest.id == id {
                        let extension_run = Command::new("sh")
                            .arg("-c")
                            .arg("./extension")
                            .current_dir(folder_path)
                            .output()
                            .expect("Error running extension");

                        if extension_run.status.success() {
                            let mut extension_results_json = String::from("");
                            let mut extensions_results_file =
                                File::open(&get_extension_results_path()).unwrap();
                            extensions_results_file
                                .read_to_string(&mut extension_results_json)
                                .unwrap();
                            let results: Vec<SimpleKLResult> =
                                serde_json::from_str(&extension_results_json).unwrap();

                            return results;
                        } else {
                            println!(
                                "Error running extension: {}",
                                String::from_utf8_lossy(&extension_run.stderr)
                            )
                        }
                    }
                }
            }
        }
    }

    return Vec::new();
}

#[tauri::command]
fn get_current_settings() -> String {
    Settings::init();
    return serde_json::to_string(&Settings::current_settings()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn update_settings(settings_json: String) -> Result<(), String> {
    Settings::init();
    return Settings::update(settings_json);
}

#[tauri::command()]
fn close_search_window(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command()]
fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
async fn run_action(
    action_type: String,
    action_json: String,
    window: tauri::Window,
) -> Result<(), String> {
    match action_type.as_str() {
        "OpenApp" => {
            window.hide().unwrap();

            let action: OpenApp = serde_json::from_str(&action_json).unwrap();
            let command = Command::new("gio")
                .arg("launch")
                .arg(action.desktop_path)
                .output()
                .expect("Error opening app");

            return match command.status.success() {
                true => Ok(()),
                false => Err("".into()),
            };
        }
        "OpenInBrowser" => {
            let action: OpenInBrowser = serde_json::from_str(&action_json).unwrap();
            open::that(action.url).expect("Error opening url");

            return Ok(());
        }
        "CopyToClipbard" => return Ok(()),
        "ExtensionAction" => {
            let action: ExtensionAction = serde_json::from_str(&action_json).unwrap();
            let parameters =
                Parameters::new_action(action.action, action.args.unwrap_or(Vec::new()));
            let mut parameters_file = File::create(&get_extension_parameters_path())
                .expect("Error opening parameters file");
            let parameters_json =
                serde_json::to_string(&parameters).expect("Error converting parameters json");

            parameters_file
                .write_all(&parameters_json.as_bytes())
                .unwrap();
            parameters_file.flush().unwrap();

            Command::new("sh")
                .arg("-c")
                .arg("./extension")
                .current_dir(get_extension_path(action.extension_id).unwrap())
                .output()
                .expect("Error running extension action");

            return Ok(());
        }
        _ => return Err("Action not found".into()),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn update_extension_setting(
    extension_id: String,
    setting_id: String,
    new_value: String,
) -> Result<(), String> {
    return Settings::update_extension_setting(extension_id, setting_id, new_value);
}

#[tauri::command()]
fn get_extensions_json() -> String {
    let mut extensions: Vec<ExtensionManifest> = Vec::new();

    if let Ok(folders) = fs::read_dir(&get_extensions_path()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.json", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();
                    let _ = manifest_file.flush();
                    let manifest: ExtensionManifest = serde_json::from_str(&manifest_json).unwrap();

                    extensions.push(manifest);
                }
            }
        }
    }

    return serde_json::to_string(&extensions).unwrap();
}

#[tauri::command()]
fn get_os() -> Result<String, ()> {
    return Ok(String::from(env::consts::OS));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_results,
            get_current_settings,
            update_settings,
            close_search_window,
            show_window,
            run_action,
            get_extensions_json,
            get_os,
            update_extension_setting
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
            main_window.set_focus().unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_positioner::init())
        .build(tauri::generate_context!())
        .expect("")
        .run(|app, e| match e {
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
            _ => {}
        });
}
