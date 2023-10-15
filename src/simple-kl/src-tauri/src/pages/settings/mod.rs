use crate::extensions::CommunityExtension;
use crate::structs::structs::AppIndex;
use crate::themes::CommunityTheme;
use git2::Repository;
use simple_kl_rs::extensions::init_extensions;
use simple_kl_rs::paths::{
    get_apps_index_path, get_autostart_path, get_community_extensions_directory,
    get_community_extensions_file_path, get_community_themes_file_path, get_community_themes_path,
    get_extensions_path, get_local_dir, get_temp_themes_path,
};
use simple_kl_rs::settings;
use simple_kl_rs::settings::{get_settings, init_settings, Settings, ThemeSettings};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl};

#[derive(serde::Serialize)]
pub struct WhiteListApp {
    icon: String,
    name: String,
    path: String,
    checked: bool,
}

#[cfg(target_os = "windows")]
use {simple_kl_rs::others::FLAG_NO_WINDOW, std::os::windows::process::CommandExt};

///Opens the settings page in a new window
#[tauri::command]
pub fn open_settings(app: AppHandle) {
    WindowBuilder::new(&app, "settings", WindowUrl::App("settings".into()))
        .title("Settings")
        .build()
        .expect("Error creating settings window");

    let main_window = &app.get_window("main").unwrap();
    main_window.close().expect("Error closing search window");
}

/// Returns the user settings as a json string
#[tauri::command]
pub fn get_current_settings() -> String {
    init_settings();
    return serde_json::to_string(&get_settings()).unwrap();
}

///
#[tauri::command(rename_all = "snake_case")]
pub fn update_settings(settings_json: String) {
    init_settings();

    let settings: Settings = serde_json::from_str(&settings_json).unwrap();

    settings::update_settings(&settings);
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_search_engine(
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
pub fn export_theme(path: String) {
    let mut file = File::create(&path).expect("Error creating theme file");
    let themes = get_settings().theme;
    let themes_json = serde_yaml::to_string(&themes).expect("Error converting theme");

    file.write_all(&themes_json.as_bytes())
        .expect("Error saving theme");
}

#[tauri::command()]
pub fn import_theme(path: String) {
    let file_content = fs::read_to_string(&path).expect("Error reading theme file");
    let mut settings = get_settings();

    let theme: ThemeSettings =
        serde_yaml::from_str(&file_content).expect("Error converting file to a theme");

    settings.theme = theme;

    settings::update_settings(&settings);
}

#[tauri::command()]
pub async fn get_community_themes() -> Result<Vec<CommunityTheme>, ()> {
    let themes_path = get_community_themes_path().unwrap();

    if Path::new(&themes_path).exists() {
        fs::remove_dir_all(&themes_path).expect("Error deleting themes directory");
    }

    fs::create_dir_all(&themes_path).expect("Error creating themes directory");

    Repository::clone(
        "https://github.com/lighttigerXIV/simple-kl-themes",
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
pub async fn apply_community_theme(repo: String, file: String, app: AppHandle) {
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
pub async fn get_community_extensions() -> Result<Vec<CommunityExtension>, ()> {
    let extensions_dir = get_community_extensions_directory().unwrap();

    if Path::new(&extensions_dir).exists() {
        fs::remove_dir_all(&extensions_dir).expect("Error deleting extensions directory");
    }

    fs::create_dir_all(&extensions_dir).expect("Error creating extensions directory");

    Repository::clone(
        "https://github.com/lighttigerXIV/simple-kl-extensions",
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
pub async fn install_community_extension(id: String, repo: String, app: AppHandle) {
    let mut path = get_extensions_path().unwrap();
    path.push(id);

    Repository::clone(&repo, &path).expect("Error cloning repo");

    init_extensions();

    app.emit_all("updateExtensions", ())
        .expect("Error calling listener");
}

#[tauri::command]
pub fn update_auto_start() {
    simple_kl_rs::settings::update_auto_start();
}

#[tauri::command]
pub fn get_whitelist_apps() -> Vec<WhiteListApp> {
    let mut apps: Vec<WhiteListApp> = Vec::new();
    let settings = get_settings();
    let blacklist = settings.results.blacklist;

    let indexed_apps_yaml = fs::read_to_string(get_apps_index_path().unwrap()).unwrap();
    let indexed_apps: Vec<AppIndex> = serde_yaml::from_str(&indexed_apps_yaml).unwrap();

    for app in indexed_apps {
        if !&blacklist.contains(&app.exec_path) {
            apps.push(WhiteListApp {
                icon: app.icon_path,
                name: app.name,
                path: app.exec_path,
                checked: false,
            });
        }
    }

    return apps;
}

#[tauri::command]
pub fn get_blacklist_apps() -> Vec<AppIndex> {
    let mut apps: Vec<AppIndex> = Vec::new();
    let settings = get_settings();
    let blacklist = settings.results.blacklist;

    let indexed_apps_yaml = fs::read_to_string(get_apps_index_path().unwrap()).unwrap();
    let indexed_apps: Vec<AppIndex> = serde_yaml::from_str(&indexed_apps_yaml).unwrap();

    for app in indexed_apps {
        if blacklist.contains(&app.exec_path) {
            apps.push(app);
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));

    return apps;
}

#[tauri::command]
pub fn add_to_blacklist(path: String, app: AppHandle) {
    let mut settings = get_settings();
    let mut blacklist = settings.results.blacklist;

    if !blacklist.contains(&path) {
        blacklist.push(path.to_owned());
    }

    settings.results.blacklist = blacklist;

    settings::update_settings(&settings);

    app.emit_all("update-blacklist",()).unwrap();
}

#[tauri::command]
pub fn remove_from_blacklist(path: String, app: AppHandle) {
    let mut settings = get_settings();
    let mut blacklist = settings.results.blacklist;

    blacklist = blacklist
        .iter()
        .map(|p| p.to_owned())
        .filter(|p| p != &path)
        .collect();

    settings.results.blacklist = blacklist.to_owned();
    settings::update_settings(&settings);

    app.emit_all("update-blacklist",()).unwrap();
}
