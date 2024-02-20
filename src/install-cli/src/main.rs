use std::{env, process::exit};

use whiskers_launcher_rs::paths::get_app_resources_dir;

//Imports only used in windows
#[cfg(target_os = "windows")]
use {is_elevated::is_elevated, std::io::stdin};

//Imports only used in linux
#[cfg(target_os = "linux")]
use {
    fs_extra::dir::CopyOptions,
    std::{fs, process::Command},
};

#[cfg(target_os = "windows")]
fn press_to_close() {
    let mut s = String::new();
    println!("Press any key to close ...");
    stdin().read_line(&mut s).unwrap();
    exit(0);
}

fn main() {
    let binary_path = env::current_exe().expect("Error getting path");
    let binary_dir = binary_path.parent().unwrap();

    let mut installation_files = binary_dir.to_owned();
    installation_files.push("installation-files");

    let mut logo = installation_files.to_owned();
    logo.push("whiskers-launcher.png");

    let mut icons_dir = installation_files.to_owned();
    icons_dir.push("resources");
    icons_dir.push("Icons");

    #[cfg(target_os = "linux")]
    if env::consts::OS == "linux" {
        let resources_dir = get_app_resources_dir().unwrap();

        let mut launcher_bin = installation_files.to_owned();
        launcher_bin.push("whiskers-launcher");

        let mut companion_bin = installation_files.to_owned();
        companion_bin.push("whiskers-launcher-companion");

        let mut desktop_file = installation_files.to_owned();
        desktop_file.push("whiskers-launcher.desktop");

        let copy_binaries_cmd = format!(
            "sudo cp '{}' '{}' /usr/bin",
            launcher_bin.into_os_string().into_string().unwrap(),
            companion_bin.into_os_string().into_string().unwrap()
        );

        println!("Copying files ...");

        let copy_binaries_result = Command::new("sh")
            .arg("-c")
            .arg(copy_binaries_cmd)
            .output()
            .expect("Error copying binaries");

        if !copy_binaries_result.status.success() {
            eprintln!(
                "Error while copying files: {}",
                String::from_utf8(copy_binaries_result.stderr).unwrap()
            );

            exit(1);
        }

        let copy_logo_cmd = format!(
            "sudo cp '{}' /usr/share/pixmaps",
            logo.into_os_string().into_string().unwrap()
        );

        let copy_logo_result = Command::new("sh")
            .arg("-c")
            .arg(copy_logo_cmd)
            .output()
            .expect("Error copying logo");

        if !copy_logo_result.status.success() {
            eprintln!(
                "Error copying logo: {}",
                String::from_utf8(copy_logo_result.stderr).unwrap()
            );

            exit(1);
        }

        let install_desktop_cmd = format!(
            "sudo desktop-file-install -m 644 --dir /usr/share/applications {}",
            desktop_file.into_os_string().into_string().unwrap()
        );

        let install_desktop_result = Command::new("sh")
            .arg("-c")
            .arg(install_desktop_cmd)
            .output()
            .expect("Error installing desktop file");

        if !install_desktop_result.status.success() {
            eprintln!(
                "Error installing desktop file: {}",
                String::from_utf8(install_desktop_result.stderr).unwrap()
            );

            exit(1);
        }

        if !&resources_dir.exists() {
            fs::create_dir_all(&resources_dir).expect("Error creating resources directory");
        }

        fs_extra::dir::copy(
            &icons_dir,
            &resources_dir,
            &CopyOptions::new().overwrite(true).to_owned(),
        )
        .expect("Error copying app icons");

        println!("Installation completed. Enjoy the launcher :D");
    }

    #[cfg(target_os = "windows")]
    if env::consts::OS == "windows" {
        if !is_elevated() {
            eprintln!("Please run the install script as administrator");
            press_to_close();
        }

        let mut install_script = include_str!("windows-install.ps1").to_owned();
        install_script = install_script.replace(
            "%installation_files_dir%",
            &installation_files.into_os_string().into_string().unwrap(),
        );

        match powershell_script::run(&install_script) {
            Ok(_) => {
                println!("Installation completed. Enjoy the launcher :D");
            }
            Err(e) => {
                eprintln!("Error running install script: {}", e.to_string());
            }
        };

        press_to_close();
    }
}
