use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use simple_kl_rs::actions::{DialogAction, DialogResult};
use simple_kl_rs::extensions::Parameters;
use simple_kl_rs::paths::{get_dialog_action_path, get_extension_parameters_path, get_extension_path};
use tauri::Window;

#[cfg(target_os = "windows")]
use{
    std::os::windows::process::CommandExt,
    simple_kl_rs::others::FLAG_NO_WINDOW
};
#[tauri::command]
pub fn get_dialog_action() -> Result<DialogAction, ()> {
    let dialog_actions_yaml = fs::read_to_string(&get_dialog_action_path().unwrap())
        .expect("Error reading dialog fields file");

    let action: DialogAction = serde_yaml::from_str(&dialog_actions_yaml)
        .expect("Error getting yaml from dialog fields file");

    return Ok(action);
}

#[tauri::command]
pub fn write_dialog_result(result: DialogResult, window: Window) {
    let fields = serde_yaml::to_string(&result.results).unwrap();

    let parameters = Parameters::new_action(result.action, vec![fields]);

    let parameters_yaml = serde_yaml::to_string(&parameters).unwrap();

    let mut parameters_file = File::create(get_extension_parameters_path().unwrap())
        .expect("Error creating parameters file");

    parameters_file
        .write_all(&parameters_yaml.as_bytes())
        .unwrap();

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./extension")
            .current_dir(get_extension_path(&result.extension_id).unwrap())
            .output()
            .expect("Error running extension action");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("start extension.exe")
            .current_dir(get_extension_path(&result.extension_id).unwrap())
            .creation_flags(FLAG_NO_WINDOW)
            .output()
            .expect("Error running extension action");
    }

    window.close().expect("Error closing window");
}