// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


pub mod commands;
pub mod windows;

use windows::open_settings_window;
use commands::settings::{get_settings, write_settings, is_wayland};


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      open_settings_window,
      get_settings,
      write_settings,
      is_wayland     
    ])
    .run(tauri::generate_context!())
    .expect("Error running app");
}
