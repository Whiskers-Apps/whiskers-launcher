// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//Imports only used in windows
#[cfg(target_os = "windows")]
use{
    simple_kl_rs::others::FLAG_NO_WINDOW,
    simple_kl_rs::paths::get_local_dir,
    std::os::windows::process::CommandExt,
};

//Imports only used in Linux
#[cfg(target_os = "linux")]
use{
    structs::structs::AppIndex
};

use enigo::MouseControllable;
use extensions::CommunityExtension;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use git2::Repository;
use simple_kl_rs::{
    actions::{ExtensionAction, OpenApp, OpenInBrowser, ResultAction},
    extensions::{get_extensions, init_extensions, ExtensionManifest, Parameters},
    paths::{
        get_apps_index_path, get_community_extensions_directory,
        get_community_extensions_file_path, get_community_themes_file_path,
        get_community_themes_path, get_extension_parameters_path, get_extension_path,
        get_extension_results_path, get_extensions_path, get_resources_directory,
        get_temp_themes_path,
    },
    results::{IconWithTextResult, SimpleKLResult},
    settings,
    settings::Settings,
};

use themes::CommunityTheme;

use simple_kl_rs::actions::{DialogAction, DialogResult};

use simple_kl_rs::paths::{get_autostart_path, get_dialog_action_path};
use simple_kl_rs::settings::{get_settings, init_settings, ThemeSettings};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, RunEvent, Window, WindowBuilder,
    WindowEvent, WindowUrl,
};

pub mod extensions;
pub mod structs;
pub mod themes;

pub mod pages;

#[tauri::command(rename_all = "snake_case")]
fn get_results(search_text: String) -> Vec<SimpleKLResult> {
    let split_search: Vec<&str> = search_text.split_whitespace().collect();
    let mut keyword = String::from("");
    let mut search_words = String::from("");
    let search_engines = get_settings().search_engines;

    for (index, word) in split_search.iter().enumerate() {
        if index == 0 {
            keyword = word.to_string();
        } else {
            search_words = search_words + word + " "
        }
    }

    search_words = String::from(search_words.trim_end());

    let settings = get_settings();
    let extensions = settings.extensions;

    for extension in extensions {
        if extension.keyword == keyword {
            return get_extension_results(extension.id, search_words);
        }
    }

    let mut search_svg_path = get_resources_directory().unwrap();
    search_svg_path.push("images");
    search_svg_path.push("search.svg");

    for search_engine in search_engines {
        if search_engine.keyword == keyword {
            let url = search_engine.query.replace("%s", &search_words);
            let mut result: Vec<SimpleKLResult> = Vec::new();

            result.push(SimpleKLResult::IconWithText(
                match search_engine.tint_icon {
                    true => {
                        IconWithTextResult::new_with_color(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                            .to_owned(),
                            format!("Search for {}", search_words).as_str(),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        )
                    }
                    false => {
                        IconWithTextResult::new(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                            .to_owned(),
                            &format!("Search for {}", search_words),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        )
                    }
                },
            ));

            return result;
        }
    }

    let app_results = get_apps_results(&search_text);

    return if &app_results.len() > &0 {
        app_results
    } else {
        //Returns a search result
        for search_engine in get_settings().search_engines {
            if search_engine.default {
                let url = search_engine.query.replace("%s", &search_text);
                let mut result: Vec<SimpleKLResult> = Vec::new();

                result.push(SimpleKLResult::IconWithText(
                    match search_engine.tint_icon {
                        true => IconWithTextResult::new_with_color(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                            .to_path_buf(),
                            format!("Search for {}", search_text).as_str(),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        ),
                        false => IconWithTextResult::new(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                            .to_owned(),
                            &format!("Search for {}", search_text),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        ),
                    },
                ));

                return result;
            }
        }

        vec![]
    };
}

