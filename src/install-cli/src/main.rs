use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

//Imports only used in windows
#[cfg(target_os = "windows")]
use {mslnk::ShellLink, std::io::stdin, std::path::Path, std::process::exit};

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use rust_embed::Embed;
use sysinfo::System;
use whiskers_launcher_rs::paths::{
    get_app_dir, get_app_resources_dir, get_app_resources_icons_dir,
};

//Imports only used in linux

#[cfg(target_os = "windows")]
fn press_to_close() {
    let mut s = String::new();
    println!("\nPress enter to close");
    stdin().read_line(&mut s).unwrap();
    exit(0);
}

pub fn is_wayland() -> bool {
    match env::var("XDG_SESSION_TYPE") {
        Ok(session) => &session.to_lowercase() == "wayland",
        Err(_) => false,
    }
}

pub fn write_file(path: PathBuf, bytes: &[u8]) {
    let mut file = File::create(&path).expect("Error creating file");
    file.write_all(bytes).expect("Error writing file");
}

#[cfg(target_os = "linux")]
#[derive(Embed)]
#[folder = "files/linux/binaries"]
struct LinuxBinaries;

#[cfg(target_os = "linux")]
#[derive(Embed)]
#[folder = "files/linux/images"]
struct LinuxImages;

#[cfg(target_os = "windows")]
#[derive(Embed)]
#[folder = "files/windows/binaries/"]
struct WindowsBinaries;

#[cfg(target_os = "windows")]
#[derive(Embed)]
#[folder = "files/windows/scripts/"]
struct WindowsScripts;

#[derive(Embed)]
#[folder = "files/general/icons/"]
struct Icons;

#[cfg(target_os = "linux")]
fn run_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Error running command");
}

fn main() {
    for process in System::new_all().processes_by_name("whiskers-launcher") {
        process.kill();
    }

    let app_dir = get_app_dir();
    let resources_dir = get_app_resources_dir();
    let icons_dir = get_app_resources_icons_dir();

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Error creating app dir");
    }

    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).expect("Error creating resources dir");
    }

    if !icons_dir.exists() {
        fs::create_dir_all(&icons_dir).expect("Error creating icons dir");
    }

    #[cfg(target_os = "linux")]
    if env::consts::OS == "linux" {
        let install_temp_dir = PathBuf::from("/tmp/whiskers-launcher-installation");

        if !install_temp_dir.exists() {
            fs::create_dir_all(&install_temp_dir).expect("Error creating install temp dir");
        }

        for file in LinuxBinaries::iter() {
            if let Some(content) = LinuxBinaries::get(&file) {
                let mut path = install_temp_dir.to_owned();
                path.push(file.to_string());

                write_file(path.to_owned(), content.data.as_ref());

                let mut perms = fs::metadata(&path)
                    .expect("Error getting permissions")
                    .permissions();

                perms.set_mode(0o111);

                fs::set_permissions(&path, perms).expect("Error giving permissions");
            }
        }

        for file in LinuxImages::iter() {
            if let Some(content) = LinuxImages::get(&file) {
                let mut path = install_temp_dir.to_owned();
                path.push(file.to_string());

                write_file(path.to_owned(), content.data.as_ref());
            }
        }

        let mut launcher_bin = install_temp_dir.to_owned();
        launcher_bin.push("whiskers-launcher");

        let mut companion_bin = install_temp_dir.to_owned();
        companion_bin.push("whiskers-launcher-companion");

        let mut desktop_file = install_temp_dir.to_owned();
        desktop_file.push("whiskers-launcher.desktop");

        let mut logo = install_temp_dir.to_owned();
        logo.push("whiskers-launcher.png");

        let copy_binaries_cmd = format!(
            "sudo cp '{}' '{}' /usr/bin",
            launcher_bin.into_os_string().into_string().unwrap(),
            companion_bin.into_os_string().into_string().unwrap()
        );

        run_command(&copy_binaries_cmd);

        let copy_logo_cmd = format!(
            "sudo cp '{}' /usr/share/pixmaps",
            logo.into_os_string().into_string().unwrap()
        );

        run_command(&copy_logo_cmd);

        let install_desktop_cmd = format!(
            "sudo desktop-file-install -m 644 --dir /usr/share/applications {}",
            desktop_file.into_os_string().into_string().unwrap()
        );

        run_command(&install_desktop_cmd);

        fs::remove_dir_all(&install_temp_dir).expect("Error removing temporary directory");
    }

    #[cfg(target_os = "windows")]
    if env::consts::OS == "windows" {
        let mut scripts_dir = resources_dir.to_owned();
        scripts_dir.push("scripts");

        if !scripts_dir.exists() {
            fs::create_dir_all(&scripts_dir).expect("Error creating scripts dir");
        }

        for file in WindowsBinaries::iter() {
            if let Some(content) = WindowsBinaries::get(&file) {
                let mut path = app_dir.to_owned();
                path.push(file.to_string());

                write_file(path, content.data.as_ref());
            }
        }

        for file in WindowsScripts::iter() {
            if let Some(content) = WindowsScripts::get(&file) {
                let mut path = scripts_dir.to_owned();
                path.push(file.to_string());

                write_file(path, content.data.as_ref());
            }
        }

        // Create the shortcut
        let mut shortcut_path =
            Path::new(&env::var("APPDATA").expect("Error getting environment variable")).to_owned();

        shortcut_path.push("Microsoft\\Windows\\Start Menu\\Programs\\Whiskers-Launcher.lnk");

        let mut target_path = app_dir.to_owned();
        target_path.push("whiskers-launcher-companion.exe");

        let link = ShellLink::new(target_path.into_os_string().into_string().unwrap())
            .expect("Error initializing link");

        link.create_lnk(shortcut_path.into_os_string().into_string().unwrap())
            .expect("Error creating link");

        let mut companion_path = app_dir.to_owned();
        companion_path.push("whiskers-launcher-companion.exe");

        std::thread::spawn(move || {
            Command::new("cmd")
                .arg("/c")
                .arg(companion_path.into_os_string().into_string().unwrap())
                .spawn()
                .expect("Error opening companion app");
        });
    }

    for file in Icons::iter() {
        if let Some(content) = Icons::get(&file) {
            let mut path = icons_dir.to_owned();
            path.push(file.to_string());

            write_file(path, content.data.as_ref());
        }
    }

    println!("✅ Installed");

    #[cfg(target_os = "linux")]
    println!("To run the app launch the whiskers-launcher-companion first.");

    match is_wayland() {
        true => println!("⚠️ Wayland was detected. You need to manually make a shortcut for the app on your DE/WM. You can use 'whiskers-launcher' or 'WEBKIT_DISABLE_COMPOSITING_MODE=1 whiskers-launcher' if you problems with the app not launching"),
        false => println!("ℹ️ The default launch shortcut is 'ctrl + space'"),
    }

    #[cfg(target_os = "windows")]
    press_to_close();
}
