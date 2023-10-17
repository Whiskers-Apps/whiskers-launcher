use std::{fs, process::Command};

use simple_kl_rs::paths::{get_autostart_path, get_local_dir};

fn main() {
    let local_dir = get_local_dir().unwrap();

    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {

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
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {}

    //Removing files
    if local_dir.exists() {
        fs::remove_dir_all(&local_dir).expect("Error removing local folder");
    }

    println!("Uninstall successfull");
}
