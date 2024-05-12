#[tauri::command]
pub fn get_new_text() -> String {
    return String::from("Bulbasaur");
}