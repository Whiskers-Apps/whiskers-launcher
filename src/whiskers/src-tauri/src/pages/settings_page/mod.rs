use std::fs;
use std::path::PathBuf;

use git2::Repository;
use serde::{Deserialize, Serialize};
use whiskers_launcher_rs::api::extensions::get_extension_dir;
use whiskers_launcher_rs::api::extensions::manifest::Manifest;
use whiskers_launcher_rs::extensions::{self};
use whiskers_launcher_rs::indexing::{self, get_indexed_apps, AppIndex};
use whiskers_launcher_rs::paths::{
    get_cached_extensions_store_path, get_cached_themes_store_path, get_store_cache_dir,
    get_user_extensions_dir,
};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreTheme {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub preview: String,
    pub variants: Vec<StoreThemeVariant>,
    pub file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreThemeVariant {
    pub name: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreExtension{
    pub id: String,
    pub name: String,
    pub description: String,
    pub repo: String,
    pub os: Vec<String>
}

fn default_checked() -> bool {
    false
}

#[cfg(target_os = "windows")]
use {simple_kl_rs::others::FLAG_NO_WINDOW, std::os::windows::process::CommandExt};

#[tauri::command]
pub fn get_settings() -> Settings {
    settings::get_settings().unwrap()
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

    whitelisted_apps
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

    blacklisted_apps
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
    extensions::index_extensions().unwrap();
    indexing::get_user_extensions().expect("Error getting user extensions")
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

    extensions_default_values
}

#[tauri::command]
pub async fn clone_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let mut repo_name = url_split[url_split.len() - 1].to_owned();

    if repo_name.ends_with(".git") {
        repo_name = repo_name.trim_end_matches(".git").to_owned();
    }

    let mut path = get_user_extensions_dir().unwrap();
    path.push(repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");
}

#[tauri::command(rename_all = "snake_case")]
pub async fn uninstall_extension(extension_id: String) -> Result<(), ()> {
    let mut settings = get_settings();
    let new_extensions: Vec<settings::Extension> = settings
        .extensions
        .iter()
        .map(|e| e.to_owned())
        .filter(|e| e.id != extension_id)
        .collect();

    settings.extensions = new_extensions;

    if let Some(extension_dir) = get_extension_dir(extension_id) {
        fs::remove_dir_all(extension_dir).unwrap();
    }

    update_settings(settings);

    Ok(())
}

#[tauri::command]
pub fn get_cached_themes_store() -> Vec<StoreTheme> {
    let store_cache_dir = get_store_cache_dir().unwrap();

    if !store_cache_dir.exists() {
        fs::create_dir_all(&store_cache_dir).unwrap();
    }

    let cached_themes_store_path = get_cached_themes_store_path().unwrap();

    if !cached_themes_store_path.exists() {
        return vec![];
    }

    let file_content = fs::read_to_string(&cached_themes_store_path).unwrap();

    if let Ok(themes) = serde_json::from_str::<Vec<StoreTheme>>(&file_content) {
        return themes;
    }

    vec![]
}

#[tauri::command]
pub fn cache_themes(themes: Vec<StoreTheme>){
    let themes_json = serde_json::to_string(&themes).unwrap();
    fs::write(&get_cached_themes_store_path().unwrap(), &themes_json).unwrap();
}

#[tauri::command]
pub fn get_cached_extensions_store() -> Vec<StoreExtension> {
    let store_cache_dir = get_store_cache_dir().unwrap();

    if !store_cache_dir.exists() {
        fs::create_dir_all(&store_cache_dir).unwrap();
    }

    let cached_extensions_store_path = get_cached_extensions_store_path().unwrap();

    if !cached_extensions_store_path.exists() {
        return vec![];
    }

    let file_content = fs::read_to_string(&cached_extensions_store_path).unwrap();

    if let Ok(extensions) = serde_json::from_str::<Vec<StoreExtension>>(&file_content) {
        return extensions;
    }

    vec![]
}

#[tauri::command]
pub async fn apply_store_theme(file: String) {
    let theme_json = reqwest::get(file).await.unwrap().text().await.unwrap();
    let theme = serde_json::from_str::<Theme>(&theme_json).unwrap();

    let mut new_settings = get_settings();
    new_settings.set_theme(theme);

    settings::update_settings(new_settings).unwrap();
}

#[tauri::command]
pub fn cache_extensions(extensions: Vec<StoreExtension>){
    let extensions_json = serde_json::to_string(&extensions).unwrap();
    fs::write(&get_cached_extensions_store_path().unwrap(), &extensions_json).unwrap();
}

#[tauri::command]
pub fn index_extensions(){
    extensions::index_extensions().unwrap();
}