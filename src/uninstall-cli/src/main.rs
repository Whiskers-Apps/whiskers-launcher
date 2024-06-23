#[cfg(target_os = "linux")]
use whiskers_launcher_rs::paths::{get_app_dir, get_autostart_dir};

//Imports only used in windows
#[cfg(target_os = "windows")]
use {
    is_elevated::is_elevated,
    std::{io::stdin, process::exit},
};

//Imports only used in linux
#[cfg(target_os = "linux")]
use std::{fs, process::Command};

#[cfg(target_os = "windows")]
fn press_to_close() {
    let mut s = String::new();
    println!("\nPress enter to close");
    stdin().read_line(&mut s).unwrap();
    exit(0);
}

fn main() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
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
    if cfg!(target_os = "windows") {
        if !is_elevated() {
            eprintln!("❌ Please run the script as administrator");
            press_to_close();
        }

        let uninstall_script = include_str!("windows-uninstall.ps1").to_owned();

        match powershell_script::run(&uninstall_script) {
            Ok(_) => {
                println!("✅ Uninstalled");
            }
            Err(error) => {
                eprintln!("❌ Error: {}", error.to_string());
            }
        };

        press_to_close();
    }
}
