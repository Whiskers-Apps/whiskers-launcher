use sysinfo::System;
#[cfg(target_os = "linux")]
use whiskers_launcher_rs::paths::{get_app_dir, get_autostart_dir};
use whiskers_launcher_rs::paths::{get_app_dir, get_indexing_dir};

use std::fs;
//Imports only used in windows
#[cfg(target_os = "windows")]
use std::{io::stdin, process::exit};

//Imports only used in linux
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
fn press_to_close() {
    let mut s = String::new();
    println!("\nPress enter to close");
    stdin().read_line(&mut s).unwrap();
    exit(0);
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        let app_dir = get_app_dir();

        let remove_binary_files_command = String::from("sudo rm -f /usr/bin/whiskers-launcher /usr/bin/whiskers-launcher-companion /usr/share/pixmaps/whiskers-launcher.png /usr/share/applications/whiskers-launcher.desktop");

        let remove_binary_files_result = Command::new("sh")
            .arg("-c")
            .arg(remove_binary_files_command)
            .output()
            .expect("Error removing file");

        if !remove_binary_files_result.status.success() {
            println!(
                "Error removing files: {}",
                String::from_utf8(remove_binary_files_result.stderr).unwrap()
            );
        }

        let mut auto_start_file = get_autostart_dir();
        auto_start_file.push("whiskers-launcher.desktop");

        if auto_start_file.exists() {
            fs::remove_file(&auto_start_file).expect("Error deleting autostart file");
        }

        if app_dir.exists() {
            fs::remove_dir_all(&app_dir).expect("Error removing local folder");
        }

        println!("Uninstalled");
    }

    #[cfg(target_os = "windows")]
    {
        for process in System::new_all().processes_by_name("whiskers-launcher") {
            process.kill();
        }

        let app_dir = get_app_dir();
        let home_launcher_dir = get_indexing_dir().parent().unwrap().to_owned();

        if app_dir.exists() {
            fs::remove_dir_all(app_dir).expect("Error removing app dir");
        } 

        if home_launcher_dir.exists() {
            fs::remove_dir_all(home_launcher_dir).expect("Error removing home config dir");
        }

        println!("✅ Uninstalled");

        press_to_close();
    }
}
