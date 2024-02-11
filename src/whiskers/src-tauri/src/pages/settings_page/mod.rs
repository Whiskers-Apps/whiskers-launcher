use std::path::PathBuf;
use std::fs;

use git2::Repository;
use serde::{Deserialize, Serialize};
use whiskers_launcher_rs::api::extensions::manifest::Manifest;
use whiskers_launcher_rs::indexing::{self, get_indexed_apps, AppIndex};
use whiskers_launcher_rs::paths::get_user_extensions_dir;
use whiskers_launcher_rs::settings::{self, Settings, Theme};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionDefaultValues {
    pub id: String,
    pub default_values: Vec<DefaultValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultValue {
    pub setting_id: String,
    pub value: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct WhiteListApp {
    pub icon_path: Option<String>,
    pub name: String,
    pub exec_path: String,
    #[serde(default = "default_checked")]
    pub checked: bool,
}

fn default_checked() -> bool {
    return false;
}

#[cfg(target_os = "windows")]
use {simple_kl_rs::others::FLAG_NO_WINDOW, std::os::windows::process::CommandExt};

#[tauri::command]
pub fn get_settings() -> Settings {
    return settings::get_settings().unwrap();
}

#[tauri::command]
pub fn update_settings(settings: settings::Settings) {
    settings::update_settings(settings).unwrap();
}

#[tauri::command]
pub fn get_whitelisted_apps() -> Vec<WhiteListApp> {
    let apps = get_indexed_apps().unwrap();
    let blacklist = settings::get_settings().unwrap().blacklist;
    let mut whitelisted_apps: Vec<WhiteListApp> = vec![];

    for app in apps {
        if !blacklist.contains(&app.exec_path) {
            whitelisted_apps.push(WhiteListApp {
                icon_path: app.icon_path,
                exec_path: app.exec_path,
                name: app.name,
                checked: false,
            });
        }
    }

    whitelisted_apps.sort_by(|a, b| a.name.cmp(&b.name));

    return whitelisted_apps;
}

#[tauri::command]
pub fn add_to_blacklist(paths: Vec<String>) {
    let mut new_settings = settings::get_settings().unwrap();
    let mut new_blacklist = new_settings.blacklist.to_owned();

    for path in paths {
        if !new_blacklist.contains(&path) {
            new_blacklist.push(path);
        }
    }

    new_settings.set_blacklist(new_blacklist);

    settings::update_settings(new_settings).unwrap();
}

#[tauri::command]
pub fn get_blacklisted_apps() -> Vec<AppIndex> {
    let blacklist = settings::get_settings().unwrap().blacklist;
    let indexed_apps = get_indexed_apps().unwrap();
    let mut blacklisted_apps: Vec<AppIndex> = indexed_apps
        .iter()
        .map(|ba| ba.to_owned())
        .filter(|ba| blacklist.contains(&ba.exec_path))
        .collect();

    blacklisted_apps.sort_by(|a, b| a.name.cmp(&b.name));

    return blacklisted_apps;
}

#[tauri::command]
pub fn export_theme(path: PathBuf) {
    let theme = settings::get_settings().unwrap().theme;
    let theme_json = serde_json::to_string_pretty(&theme).unwrap();

    fs::write(&path, &theme_json).unwrap();
}

#[tauri::command]
pub fn import_theme(path: PathBuf) {
    let content = fs::read_to_string(&path).unwrap();
    let theme: Theme = serde_json::from_str(&content).unwrap();

    let mut settings = settings::get_settings().unwrap();
    settings.set_theme(theme);
    settings::update_settings(settings).unwrap();
}

#[tauri::command]
pub fn get_user_extensions() -> Vec<Manifest> {
    return indexing::get_user_extensions().expect("Error getting user extensions");
}

#[tauri::command]
pub fn get_extensions_default_values() -> Vec<ExtensionDefaultValues> {
    let mut extensions_default_values: Vec<ExtensionDefaultValues> = vec![];
    let extensions = get_user_extensions();

    for extension in extensions {
        let mut default_values: Vec<DefaultValue> = vec![];

        if let Some(settings) = extension.settings {
            for setting in settings {
                default_values.push(DefaultValue {
                    setting_id: setting.id,
                    value: setting.default_value,
                });
            }
        }

        extensions_default_values.push(ExtensionDefaultValues {
            id: extension.id,
            default_values,
        });
    }

    return extensions_default_values;
}

#[tauri::command]
pub async fn clone_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let mut repo_name = url_split[url_split.len() - 1].to_owned();

    if repo_name.ends_with(".git"){
        repo_name = repo_name.trim_end_matches(".git").to_owned();
    }

    let mut path = get_user_extensions_dir().unwrap();
    path.push(repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");
}