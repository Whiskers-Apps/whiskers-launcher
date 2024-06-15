use std::{env, fs};

use git2::Repository;
use serde::{Deserialize, Serialize};
use whiskers_launcher_rs::{
    api::{apps::get_apps, extensions::get_extension_dir, settings},
    extension::Extension,
    indexing::App,
    paths::{get_extensions_dir, get_extensions_store_path},
    settings::{SearchEngine, Settings, Theme},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionStoreItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub repo: String,
    pub preview: String,
    #[serde(default = "default_os")]
    pub os: Option<Vec<String>>,
}

fn default_os() -> Option<Vec<String>> {
    None
}

#[tauri::command]
pub async fn get_settings() -> Settings {
    settings::get_settings()
}

#[tauri::command]
pub async fn write_settings(settings: Settings) {
    settings::write_settings(settings);
}

#[tauri::command]
pub fn get_os() -> String {
    env::consts::OS.to_string()
}

#[tauri::command]
pub fn is_wayland() -> bool {
    if get_os() != "linux" {
        return false;
    }

    match env::var("XDG_SESSION_TYPE") {
        Ok(session) => &session.to_lowercase() == "wayland",
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn get_blacklisted_apps() -> Vec<App> {
    let settings = settings::get_settings();
    let ids = settings.blacklist;
    let apps = get_apps();

    let mut blacklisted_apps = Vec::<App>::new();

    for app in apps {
        if ids.contains(&app.id) {
            blacklisted_apps.push(app);
        }
    }

    blacklisted_apps
}

#[tauri::command]
pub async fn get_whitelisted_apps() -> Vec<App> {
    let settings = settings::get_settings();
    let ids = settings.blacklist;
    let apps = get_apps();

    let mut whitelisted_apps = Vec::<App>::new();

    for app in apps {
        if !ids.contains(&app.id) {
            whitelisted_apps.push(app)
        }
    }

    whitelisted_apps
}

#[tauri::command]
pub async fn add_to_blacklist(id: String) {
    let mut settings: Settings = settings::get_settings();
    let mut new_blacklist = settings.blacklist;
    new_blacklist.push(id);

    settings.blacklist = new_blacklist;
    settings::write_settings(settings);
}

#[tauri::command]
pub async fn remove_from_blacklist(id: String) {
    let mut settings = settings::get_settings();

    settings.blacklist = settings
        .blacklist
        .iter()
        .map(|b_id| b_id.to_owned())
        .filter(|b_id| b_id != &id)
        .collect();

    settings::write_settings(settings);
}

#[tauri::command]
pub async fn get_new_search_engine_id() -> usize {
    let settings = settings::get_settings();
    let engines = settings.search_engines;

    let last_engine = engines.iter().max_by_key(|e| e.id);

    match last_engine {
        Some(engine) => engine.id + 1,
        None => 0,
    }
}

#[tauri::command]
pub async fn add_search_engine(engine: SearchEngine) {
    let mut settings = settings::get_settings();
    settings.search_engines.push(engine);
    settings::write_settings(settings);
}

#[tauri::command]
pub async fn update_search_engine(engine: SearchEngine) {
    let mut settings = settings::get_settings();
    let mut new_engines = Vec::<SearchEngine>::new();

    for eng in settings.search_engines {
        if eng.id == engine.id {
            new_engines.push(engine.to_owned());
        } else {
            new_engines.push(eng);
        }
    }

    settings.search_engines = new_engines;
    settings::write_settings(settings);
}

#[tauri::command]
pub async fn delete_search_engine(id: usize) {
    let mut settings = settings::get_settings();
    settings.search_engines = settings
        .search_engines
        .to_owned()
        .iter()
        .map(|s| s.to_owned())
        .filter(|s| s.id != id)
        .collect();

    settings::write_settings(settings);
}

#[tauri::command]
pub async fn get_theme_from_file(path: String) -> Theme {
    let json = fs::read_to_string(path).expect("Error reading theme file");
    let theme: Theme = serde_json::from_str(&json).expect("Error converting json to theme");
    theme
}

#[tauri::command]
pub async fn export_theme(path: String) {
    let settings = settings::get_settings();
    let theme = settings.theme;
    let theme_json = serde_json::to_string(&theme).expect("Error converting theme to json");
    fs::write(path, theme_json).expect("Error exporting theme to file");
}

#[tauri::command]
pub async fn clone_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let mut repo_name = url_split[url_split.len() - 1].to_owned();

    if repo_name.ends_with(".git") {
        repo_name = repo_name.trim_end_matches(".git").to_owned();
    }

    let mut path = get_extensions_dir();
    path.push(repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");

    index_extensions().await;
}

#[tauri::command]
pub async fn get_extensions() -> Vec<Extension> {
    whiskers_launcher_rs::api::extensions::get_extensions()
}

#[tauri::command]
pub async fn open_extension_dir(id: String) {
    let path = get_extension_dir(&id).expect("Error getting extension dir");
    open::that(&path).expect("Error opening extension dir");
}

#[tauri::command]
pub async fn index_extensions() {
    whiskers_launcher_rs::api::extensions::index_extensions();
}

#[tauri::command]
pub async fn remove_extension(id: String) {
    let dir = get_extension_dir(&id).expect("Error getting extension dir");
    fs::remove_dir_all(&dir).expect("Error removing extension dir");
}

#[tauri::command]
pub async fn get_extensions_store() -> Vec<ExtensionStoreItem> {
    let path = get_extensions_store_path();

    if !path.exists() {
        return Vec::new();
    }

    let bytes = fs::read(&path).expect("Error reading extensions store");
    let store = bincode::deserialize::<Vec<ExtensionStoreItem>>(&bytes)
        .expect("Error deserializing extensions store");

    store
}

#[tauri::command]
pub async fn write_extensions_store(store: Vec<ExtensionStoreItem>) {
    let path = get_extensions_store_path();

    if !path.exists(){
        fs::create_dir_all(path.parent().expect("Expected parent directory")).expect("Error creating extensions store directory");
    }
    
    let bytes = bincode::serialize(&store).expect("Error serializing extensions store");
    fs::write(&path, &bytes).expect("Error writing extensions store");
}