fn get_apps_results(search_text: &str) -> Vec<SimpleKLResult> {
    let mut results: Vec<SimpleKLResult> = Vec::new();

    let indexed_apps_yaml = fs::read_to_string(get_apps_index_path().unwrap()).unwrap();
    let apps: Vec<AppIndex> = serde_yaml::from_str(&indexed_apps_yaml).unwrap();

    for app in apps {
        if SkimMatcherV2::default()
            .fuzzy_match(&app.name, search_text)
            .is_some()
        {
            let action = OpenApp::new(app.exec_path);

            results.push(SimpleKLResult::IconWithText(IconWithTextResult::new(
                Path::new(&app.icon_path).to_owned(),
                &app.name,
                ResultAction::OpenApp(action),
            )));
        }
    }

    return results;
}

fn get_extension_results(id: String, search_text: String) -> Vec<SimpleKLResult> {
    let mut parameters_file = File::create(&get_extension_parameters_path().unwrap()).unwrap();
    let parameters = Parameters::new_get_results(search_text);
    let parameters_yaml =
        serde_yaml::to_string(&parameters).expect("Error converting parameters yaml");

    parameters_file
        .write_all(&parameters_yaml.as_bytes())
        .unwrap();

    parameters_file.flush().unwrap();

    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_yaml = String::from("");
                    manifest_file.read_to_string(&mut manifest_yaml).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_yaml).unwrap();

                    if manifest.id == id {
                        if cfg!(target_os = "linux") {
                            let extension_run = Command::new("sh")
                                .arg("-c")
                                .arg("./extension")
                                .current_dir(&folder_path)
                                .output()
                                .expect("Error running extension");

                            if extension_run.status.success() {
                                let mut extension_results_yaml = String::from("");
                                let mut extensions_results_file =
                                    File::open(&get_extension_results_path().unwrap()).unwrap();

                                extensions_results_file
                                    .read_to_string(&mut extension_results_yaml)
                                    .unwrap();

                                let results: Vec<SimpleKLResult> =
                                    serde_yaml::from_str(&extension_results_yaml).unwrap();

                                return results;
                            } else {
                                println!(
                                    "Error running extension: {}",
                                    String::from_utf8_lossy(&extension_run.stderr)
                                )
                            }
                        }

                        #[cfg(target_os = "windows")]
                        if cfg!(target_os = "windows") {
                            let extension_run = Command::new("cmd")
                                .arg("/C")
                                .arg("start extension.exe")
                                .current_dir(&folder_path)
                                .creation_flags(FLAG_NO_WINDOW)
                                .output()
                                .expect("Error running extension");

                            if extension_run.status.success() {
                                let mut extension_results_yaml = String::from("");
                                let mut extensions_results_file =
                                    File::open(&get_extension_results_path().unwrap()).unwrap();

                                extensions_results_file
                                    .read_to_string(&mut extension_results_yaml)
                                    .unwrap();

                                let results: Vec<SimpleKLResult> =
                                    serde_yaml::from_str(&extension_results_yaml).unwrap();

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
    }

    return Vec::new();
}



#[tauri::command]
fn get_current_settings() -> String {
    init_settings();
    return serde_json::to_string(&get_settings()).unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn update_settings(settings_json: String) {
    init_settings();

    let settings: Settings = serde_json::from_str(&settings_json).unwrap();

    settings::update_settings(&settings);
}

#[tauri::command()]
fn close_search_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command()]
fn show_window(window: Window) {
    window.show().unwrap();
}

#[tauri::command()]
fn close_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
async fn run_action(
    action_type: String,
    action_json: String,
    window: Window,
    app_handle: AppHandle,
) {
    match action_type.as_str() {
        "OpenApp" => {
            window.hide().unwrap();

            let action: OpenApp =
                serde_json::from_str(&action_json).expect("Error getting action from JSON");

            #[cfg(target_os = "linux")]
            if cfg!(target_os = "linux") {
                let desktop_file_dir = Path::new(&action.desktop_path)
                    .parent()
                    .expect("Error reading parent directory")
                    .to_owned();

                let desktop_file_name = Path::new(&action.desktop_path)
                    .file_name()
                    .expect("")
                    .to_owned();

                std::thread::spawn(|| {
                    Command::new("gtk-launch")
                        .arg(desktop_file_name)
                        .current_dir(desktop_file_dir)
                        .spawn()
                        .expect("");
                });
            }

            #[cfg(target_os = "windows")]
            if cfg!(target_os = "windows") {
                let path = Path::new(&action.desktop_path).to_owned();

                std::thread::spawn(move || {
                    let script = format!(
                        "invoke-item '{}'",
                        path.into_os_string().into_string().unwrap()
                    );

                    powershell_script::run(&script).expect("Error opening app");
                });
            }
        }
        "OpenInBrowser" => {
            let action: OpenInBrowser = serde_json::from_str(&action_json).unwrap();
            open::that(action.url).expect("Error opening url");
        }
        "CopyToClipboard" => {}
        "ExtensionAction" => {
            let action: ExtensionAction = serde_json::from_str(&action_json).unwrap();

            let parameters =
                Parameters::new_action(action.action, action.args.unwrap_or(Vec::new()));

            let mut parameters_file = File::create(&get_extension_parameters_path().unwrap())
                .expect("Error opening parameters file");

            let parameters_yaml =
                serde_yaml::to_string(&parameters).expect("Error converting parameters yaml");

            parameters_file
                .write_all(&parameters_yaml.as_bytes())
                .unwrap();

            parameters_file.flush().unwrap();

            if cfg!(target_os = "linux") {
                Command::new("sh")
                    .arg("-c")
                    .arg("./extension")
                    .current_dir(get_extension_path(&action.extension_id).unwrap())
                    .output()
                    .expect("Error running extension action");
            }

            #[cfg(target_os = "windows")]
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("start extension.exe")
                    .current_dir(get_extension_path(&action.extension_id).unwrap())
                    .creation_flags(FLAG_NO_WINDOW)
                    .output()
                    .expect("Error running extension action");
            }
        }
        "DialogAction" => {
            //Writes a file with all the fields for the dialog
            let action: DialogAction =
                serde_json::from_str(&action_json).expect("Error getting action from JSON");

            let action_yaml =
                serde_yaml::to_string(&action).expect("Error creating dialog action yaml");

            let mut dialog_action_file = File::create(get_dialog_action_path().unwrap())
                .expect("Error creating dialog action file");

            dialog_action_file
                .write_all(action_yaml.as_bytes())
                .expect("Error writing dialog action file");

            dialog_action_file
                .flush()
                .expect("Error closing dialog action file");

            //Opens the dialog window
            WindowBuilder::new(
                &app_handle,
                "extension_dialog",
                WindowUrl::App("extension-dialog".parse().unwrap()),
            )
            .max_inner_size(300.0, 800.0)
            .resizable(false)
            .title(action.title)
            .build()
            .expect("Error spawning extension dialog");
        }
        _ => {}
    };

    window.close().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn update_extension_setting(extension_id: String, setting_id: String, new_value: String) {
    simple_kl_rs::extensions::update_extension_setting(&extension_id, &setting_id, &new_value);
}

#[tauri::command(rename_all = "snake_case")]
fn update_extension_keyword(extension_id: String, keyword: String) {
    simple_kl_rs::extensions::update_extension_keyword(&extension_id, &keyword);
}

#[tauri::command()]
fn get_extensions_json() -> String {
    let mut extensions: Vec<ExtensionManifest> = Vec::new();

    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file
                        .read_to_string(&mut manifest_json)
                        .expect("Error writing content to string");
                    let _ = manifest_file.flush();
                    let manifest: ExtensionManifest =
                        serde_yaml::from_str(&manifest_json).expect("Error converting manifest");

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
    let mut settings = get_settings();
    let icon: Option<String> = match icon_path.is_empty() {
        true => None,
        false => Some(icon_path),
    };

    settings
        .search_engines
        .push(settings::SearchEngineSettings {
            keyword,
            icon: if icon.is_some() {
                Some(icon.unwrap())
            } else {
                None
            },
            tint_icon,
            name,
            query,
            default: false,
        });

    settings::update_settings(&settings);
}

#[tauri::command()]
fn export_theme(path: String) {
    let mut file = File::create(&path).expect("Error creating theme file");
    let themes = get_settings().theme;
    let themes_json = serde_yaml::to_string(&themes).expect("Error converting theme");

    file.write_all(&themes_json.as_bytes())
        .expect("Error saving theme");
}

#[tauri::command()]
fn import_theme(path: String) {
    let file_content = fs::read_to_string(&path).expect("Error reading theme file");
    let mut settings = get_settings();

    let theme: ThemeSettings =
        serde_yaml::from_str(&file_content).expect("Error converting file to a theme");

    settings.theme = theme;

    settings::update_settings(&settings);
}

#[tauri::command()]
fn debug_message(message: String) {
    println!("{}", message);
}

#[tauri::command()]
async fn import_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let repo_name = url_split[url_split.len() - 1];
    let mut path = get_extensions_path().unwrap();
    path.push(repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");

    init_extensions();
}

#[tauri::command()]
async fn delete_extension(id: String) {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_json).unwrap();

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
            if let Some(settings) = extension.settings {
                for setting in settings.any {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }

                for setting in settings.linux {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }

                for setting in settings.windows {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }
            }
        }
    }

    return Err(());
}

