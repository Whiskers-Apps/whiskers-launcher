use std::env;

use whiskers_launcher_rs::{
    api::settings::{self},
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
