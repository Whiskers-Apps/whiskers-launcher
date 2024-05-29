use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;
#[cfg(target_os = "windows")]
use whiskers_launcher_rs::paths::get_app_resources_dir;
use whiskers_launcher_rs::{indexing::App, paths::get_home_dir};

#[cfg(target_os = "linux")]
use {
    freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter},
    whiskers_launcher_rs::paths::{get_indexing_apps_path, get_indexing_dir},
};

/// Gets the apps from the system and indexes them into a file
pub fn index_apps() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let mut apps_indexing = Vec::<App>::new();
        let mut ids = Vec::<String>::new();

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
                            if let Some(icon) = entry.icon() {
                                let icon = icon.to_string();

                                let exec_path =
                                    path.clone().into_os_string().into_string().unwrap();
                                let title = entry.name(None).unwrap().to_string();

                                let mut app_indexing = App::new(&exec_path, &title, &exec_path);

                                match get_app_icon(icon) {
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
        let mut script_path = get_app_resources_dir().unwrap();
        script_path.push("Scripts/index-apps.ps1");

        let script_content = fs::read_to_string(&script_path).unwrap();
        powershell_script::run(&script_content).unwrap();
    }
}

pub fn get_app_icon(icon: String) -> Option<PathBuf> {
    let data_dirs_string =
        env::var("XDG_DATA_DIRS/icons").unwrap_or("/usr/local/share:/usr/share".to_string());

    let mut xdg_data_dirs: Vec<&str> = data_dirs_string.split(":").collect();

    let mut home_icons_location = get_home_dir();
    home_icons_location.push(".local/share");

    let home_icons_location_str = home_icons_location.into_os_string().into_string().unwrap();

    xdg_data_dirs.push(&home_icons_location_str);

    /*
    let nix_store_paths = PathBuf::from("/nix/store");

    if nix_store_paths.exists() {
        for entry in fs::read_dir(nix_store_paths).expect("Error reading dir") {
            if let Ok(entry) = entry {
                let path = entry.path().into_os_string().into_string().unwrap();
                xdg_data_dirs.to_owned().push(&path);
            }
        }
    }
    */

    if Path::new(&icon).exists() {
        return Some(Path::new(&icon).to_owned());
    }

    let icon_theme_command = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("icon-theme")
        .output()
        .expect("Error getting current icon theme. Using hicolor instead");

    let mut icon_theme = String::from_utf8(icon_theme_command.stdout)
        .unwrap()
        .replace("'", "")
        .replace("\n", "");

    for data_dir in xdg_data_dirs.to_owned() {
        if Path::new(data_dir).exists() {
            let mut path = Path::new(&data_dir).to_owned();
            path.push("icons");
            path.push(&icon_theme);

            match get_icon_from_dir(path.to_owned(), &icon) {
                Some(icon_path) => return Some(icon_path),
                None => {}
            }
        }
    }

    //Use light theme if icon in dark icon themes wasn't found (Papirus and Breeze for example)
    if icon_theme.to_lowercase().contains("_dark") {
        icon_theme = icon_theme.replace("_dark", "");
        icon_theme = icon_theme.replace("_Dark", "");
    }

    if icon_theme.to_lowercase().contains("-dark") {
        icon_theme = icon_theme.replace("-dark", "");
        icon_theme = icon_theme.replace("-Dark", "");
    }

    for data_dir in xdg_data_dirs.to_owned() {
        if Path::new(data_dir).exists() {
            let mut path = Path::new(&data_dir).to_owned();
            path.push("icons");
            path.push(&icon_theme);

            match get_icon_from_dir(path.to_owned(), &icon) {
                Some(icon_path) => return Some(icon_path),
                None => {}
            }
        }
    }

    //Gets Icons From Pixmaps
    match get_icon_from_dir(Path::new("/usr/share/pixmaps").to_owned(), &icon) {
        Some(icon_path) => return Some(icon_path),
        None => {}
    }

    //Gets Icons From Hicolor
    for data_dir in xdg_data_dirs.to_owned() {
        if Path::new(data_dir).exists() {
            let mut path = Path::new(&data_dir).to_owned();
            path.push("icons/hicolor");

            match get_icon_from_dir(path.to_owned(), &icon) {
                Some(icon_path) => return Some(icon_path),
                None => {}
            }
        }
    }

    // Just Yolo and find any icon that matches
    for data_dir in xdg_data_dirs.to_owned() {
        if Path::new(data_dir).exists() {
            let mut path = Path::new(&data_dir).to_owned();
            path.push("icons");

            match get_icon_from_dir(path.to_owned(), &icon) {
                Some(icon_path) => return Some(icon_path),
                None => {}
            }
        }
    }

    return None;
}

pub fn get_icon_from_dir(path: PathBuf, icon: &str) -> Option<PathBuf> {
    if path.exists() && path.is_dir() {
        for entry in WalkDir::new(&path) {
            let entry = entry;

            if entry.is_ok() {
                let entry = entry.unwrap();

                if entry.path().is_file() {
                    let file_stem = entry.path().file_stem().unwrap().to_str().unwrap();

                    if icon == file_stem {
                        return Some(entry.path().to_owned());
                    }
                }
            }
        }
    }

    return None;
}
