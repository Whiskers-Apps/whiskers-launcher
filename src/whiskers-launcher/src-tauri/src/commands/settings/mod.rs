use whiskers_launcher_rs::{api::settings::{self}, settings::Settings};

#[tauri::command]
pub async fn get_settings() -> Settings{
    settings::get_settings()
}