#[tauri::command()]
async fn get_community_themes() -> Result<Vec<CommunityTheme>, ()> {
    let themes_path = get_community_themes_path().unwrap();

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
        File::open(get_community_themes_file_path().unwrap()).expect("Error opening themes file");

    let mut themes_file_content = "".to_string();

    themes_file
        .read_to_string(&mut themes_file_content)
        .expect("Error reading themes file");

    let themes = serde_yaml::from_str(&themes_file_content);

    return match themes {
        Ok(themes) => Ok(themes),
        Err(_) => Err(()),
    };
}

#[tauri::command()]
async fn apply_community_theme(repo: String, file: String, app: AppHandle) {
    if Path::new(&get_temp_themes_path().unwrap()).exists() {
        fs::remove_dir_all(&get_temp_themes_path().unwrap())
            .expect("Error deleting temp themes directory");
    }

    fs::create_dir(&get_temp_themes_path().unwrap()).expect("Error creating themes directory");

    Repository::clone(&repo, &get_temp_themes_path().unwrap()).expect("Error cloning theme repo");

    let mut theme_file_path = get_temp_themes_path().unwrap();
    theme_file_path.push(file);

    let mut theme_file_content = "".to_string();

    let mut theme_file = File::open(&theme_file_path).expect("Error opening theme file");
    theme_file
        .read_to_string(&mut theme_file_content)
        .expect("Error reading theme content");

    let theme: ThemeSettings =
        serde_yaml::from_str(&theme_file_content).expect("Error getting theme from file");

    let mut settings = get_settings();
    settings.theme.background = theme.background;
    settings.theme.secondary_background = theme.secondary_background;
    settings.theme.tertiary_background = theme.tertiary_background;
    settings.theme.accent = theme.accent;
    settings.theme.on_accent = theme.on_accent;
    settings.theme.danger = theme.danger;
    settings.theme.on_danger = theme.on_danger;
    settings.theme.text = theme.text;
    settings.theme.secondary_text = theme.secondary_text;

    settings::update_settings(&settings);

    app.emit_all("updateTheme", ()).expect("Error running emit");
}

