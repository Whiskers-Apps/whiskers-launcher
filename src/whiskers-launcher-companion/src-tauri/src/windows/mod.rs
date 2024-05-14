use std::{process::Command, thread};

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
                .current_dir(get_local_dir().unwrap())
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
            .current_dir(get_local_dir().unwrap())
            .creation_flags(FLAG_NO_WINDOW)
            .spawn()
            .expect("Error opening app");
    });
}
