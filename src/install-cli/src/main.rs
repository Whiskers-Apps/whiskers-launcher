use std::{env, fs, process::{Command, exit}};

use fs_extra::dir::CopyOptions;
use simple_kl_rs::paths::{get_local_dir, get_resources_directory};

fn main() {
    let binary_path = env::current_exe().expect("Error getting path");
    let binary_dir = binary_path.parent().unwrap();

    let mut installation_files = binary_dir.to_owned();
    installation_files.push("installation-files");

    let local_dir = get_local_dir().unwrap();

    let resources_dir = get_resources_directory().unwrap();

    let mut logo = installation_files.to_owned();
    logo.push("simple-kl.png");

    let mut icons_dir = installation_files.to_owned();
    icons_dir.push("resources");
    icons_dir.push("icons");

    #[cfg(target_os = "linux")]
    if env::consts::OS == "linux" {
        let mut launcher_bin = installation_files.to_owned();
        launcher_bin.push("simple-keyboard-launcher");

        let mut service_bin = installation_files.to_owned();
        service_bin.push("simple-kl-service");

        let mut desktop_file = installation_files.to_owned();
        desktop_file.push("simple-kl.desktop");


        let copy_binaries_cmd = format!(
            "sudo cp '{}' '{}' /usr/bin",
            launcher_bin.into_os_string().into_string().unwrap(),
            service_bin.into_os_string().into_string().unwrap()
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

        println!("Installation completed. Enjoy the launcher :D");
    }

    #[cfg(target_os = "windows")]
    if env::consts::OS == "windows" {}

    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).expect("Error creating resources directory");
    }

    fs_extra::dir::copy(
        &icons_dir,
        &resources_dir,
        &CopyOptions::new().overwrite(true).to_owned(),
    )
    .expect("Error copying app icons");
}
