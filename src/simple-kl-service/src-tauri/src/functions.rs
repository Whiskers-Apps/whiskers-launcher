use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use simple_kl_rs::paths::get_home_path;

pub fn get_app_icon(icon: String) -> Option<PathBuf> {
    let data_dirs_string = env::var("XDG_DATA_DIRS").unwrap();

    let mut xdg_data_dirs: Vec<&str> = data_dirs_string.split(":").collect();

    let mut home_icons_location = get_home_path().unwrap();
    home_icons_location.push(".local/share");

    let home_icons_location_str = home_icons_location.into_os_string().into_string().unwrap();

    xdg_data_dirs.push(&home_icons_location_str);
    xdg_data_dirs.push("/usr/share/pixmaps");

    let icon_dirs: Vec<&str> = vec![
        "apps",
        "actions",
        "categories",
        "devices",
        "emblems",
        "mimetypes",
        "places",
        "status",
        "scalable",
    ];

    let sizes_dirs: Vec<&str> = vec![
        "48x48", "8x8", "16x16", "18x18", "20x20", "22x22", "24x24", "32x32", "36x36", "40x40", "42x42", "64x64", "72x72" , "84x84",
        "96x96", "128x128", "256x256", "384x384", "480x480", "512x512"
    ];

    if Path::new(&icon).exists() {
        return Some(Path::new(&icon).to_owned());
    }

    let icon_theme_command = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("icon-theme")
        .output()
        .expect("Error getting current icon theme. Using hicolor instead");

    let icon_theme = String::from_utf8(icon_theme_command.stdout)
        .unwrap()
        .replace("'", "")
        .replace("\n", "");

    for data_dir in xdg_data_dirs {
        if Path::new(data_dir).exists() {
            let mut path = Path::new(data_dir).to_owned();

            //Searches the icon in pixmaps (it doesn't have any scale)
            if path == Path::new("/usr/share/pixmaps") {
                match get_icon_with_extension(path, icon.to_owned()) {
                    Some(path) => return Some(path),
                    None => return None
                }
            }

            //Searches the icon inside the icon theme folders
            for icon_dir in icon_dirs.to_owned() {
                for size_dir in sizes_dirs.to_owned() {
                    path = Path::new(&data_dir).to_owned();
                    path.push("icons");
                    path.push(icon_theme.to_owned());
                    path.push(&size_dir);
                    path.push(&icon_dir);


                    match get_icon_with_extension(path.to_owned(), icon.to_owned()) {
                        Some(path) => return Some(path),
                        None => {}
                    }

                    path = Path::new(&data_dir).to_owned();
                    path.push("icons");
                    path.push("hicolor");
                    path.push(&size_dir);
                    path.push(&icon_dir);
                    
                    match get_icon_with_extension(path.to_owned(), icon.to_owned()) {
                        Some(path) => return Some(path),
                        None => {}
                    }
                }
            }
        }
    }

    println!("Icon not found: {}", icon.to_owned());

    return None;
}

fn get_icon_with_extension(path: PathBuf, icon: String) -> Option<PathBuf> {
    let mut png_path = path.to_owned();
    let mut svg_path = path.to_owned();

    png_path.push(icon.to_owned() + ".svg");
    svg_path.push(icon.to_owned() + ".png");

    return if png_path.exists() {
        Some(png_path)
    } else if svg_path.exists() {
        Some(svg_path)
    } else {
        None
    };
}
