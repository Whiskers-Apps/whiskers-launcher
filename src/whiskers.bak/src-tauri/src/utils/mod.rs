use std::env;

use whiskers_launcher_rs::settings::get_settings;

#[tauri::command()]
fn get_os() -> Result<String, ()> {
    return Ok(String::from(env::consts::OS));
}

#[tauri::command()]
pub fn debug_message(message: String) {
    println!("{}", message);
}

#[tauri::command()]
pub fn get_new_search_engine_id() -> Result<usize, ()> {
    let settings = get_settings().unwrap();
    let search_engines = settings.search_engines;

    let last_search_engine = search_engines.iter().max_by_key(|se| se.id);

    match last_search_engine {
        Some(se) => Ok(se.id + 1),
        None => Ok(0),
    }
}
