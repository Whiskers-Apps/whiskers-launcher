use std::time::Duration;

use tauri::{AppHandle, WindowBuilder};

#[tauri::command]
pub async fn open_settings_window(app: AppHandle) {
    let window = WindowBuilder::new(&app, "settings", tauri::WindowUrl::App("settings".into()))
        .title("Settings")
        .inner_size(1200.0, 600.0);

    window.build().expect("Error opening settings window");

    tokio::time::sleep(Duration::from_micros(500)).await;
}
