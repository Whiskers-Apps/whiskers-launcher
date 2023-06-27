use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub general: GeneralSettings,
    pub search_box: SearchBoxSettings,
    pub theming: ThemingSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralSettings {
    pub first_key: String,
    pub second_key: String,
    pub third_key: String,
    pub limit: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchBoxSettings {
    pub show_search_icon: bool,
    pub show_settings_icon: bool,
    pub roundness: usize,
    pub border_width: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemingSettings {
    pub background: String,
    pub secondary_background: String,
    pub tertiary_background: String,
    pub accent: String,
    pub on_accent: String,
    pub text: String,
    pub seconday_text: String,
}

impl Settings {
    pub fn path() -> String {
        let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
        let settings_path = format!("{home_path}/.config/simple-keyboard-launcher/settings.json");

        return settings_path;
    }

    pub fn default_settings() -> Settings {
        return Settings {
            general: GeneralSettings {
                first_key: "alt".to_string(),
                second_key: "".to_string(),
                third_key: "space".to_string(),
                limit: 6,
            },
            search_box: SearchBoxSettings {
                show_search_icon: false,
                show_settings_icon: true,
                roundness: 4,
                border_width: 2,
            },
            theming: ThemingSettings {
                background: "#1e1e2e".to_string(),
                secondary_background: "#11111b".to_string(),
                tertiary_background: "#181825".to_string(),
                accent: "#89b4fa".to_string(),
                on_accent: "#1e1e2e".to_string(),
                text: "#cdd6f4".to_string(),
                seconday_text: "#bac2de".to_string(),
            },
        };
    }

    pub fn init_settings() {
        let settings_path = Settings::path();
        let default_settings = Settings::default_settings();

        if !Path::new(&settings_path).exists() {
            fs::create_dir_all(Path::new(&settings_path).parent().unwrap())
                .expect("Failed to create configs folder");

            let mut settings_file =
                File::create(&settings_path).expect("Failed to create settings file");
            let settings_json =
                serde_json::to_string(&default_settings).expect("Error converting settings json");

            settings_file
                .write_all(&settings_json.as_bytes())
                .expect("Error saving default settings");

            settings_file.flush().unwrap();
        }
    }

    pub fn current_settings() -> Settings {
        let settings_path = Settings::path();
        let mut settings_file = File::open(&settings_path).expect("Failed to open settings");
        let mut settings_content = String::new();

        settings_file
            .read_to_string(&mut settings_content)
            .expect("Failed to read settings");

        let current_settings =
            serde_json::from_str(&settings_content).unwrap_or(Settings::default_settings());
        return current_settings;
    }

    pub fn update(new_settings: String) -> Result<(), String> {
        let settings_path = Settings::path();
        let mut settings_file = File::create(&settings_path).expect("Failed to open settings");

        settings_file.write_all(&new_settings.as_bytes()).expect("");

        return match settings_file.flush() {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string()),
        };
    }

    pub fn launch_shortcut() -> String {
        let settings = Settings::current_settings();
        let first_key = settings.general.first_key;
        let second_key = settings.general.second_key;
        let third_key = settings.general.third_key;

        return match second_key.is_empty() || second_key.as_str() == "-" {
            true => {
                format!("{first_key}+{third_key}")
            }
            false => {
                format!("{first_key}+{second_key}+{third_key}")
            }
        };
    }
}