#[tauri::command()]
async fn get_community_extensions() -> Result<Vec<CommunityExtension>, ()> {
    let extensions_dir = get_community_extensions_directory().unwrap();

    if Path::new(&extensions_dir).exists() {
        fs::remove_dir_all(&extensions_dir).expect("Error deleting extensions directory");
    }

    fs::create_dir_all(&extensions_dir).expect("Error creating extensions directory");

    Repository::clone(
        "https://github.com/lighttigerXIV/simple-kl-extensions-hub",
        &extensions_dir,
    )
    .expect("Error cloning extensions repo");

    let mut extensions_file = File::open(get_community_extensions_file_path().unwrap())
        .expect("Error opening extensions file");

    let mut extensions_file_content = "".to_string();

    extensions_file
        .read_to_string(&mut extensions_file_content)
        .expect("Error reading extensions file");

    let extensions = serde_yaml::from_str(&extensions_file_content);

    return match extensions {
        Ok(extensions) => Ok(extensions),
        Err(_) => Err(()),
    };
}

#[tauri::command(rename_all = "snake_case")]
async fn install_community_extension(id: String, repo: String, app: AppHandle) {
    let mut path = get_extensions_path().unwrap();
    path.push(id);

    Repository::clone(&repo, &path).expect("Error cloning repo");

    init_extensions();

    app.emit_all("updateExtensions", ()).expect("Error calling listener");
}

