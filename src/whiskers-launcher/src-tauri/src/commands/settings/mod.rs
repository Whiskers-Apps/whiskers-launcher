use std::{env, fs};

use whiskers_launcher_rs::{
    api::{apps::get_apps_indexing, settings},
    indexing::App,
    settings::{SearchEngine, Settings, Theme},
};

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
    let apps = get_apps_indexing();

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
    let apps = get_apps_indexing();

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
