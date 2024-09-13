use std::fs;
use whiskers_launcher_rs::{
    indexing::App,
    paths::{get_indexing_apps_path, get_indexing_dir},
};

#[cfg(target_os = "windows")]
use whiskers_launcher_rs::paths::{get_app_dir, get_app_resources_dir};

#[cfg(target_os = "linux")]
use {freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter},tux_icons::icon_fetcher::IconFetcher};

/// Gets the apps from the system and indexes them into a file
pub fn index_apps() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let mut apps_indexing = Vec::<App>::new();
        let mut ids = Vec::<String>::new();
        let fetcher = IconFetcher::new();

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
                            let icon = fetcher.clone().get_icon_path_from_desktop(entry.path);
                            let exec_path = path.clone().into_os_string().into_string().unwrap();
                            let title = entry.name(None).unwrap().to_string();

                            let mut app_indexing = App::new(&exec_path, &title, &exec_path);

                            match icon {
                                Some(path) => {
                                    app_indexing
                                        .icon(&path.into_os_string().into_string().unwrap());
                                }
                                None => {}
                            }

                            apps_indexing.push(app_indexing);
                        }
                    }
                }
            }
        }

        let indexig_dir = get_indexing_dir();

        if !indexig_dir.exists() {
            fs::create_dir_all(&indexig_dir).expect("Error creating indexing dir");
        }

        apps_indexing.sort_by_key(|a| a.to_owned().title);

        let encoded_apps_indexing =
            bincode::serialize(&apps_indexing).expect("Error serializing apps indexing");

        fs::write(get_indexing_apps_path(), encoded_apps_indexing)
            .expect("Error writing apps indexing");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let mut script_path = get_app_resources_dir();
        script_path.push("scripts/index-apps.ps1");

        let script_content =
            fs::read_to_string(&script_path).expect("Error getting script content");
        powershell_script::run(&script_content).expect("Error running index script");

        let mut apps_json_path = get_app_dir();
        apps_json_path.push("indexing/apps.json");

        let apps_json_content =
            fs::read_to_string(&apps_json_path).expect("Error getting apps json");

        let apps: Vec<App> = serde_json::from_str(&apps_json_content).expect("Error getting apps");

        let bytes = bincode::serialize(&apps).expect("Error serializing apps");

        if !get_indexing_dir().exists() {
            fs::create_dir_all(get_indexing_dir()).expect("Error creating index directory");
        }

        fs::write(get_indexing_apps_path(), &bytes).expect("Error writing apps binary");
    }
}
