use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use simple_kl_rs::paths::get_home_path;
use walkdir::WalkDir;

pub fn get_app_icon(icon: String) -> Option<PathBuf> {
    let data_dirs_string = env::var("XDG_DATA_DIRS").unwrap();

    let mut xdg_data_dirs: Vec<&str> = data_dirs_string.split(":").collect();

    let mut home_icons_location = get_home_path().unwrap();
    home_icons_location.push(".local/share");

    let home_icons_location_str = home_icons_location.into_os_string().into_string().unwrap();

    xdg_data_dirs.push(&home_icons_location_str);

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
