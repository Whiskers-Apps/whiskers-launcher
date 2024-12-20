use std::time::Duration;

use enigo::{Enigo, Mouse, Settings};
use tauri::{AppHandle, PhysicalPosition, WindowBuilder};
use whiskers_launcher_core::features::core::settings::get_settings;

#[tauri::command]
pub async fn run_open_settings_window(app: AppHandle) {
    open_settings_window(&app);
    tokio::time::sleep(Duration::from_micros(500)).await;
}

pub fn open_settings_window(app: &AppHandle) {
    let window = WindowBuilder::new(app, "settings", tauri::WindowUrl::App("settings".into()))
        .title("Settings")
        .inner_size(1200.0, 600.0);

    window.build().expect("Error opening settings window");
}

pub fn open_search_window(app: &AppHandle) {
    let window = WindowBuilder::new(app, "search", tauri::WindowUrl::App("search".into()))
        .title("Settings")
        .inner_size(900.0, 800.0)
        .maximizable(true)
        .resizable(false)
        .title("Whiskers Launcher")
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .build()
        .unwrap();

    // Opens the window in the monitor where the cursor is and centers it
    let mut monitors = window
        .available_monitors()
        .expect("Error getting monitors");

    let (cursor_x, _cursor_y) = Enigo::new(&Settings::default())
        .expect("Error initializing enigo")
        .location()
        .expect("Error getting cursor location");

    monitors.sort_by(|a, b| b.position().x.cmp(&a.position().x));

    for monitor in monitors.iter() {
        if monitor.position().x <= cursor_x {
            let settings = get_settings();

            window
                .set_position(PhysicalPosition::new(monitor.position().x, 0))
                .expect("Error moving window");

            if settings.wallpaper.is_some() {
                window.set_fullscreen(true).unwrap();
                window.maximize().expect("Error maximizing window");
            } else {
                window.center().expect("Error centering window");
            }

            window.show().unwrap();
            break;
        }
    }
}
