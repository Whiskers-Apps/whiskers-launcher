// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data_structs::SimpleKLResult;

pub mod data_structs;

#[tauri::command]
fn get_results() -> Result<Vec<SimpleKLResult>, String>{

    let mut results: Vec<SimpleKLResult> = Vec::new();

    results.push(SimpleKLResult{
        icon: None,
        title: Some("".into()),
        description: None,
        text: None
    });

    return Ok(results)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_results])
        .plugin(tauri_plugin_positioner::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
