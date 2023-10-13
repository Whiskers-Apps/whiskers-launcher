// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs};
use std::path::Path;

#[tauri::command]
async fn install(
    url: String
) {
    tokio::spawn(async move {
        let file_path = Path::new("/tmp/linux.zip");
        let response = reqwest::get(&url).await.expect("Error getting file from url");
        let file_content = response.bytes().await.expect("Error getting content from file");
        fs::write(&file_path, &mut &file_content).expect("Error creating file");
    });
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
