use std::env;

#[tauri::command()]
pub fn get_os() -> Result<String, ()> {
    return Ok(String::from(env::consts::OS));
}

#[tauri::command()]
pub fn debug_message(message: String) {
    println!("{}", message);
}