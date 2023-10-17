// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Cursor;
use std::path::Path;

use fs_extra::dir::CopyOptions;
use serde::Serialize;
use simple_kl_rs::paths::get_local_dir;
use tauri::{AppHandle, Manager, App};

#[derive(Serialize, Clone)]
struct InstallPayload {
    message: String,
    finished: bool,
}

#[tauri::command]
async fn install(url: String, tag: String, app: AppHandle) {
    tokio::spawn(async move {

        set_install_message(&app, "Downloading files ...", false);

        let zip_name = format!("/tmp/Linux-Simple-KL-{}.zip", &tag);
        let zip_path = Path::new(&zip_name);

        let response = reqwest::get(&url)
            .await
            .expect("Error getting file from url");
        let file_content = response
            .bytes()
            .await
            .expect("Error getting content from file");
        fs::write(&zip_path, &mut &file_content).expect("Error creating file");

        let zip_bytes = fs::read(&zip_path).unwrap();
        let extraction_dir_name = format!("/tmp/Linux-Simple-KL-{}", &tag);
        let extraction_dir = Path::new(&extraction_dir_name);

        set_install_message(&app, "Extracting files ...", false);

        zip_extract::extract(Cursor::new(zip_bytes), &extraction_dir, true).unwrap();

        set_install_message(&app, "Copying files ...", false);

        let local_dir = get_local_dir().unwrap();

        if !&local_dir.exists() {
            fs::create_dir_all(&local_dir).unwrap();
        }

        let mut zip_resources = extraction_dir.to_owned();
        zip_resources.push("resources");

        let mut zip_logo = extraction_dir.to_owned();
        zip_logo.push("simple-kl.png");

        let mut zip_launcher = extraction_dir.to_owned();
        zip_launcher.push("simple-keyboard-launcher");

        let mut zip_service = extraction_dir.to_owned();
        zip_service.push("simple-kl-service");

        let mut zip_desktop = extraction_dir.to_owned();
        zip_desktop.push("simple-kl.desktop");


        fs_extra::dir::copy(
            &zip_resources,
            &local_dir,
            &CopyOptions::new().overwrite(true),
        )
        .unwrap();

        set_install_message(&app, "Install completed. Thank you for trying Simple KL", true);
    });
}

fn set_install_message(app: &AppHandle, message: &str, finished: bool){
    app.emit_all(
            "install",
            InstallPayload {
                message: String::from(message),
                finished,
            },
        )
        .unwrap();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