#[tauri::command]
fn get_dialog_action() -> Result<DialogAction, ()> {
    let dialog_actions_yaml = fs::read_to_string(&get_dialog_action_path().unwrap())
        .expect("Error reading dialog fields file");

    let action: DialogAction = serde_yaml::from_str(&dialog_actions_yaml)
        .expect("Error getting yaml from dialog fields file");

    return Ok(action);
}

#[tauri::command]
fn write_dialog_result(result: DialogResult, window: Window) {
    let fields = serde_yaml::to_string(&result.results).unwrap();

    let parameters = Parameters::new_action(result.action, vec![fields]);

    let parameters_yaml = serde_yaml::to_string(&parameters).unwrap();

    let mut parameters_file = File::create(get_extension_parameters_path().unwrap())
        .expect("Error creating parameters file");

    parameters_file
        .write_all(&parameters_yaml.as_bytes())
        .unwrap();

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./extension")
            .current_dir(get_extension_path(&result.extension_id).unwrap())
            .output()
            .expect("Error running extension action");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("start extension.exe")
            .current_dir(get_extension_path(&result.extension_id).unwrap())
            .creation_flags(FLAG_NO_WINDOW)
            .output()
            .expect("Error running extension action");
    }

    window.close().expect("Error closing window");
}

#[tauri::command]
fn open_settings(app: AppHandle) {
    WindowBuilder::new(&app, "settings", WindowUrl::App("settings".into()))
        .title("Settings")
        .build()
        .expect("Error creating settings window");

    let main_window = &app.get_window("main").unwrap();
    main_window.close().expect("Error closing search window");
}

#[tauri::command]
fn update_auto_start() {
    let path = get_autostart_path().unwrap();
    let settings = get_settings();
    let auto_start = settings.general.auto_start;

    if !path.exists() && auto_start {
        fs::create_dir_all(&path).expect("Error creating autostart folder");
    }

    match env::consts::OS {
        "linux" => {
            let desktop_file_content = include_str!("files/simple-kl-service.desktop");
            let mut desktop_file_path = path.to_owned();
            desktop_file_path.push("simple-kl-service.desktop");

            if auto_start {
                fs::write(&desktop_file_path, &desktop_file_content)
                    .expect("Error creating autostart file");
            } else {
                if desktop_file_path.exists() {
                    fs::remove_file(&desktop_file_path).expect("Error removing autostart file");
                }
            }
        }
        #[cfg(target_os = "windows")]
        "windows" => {

            let script = if auto_start {"enable-autostart.ps1"} else { "disable-autostart.ps1" };

            let mut path = get_local_dir().unwrap();
            path.push("scripts");
            path.push(script);

            Command::new("powershell")
                .arg("-File")
                .arg(&path.into_os_string().into_string().unwrap())
                .creation_flags(FLAG_NO_WINDOW)
                .output()
                .expect("Error running autostart script");
        }
        _ => {}
    }
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
            install_community_extension,
            get_dialog_action,
            write_dialog_result,
            open_settings,
            update_auto_start
        ])
        .setup(|app| {
            let arguments: Vec<String> = env::args().collect();
            let open_settings = arguments.iter().any(|e| e == "--settings");

            
            //Opens settings if the argument is found
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
        .unwrap()
        .run(|app, e| match e {
            RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    match event {
                        WindowEvent::Focused(focused) => {
                            //Hides the window when user clicks outside
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
