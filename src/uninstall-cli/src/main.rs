use std::{io::stdin, process::exit};

use is_elevated::is_elevated;

//Imports only used in linux
#[cfg(target_os = "linux")]
use {
    simple_kl_rs::paths::{get_autostart_path, get_local_dir},
    std::{fs, process::Command},
};

fn press_to_close() {
    let mut s = String::new();
    println!("Press any key to close ...");
    stdin().read_line(&mut s).unwrap();
    exit(0);
}

fn main() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let local_dir = get_local_dir().unwrap();

        let remove_root_files_cmd = format!("sudo rm -f /usr/bin/simple-keyboard-launcher /usr/bin/simple-kl-service /usr/share/pixmaps/simple-kl.png /usr/share/applications/simple-kl.desktop");

        let remove_root_files_result = Command::new("sh")
            .arg("-c")
            .arg(remove_root_files_cmd)
            .output()
            .expect("Error removing file");

        println!("Removing files ...");

        if !remove_root_files_result.status.success() {
            println!(
                "Error removing files: {}",
                String::from_utf8(remove_root_files_result.stderr).unwrap()
            );
        }

        let mut auto_start_file = get_autostart_path().unwrap();
        auto_start_file.push("simple-kl-service.desktop");

        if auto_start_file.exists() {
            fs::remove_file(&auto_start_file).expect("Error deleting autostart file");
        }

        //Removing files
        if local_dir.exists() {
            fs::remove_dir_all(&local_dir).expect("Error removing local folder");
        }

        println!("Uninstall successfull");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {

        if !is_elevated(){
            eprintln!("Please run the uninstall script as administrator");
            press_to_close();
        }

        let uninstall_script = include_str!("windows-uninstall.ps1").to_owned();

        match powershell_script::run(&uninstall_script) {
            Ok(_) => {
                println!("Uninstall successfull");
            }
            Err(e) => {
                eprintln!("Error running uninstall script: {}", e.to_string());
            }
        };

        press_to_close();
    }
}
