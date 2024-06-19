#[cfg(target_os = "linux")]
use std::{process::Command, thread};

#[cfg(target_os = "windows")]
use {
    std::os::windows::process::CommandExt,
    std::process::Command,
    std::thread,
    whiskers_launcher_rs::{paths::get_app_dir, utils::FLAG_NO_WINDOW},
};

pub fn open_launcher_window() {
    thread::spawn(|| {
        #[cfg(target_os = "linux")]
        let _ = Command::new("sh")
            .arg("-c")
            .arg("WEBKIT_DISABLE_COMPOSITING_MODE=1 whiskers-launcher")
            .output()
            .expect("Error opening launcher");

        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .arg("/C")
            .arg("start whiskers-launcher")
            .current_dir(get_app_dir())
            .creation_flags(FLAG_NO_WINDOW)
            .spawn()
            .expect("Error opening app");
    });
}

pub fn open_settings_window() {
    thread::spawn(|| {
        #[cfg(target_os = "linux")]
        let _ = Command::new("sh")
            .arg("-c")
            .arg("WEBKIT_DISABLE_COMPOSITING_MODE=1 whiskers-launcher --settings")
            .output()
            .expect("Error opening app");

        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .arg("/C")
            .arg("start whiskers-launcher --settings")
            .current_dir(get_app_dir())
            .creation_flags(FLAG_NO_WINDOW)
            .spawn()
            .expect("Error opening app");
    });
}
