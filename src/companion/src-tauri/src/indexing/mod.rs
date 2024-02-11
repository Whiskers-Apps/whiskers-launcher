use std::fs;

use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use whiskers_launcher_rs::{indexing::AppIndex, paths::{get_indexing_apps_path, get_indexing_dir}};

use crate::functions::get_app_icon;

/// Searches for all apps and saves them in a apps json file.
/// It reduces the time the app needs to get all the apps over and over again
pub fn index_apps() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let mut apps: Vec<AppIndex> = Vec::new();
        let mut ids: Vec<String> = Vec::new();

        //Gets All Apps
        for path in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                    if !ids.contains(&entry.appid.to_string()) && !entry.no_display() {
                        ids.push(entry.appid.to_string());

                        if let Some(entry_type) = entry.type_() {
                            if entry_type == "Application" && !entry.no_display() {}
                        }

                        if entry.type_().unwrap() == "Application" && !entry.no_display() {
                            let icon = entry.icon().unwrap().to_string();
                            let icon_path = match get_app_icon(icon){
                                Some(path)=> path.into_os_string().into_string().unwrap(),
                                None => "".into()
                            };
                            
                            let exec_path = path.clone().into_os_string().into_string().unwrap();
                            let name = entry.name(None).unwrap().to_string();

                            apps.push(AppIndex::new(icon_path, exec_path, name))
                        }
                    }
                }
            }
        }

        let indexig_dir = get_indexing_dir().unwrap();

        if !indexig_dir.exists(){
            fs::create_dir_all(&indexig_dir).unwrap();
        }

        //Saves the apps in a json file
        let json_apps = serde_json::to_string(&apps).unwrap();
        fs::write(&get_indexing_apps_path().unwrap(), &json_apps).unwrap();
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let mut script_path = get_local_dir().unwrap();
        script_path.push("resources\\ps-scripts\\index-apps.ps1");

        let script_content = fs::read_to_string(&script_path).unwrap();
        powershell_script::run(&script_content).unwrap();
    }
}
