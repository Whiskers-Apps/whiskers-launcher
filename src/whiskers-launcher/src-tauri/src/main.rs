// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


pub mod commands;

use commands::settings::get_new_text;




fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_new_text      
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
