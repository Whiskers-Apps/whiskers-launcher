use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use git2::Repository;
use simple_kl_rs::extensions::{ExtensionManifest, get_extensions, init_extensions};
use simple_kl_rs::paths::get_extensions_path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommunityExtension {
    pub id: String,
    #[serde(default = "get_empty_string")]
    pub name: String,
    #[serde(default = "get_empty_string")]
    pub description: String,
    #[serde(default = "get_empty_string")]
    pub repo: String,
    #[serde(default = "get_empty_string")]
    pub preview: String,
    #[serde(default = "get_empty_string_vec")]
    pub platforms: Vec<String>,
}

fn get_empty_string() -> String { return String::from(""); }

fn get_empty_string_vec() -> Vec<String> { return vec![] }

#[tauri::command(rename_all = "snake_case")]
pub fn update_extension_setting(extension_id: String, setting_id: String, new_value: String) {
    simple_kl_rs::extensions::update_extension_setting(&extension_id, &setting_id, &new_value);
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_extension_keyword(extension_id: String, keyword: String) {
    simple_kl_rs::extensions::update_extension_keyword(&extension_id, &keyword);
}

#[tauri::command()]
pub fn get_extensions_json() -> String {
    let mut extensions: Vec<ExtensionManifest> = Vec::new();

    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file
                        .read_to_string(&mut manifest_json)
                        .expect("Error writing content to string");
                    let _ = manifest_file.flush();
                    let manifest: ExtensionManifest =
                        serde_yaml::from_str(&manifest_json).expect("Error converting manifest");

                    extensions.push(manifest);
                }
            }
        }
    }

    return serde_json::to_string(&extensions).unwrap();
}

#[tauri::command()]
pub async fn import_extension(url: String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let repo_name = url_split[url_split.len() - 1];
    let mut path = get_extensions_path().unwrap();
    path.push(repo_name);

    Repository::clone(&url, path).expect("Error cloning repo");

    init_extensions();
}

#[tauri::command()]
pub async fn delete_extension(id: String) {
    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_json = String::from("");
                    manifest_file.read_to_string(&mut manifest_json).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_json).unwrap();

                    if manifest.id == id {
                        fs::remove_dir_all(&folder_path).expect("Error deleting extension folder");
                    }

                    init_extensions();

                    manifest_file.flush().unwrap();
                }
            }
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_extension_default_keyword(extension_id: String) -> Result<String, ()> {
    let extensions = get_extensions();

    return match extensions.iter().find(|e| e.id == extension_id) {
        Some(extension) => Ok(extension.keyword.clone()),
        None => Err(()),
    };
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_extension_default_setting(setting_id: String, extension_id: String) -> Result<String, ()> {
    let extensions = get_extensions();

    for extension in extensions {
        if extension.id == extension_id {
            if let Some(settings) = extension.settings {
                for setting in settings.any {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }

                for setting in settings.linux {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }

                for setting in settings.windows {
                    if setting.id == setting_id {
                        return Ok(setting.default_value);
                    }
                }
            }
        }
    }

    return Err(());
}

