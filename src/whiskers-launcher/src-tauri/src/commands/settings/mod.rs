use std::env;

use whiskers_launcher_rs::{
    api::{apps::get_apps_indexing, settings},
    indexing::App,
    settings::Settings,
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
    let mut settings = settings::get_settings();
    let mut new_blacklist = settings.blacklist;
    new_blacklist.push(id);

    settings.blacklist = new_blacklist;
    settings::write_settings(settings);
}

#[tauri::command]
pub async fn remove_from_blacklist(id: String) {
    let mut settings = settings::get_settings();
    let new_blacklist: Vec<String> = settings
        .blacklist
        .iter()
        .map(|b_id| b_id.to_owned())
        .filter(|b_id| b_id != &id)
        .collect();

    settings.blacklist = new_blacklist;
    settings::write_settings(settings);
}
