// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use extensions::CommunityExtension;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use git2::Repository;
use simple_kl_rs::{
    actions::{ExtensionAction, OpenApp, OpenInBrowser, ResultAction},
    extensions::{get_extensions, init_extensions, ExtensionManifest, Parameters},
    paths::{
        get_apps_index_path, get_community_extensions_file_path, get_community_extensions_path,
        get_community_themes_file_path, get_community_themes_path, get_extension_parameters_path,
        get_extension_path, get_extension_results_path, get_extensions_path, get_resources_path,
        get_temp_themes_path,
    },
    results::{IconWithTextResult, SimpleKLResult},
    settings::{Settings, Theme},
};
use structs::structs::AppIndex;
use themes::CommunityTheme;

use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent};
pub mod extensions;
pub mod structs;
pub mod themes;

#[tauri::command(rename_all = "snake_case")]
fn get_results(search_text: String) -> Result<String, String> {
    let split_search: Vec<&str> = search_text.split_whitespace().collect();
    let mut keyword = String::from("");
    let mut search_words = String::from("");
    let search_engines = Settings::current_settings().search_engines;

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

    for search_engine in search_engines {
        if search_engine.keyword == keyword {
            let url = search_engine.query.replace("%s", &search_words);
            let mut results: Vec<SimpleKLResult> = Vec::new();

            results.push(SimpleKLResult::IconWithText(
                match search_engine.tint_icon {
                    true => IconWithTextResult::new_with_color(
                        search_engine
                            .icon
                            .unwrap_or(format!("{}/images/search.svg", get_resources_path())),
                        "accent".to_string(),
                        format!("Search for {}", search_words),
                        ResultAction::OpenInBrowser(OpenInBrowser { url }),
                    ),
                    false => IconWithTextResult::new(
                        search_engine
                            .icon
                            .unwrap_or(format!("{}/images/search.svg", get_resources_path())),
                        format!("Search for {}", search_words),
                        ResultAction::OpenInBrowser(OpenInBrowser { url }),
                    ),
                },
            ));

            return Ok(serde_json::to_string(&results).unwrap());
        }
    }

    //Returns app results or search if it's empty
    let app_results = &get_apps_results(&search_text);
    return match app_results.len() > 0 {
        true => Ok(serde_json::to_string(&app_results).unwrap()),
        false => {
            for search_engine in Settings::current_settings().search_engines {
                if search_engine.default {
                    let url = search_engine.query.replace("%s", &search_text);
                    let mut results: Vec<SimpleKLResult> = Vec::new();

                    results.push(SimpleKLResult::IconWithText(
                        match search_engine.tint_icon {
                            true => IconWithTextResult::new_with_color(
                                search_engine.icon.unwrap_or(format!(
                                    "{}/images/search.svg",
                                    get_resources_path()
                                )),
                                "accent".to_string(),
                                format!("Search for {}", search_text),
                                ResultAction::OpenInBrowser(OpenInBrowser { url }),
                            ),
                            false => IconWithTextResult::new(
                                search_engine.icon.unwrap_or(format!(
                                    "{}/images/search.svg",
                                    get_resources_path()
                                )),
                                format!("Search for {}", search_text),
                                ResultAction::OpenInBrowser(OpenInBrowser { url }),
                            ),
                        },
                    ));

                    return Ok(serde_json::to_string(&results).unwrap());
                }
            }

            Ok("".into())
        }
    };
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

            results.push(SimpleKLResult::IconWithText(IconWithTextResult::new(
                app.icon_path,
                app.name,
                ResultAction::OpenApp(action),
            )));
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

                            //println!("{:?}", results);

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

#[tauri::command()]
fn close_window(window: tauri::Window) {
    window.close().unwrap();
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

            let action: OpenApp =
                serde_json::from_str(&action_json).expect("Error getting action from JSON");

            let desktop_file_dir = Path::new(&action.desktop_path)
                .parent()
                .expect("Error reading parent directory")
                .to_owned();

            let desktop_file_name = Path::new(&action.desktop_path)
                .file_name()
                .expect("")
                .to_owned();

            tokio::spawn(async move {
                Command::new("gtk-launch")
                    .arg(desktop_file_name)
                    .current_dir(desktop_file_dir)
                    .spawn()
                    .expect("");
            });

            return Ok(());
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

#[tauri::command(rename_all = "snake_case")]
fn update_extension_keyword(extension_id: String, keyword: String) -> Result<(), String> {
    return Settings::update_extension_keyword(extension_id, keyword);
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
                    manifest_file
                        .read_to_string(&mut manifest_json)
                        .expect("Error writing content to string");
                    let _ = manifest_file.flush();
                    let manifest: ExtensionManifest =
                        serde_json::from_str(&manifest_json).expect("Error converting manifest");

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

#[tauri::command(rename_all = "snake_case")]
fn add_search_engine(
    keyword: String,
    icon_path: String,
    tint_icon: bool,
    name: String,
    query: String,
) {
    let mut settings = Settings::current_settings();
    let icon: Option<String> = match icon_path.is_empty() {
        true => None,
        false => Some(icon_path),
    };

    settings
        .search_engines
        .push(simple_kl_rs::settings::SearchEngine {
            keyword,
            icon,
            tint_icon,
            name,
            query,
            default: false,
        });

    let settings_json = serde_json::to_string(&settings).expect("Error converting search engine");
    Settings::update(settings_json).expect("Error updating settings");
}

#[tauri::command()]
fn export_theme(path: String) {
    let mut file = File::create(&path).expect("Error creating theme file");
    let themes = Settings::current_settings().theme;
    let themes_json = serde_json::to_string(&themes).expect("Error converting theme");

    file.write_all(&themes_json.as_bytes())
        .expect("Error saving theme");
}

#[tauri::command()]
fn import_theme(path: String) {
    let mut file = File::open(&path).expect("Error opening theme file");
    let mut file_content = "".to_string();
    let mut settings = Settings::current_settings();

    file.read_to_string(&mut file_content)
        .expect("Error reading theme file");

    let theme: Theme = serde_json::from_str(&file_content).expect("Error converting to a theme");

    settings.theme = theme;

    Settings::update(serde_json::to_string(&settings).expect("Error converting settings"))
        .expect("Error updating settings");
}

#[tauri::command()]
fn debug_message(message: String) {
    println!("{}", message);
}

#[tauri::command()]
async fn import_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let repo_name = url_split[url_split.len() - 1];
    let path = format!("{}/{}", get_extensions_path(), repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");

    simple_kl_rs::extensions::init_extensions();
}

#[tauri::command()]
async fn delete_extension(id: String) {
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
                        fs::remove_dir_all(&folder_path).expect("Error deleting extension folder");
                    }

                    init_extensions();

                    manifest_file.flush().unwrap();
                }
            }
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_extension_default_keyword(extension_id: String) -> Result<String, ()> {
    let extensions = get_extensions();

    return match extensions.iter().find(|e| e.id == extension_id) {
        Some(extension) => Ok(extension.keyword.clone()),
        None => Err(()),
    };
}

#[tauri::command(rename_all = "snake_case")]
fn get_extension_default_setting(setting_id: String, extension_id: String) -> Result<String, ()> {
    let extensions = get_extensions();

    for extension in extensions {
        if extension.id == extension_id {
            for setting in extension.settings.any {
                if setting.id == setting_id {
                    return Ok(setting.default_value);
                }
            }
            for setting in extension.settings.linux {
                if setting.id == setting_id {
                    return Ok(setting.default_value);
                }
            }
            for setting in extension.settings.windows {
                if setting.id == setting_id {
                    return Ok(setting.default_value);
                }
            }
        }
    }

    return Err(());
}

#[tauri::command()]
async fn get_community_themes() -> Result<Vec<CommunityTheme>, ()> {
    let themes_path = get_community_themes_path();

    if Path::new(&themes_path).exists() {
        fs::remove_dir_all(&themes_path).expect("Error deleting themes directory");
    }

    fs::create_dir_all(&themes_path).expect("Error creating themes directory");

    Repository::clone(
        "https://github.com/lighttigerXIV/simple-kl-themes-hub",
        &themes_path,
    )
    .expect("Error cloning themes repo");

    let mut themes_file =
        File::open(get_community_themes_file_path()).expect("Error opening themes file");

    let mut themes_file_content = "".to_string();

    themes_file
        .read_to_string(&mut themes_file_content)
        .expect("Error reading themes file");

    let themes = serde_json::from_str(&themes_file_content);

    return match themes {
        Ok(themes) => Ok(themes),
        Err(_) => Err(()),
    };
}

#[tauri::command()]
async fn apply_community_theme(repo: String, file: String, app: AppHandle) {
    if Path::new(&get_temp_themes_path()).exists() {
        fs::remove_dir_all(&get_temp_themes_path()).expect("Error deleting temp themes directory");
    }

    fs::create_dir(&get_temp_themes_path()).expect("Error creating themes directory");

    Repository::clone(&repo, &get_temp_themes_path()).expect("Error cloning theme repo");

    let theme_file_path = format!("{}/{}", get_temp_themes_path(), file);
    let mut theme_file_content = "".to_string();

    let mut theme_file = File::open(&theme_file_path).expect("Error opening theme file");
    theme_file
        .read_to_string(&mut theme_file_content)
        .expect("Error reading theme content");

    let theme: Theme =
        serde_json::from_str(&theme_file_content).expect("Error getting theme from file");

    let mut settings = Settings::current_settings();
    settings.theme.background = theme.background;
    settings.theme.secondary_background = theme.secondary_background;
    settings.theme.tertiary_background = theme.tertiary_background;
    settings.theme.accent = theme.accent;
    settings.theme.on_accent = theme.on_accent;
    settings.theme.danger = theme.danger;
    settings.theme.on_danger = theme.on_danger;
    settings.theme.text = theme.text;
    settings.theme.secondary_text = theme.secondary_text;

    let settings_json = serde_json::to_string(&settings).expect("Error converting theme to json");
    Settings::update(settings_json).expect("Error updating settings");

    app.emit_all("updateTheme", ()).expect("Error running emit");
}

#[tauri::command()]
async fn get_community_extensions() -> Result<Vec<CommunityExtension>, ()> {
    let extensions_path = get_community_extensions_path();

    if Path::new(&extensions_path).exists() {
        fs::remove_dir_all(&extensions_path).expect("Error deleting extensions directory");
    }

    fs::create_dir_all(&extensions_path).expect("Error creating extensions directory");

    Repository::clone(
        "https://github.com/lighttigerXIV/simple-kl-extensions-hub",
        &extensions_path,
    )
    .expect("Error cloning extensions repo");

    let mut extensions_file =
        File::open(get_community_extensions_file_path()).expect("Error opening extensions file");

    let mut extensions_file_content = "".to_string();

    extensions_file
        .read_to_string(&mut extensions_file_content)
        .expect("Error reading extensions file");

    let extensions = serde_json::from_str(&extensions_file_content);

    return match extensions {
        Ok(extensions) => Ok(extensions),
        Err(_) => Err(()),
    };
}

#[tauri::command(rename_all = "snake_case")]
async fn install_community_extension(id: String, repo: String, app: AppHandle) {
    let path = format!("{}/{}", get_extensions_path(), id);

    Repository::clone(&repo, &path).expect("Error cloning repo");

    init_extensions();

    app.emit_all("updateExtensions", ()).expect("Error emiting");
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_results,
            get_current_settings,
            update_settings,
            close_search_window,
            show_window,
            close_window,
            run_action,
            get_extensions_json,
            get_os,
            update_extension_setting,
            update_extension_keyword,
            add_search_engine,
            export_theme,
            import_theme,
            debug_message,
            import_extension,
            delete_extension,
            get_extension_default_keyword,
            get_extension_default_setting,
            get_community_themes,
            apply_community_theme,
            get_community_extensions,
            install_community_extension
        ])
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments.iter().any(|e| e == "--settings");

            if open_settings {
                tauri::WindowBuilder::new(
                    app,
                    "settings",
                    tauri::WindowUrl::App("settings".into()),
                )
                .build()
                .expect("Error creating settings window");

                let main_window = app.get_window("main").unwrap();
                main_window.close().expect("Error closing search window");

                return Ok(());
            }

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
        /*
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            #[derive(Clone, serde::Serialize)]
            struct PluginPayload {
                args: Vec<String>,
                cwd: String,
            }
            app.emit_all("single-instance", PluginPayload { args: argv, cwd })
                .unwrap();
        }))
        */
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
                                window.close().unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
}
