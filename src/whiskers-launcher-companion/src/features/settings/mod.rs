use std::process::exit;

use whiskers_launcher_core::features::core::settings::{get_settings, write_settings, Settings};

pub fn cli_show_settings() {
    let settings_json = serde_json::to_string(&get_settings()).expect("Error getting settings");
    println!("{settings_json}");
    exit(0)
}

pub fn cli_write_settings(settings_json: &str) {
    let settings: Settings =
        serde_json::from_str(settings_json).expect("Error deserializing settings");

    write_settings(settings);
}
