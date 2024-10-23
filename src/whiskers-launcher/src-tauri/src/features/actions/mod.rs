use std::{fs, path::Path, process::Command, thread, time::Duration};

use tauri::{AppHandle, Manager, Window, WindowBuilder};
use whiskers_launcher_core::{
    features::{
        core::{
            apps::{get_apps, App},
            extensions::{write_extension_request, write_form_request},
        },
        extensions::{get_extension_dir, ExtensionRequest},
    },
    paths::get_recent_apps_path,
    results::{
        CopyImageAction, CopyTextAction, OpenAppAction, OpenFormAction, OpenLinkAction,
        ResultAction, RunExtensionAction,
    },
    utils::on_linux,
};

#[cfg(target_os = "windows")]
use {
    std::os::windows::process::CommandExt,
    whiskers_launcher_core::utils::FLAG_NO_WINDOW
};

#[cfg(target_os = "linux")] 
use whiskers_launcher_core::utils::{on_hyprland, on_wayland};

use crate::get_recent_apps;

 #[tauri::command]
pub fn run_action(action: ResultAction, app: AppHandle, window: Window) {
    thread::spawn(move || match action.action_type {
        whiskers_launcher_core::results::ActionType::CopyText => {
            copy_text(action.copy_text_action.unwrap(), &app);
        }
        whiskers_launcher_core::results::ActionType::CopyImage => {
            copy_image(action.copy_image_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::OpenLink => {
            open_link(action.open_link_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::OpenApp => {
            open_app(action.open_app_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::OpenForm => {
            open_form(action.open_form_action.unwrap(), &app);
        }
        whiskers_launcher_core::results::ActionType::RunExtension => {
            run_extension_action(action.run_extension_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::DoNothing => {}
    })
    .join()
    .expect("Error running action in thread");

    window.close().expect("Error closing window");
}

fn copy_text(action: CopyTextAction, app: &AppHandle) {
    let clipboard = app.state::<tauri_plugin_clipboard::ClipboardManager>();
    clipboard
        .write_text(action.text.to_owned())
        .expect("Error copying text");

    #[cfg(target_os = "linux")]
    {
        if on_hyprland() {
            Command::new("sh")
                .arg("-c")
                .arg(format!("wl-copy '{}'", &action.text.to_owned()))
                .spawn()
                .expect("Error writing to clipboard");
        }
    }
}

fn copy_image(action: CopyImageAction) {
    #[cfg(target_os = "linux")]
    {
        if on_wayland() {
            if on_hyprland() {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "cat {} | wl-copy --type image/png",
                        &action.image_path
                    ))
                    .spawn()
                    .expect("Error copying image");
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let script = r#"
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile("%path%"))"#
            .replace("%path%", &action.image_path);

        powershell_script::run(&script).expect("Error executing copy command");
    }
}

fn open_link(action: OpenLinkAction) {
    // Explanation: Needs to be on a thread because there's a chance that it will open a browser and then the app freezes
    thread::spawn(move || {
        open::that(action.link).expect("Error opening link");
    });
}

fn open_app(action: OpenAppAction) {
    let app: App = get_apps()
        .iter()
        .find(|a| a.id == action.app_id)
        .unwrap()
        .to_owned();

    #[cfg(target_os = "linux")]
    {
        let desktop_file_dir = Path::new(&app.path)
            .parent()
            .expect("Error reading parent directory")
            .to_owned();

        let desktop_file_name = Path::new(&app.path)
            .file_name()
            .expect("Error getting app file name")
            .to_owned();

        Command::new("gtk-launch")
            .arg(desktop_file_name)
            .current_dir(desktop_file_dir)
            .spawn()
            .expect("Error opening app");
    }

    #[cfg(target_os = "windows")]
    {
        let path = Path::new(&app.path).to_owned();

        let script = format!(
            "invoke-item '{}'",
            path.into_os_string().into_string().unwrap()
        );

        powershell_script::run(&script).expect("Error opening app");
    }

    if on_linux() {
        let mut recent_apps = get_recent_apps();

        recent_apps = recent_apps
            .iter()
            .cloned()
            .filter(|a| a.id != app.id)
            .collect();

        recent_apps.insert(0, app.to_owned());

        let cut_size = if recent_apps.len() < 14 {
            recent_apps.len()
        } else {
            14
        };
        let recent_apps = &recent_apps[0..cut_size].to_owned();

        if !get_recent_apps_path()
            .parent()
            .expect("Error getting parent")
            .exists()
        {
            fs::create_dir_all(&get_recent_apps_path().parent().unwrap())
                .expect("Error creating recent apps dir");
        }

        fs::write(
            &get_recent_apps_path(),
            bincode::serialize(recent_apps).expect("Error serrialing recent apps"),
        )
        .expect("Error writing recent apps");
    }
}

fn open_form(action: OpenFormAction, app: &AppHandle) {
    write_form_request(action.to_owned());

    WindowBuilder::new(
        app,
        "form",
        tauri::WindowUrl::App("/form".parse().unwrap()),
    )
    .title(&action.title)
    .inner_size(900.0, 800.0)
    .max_inner_size(900.0, 800.0)
    .fullscreen(false)
    .maximizable(false)
    .center()
    .build()
    .expect("Error opening dialog window");

    thread::sleep(Duration::from_millis(300));
}

#[tauri::command]
pub fn run_extension_action(action: RunExtensionAction) {
    thread::spawn(move || {
        let extension_dir =
            get_extension_dir(&action.extension_id).expect("Error getting extension dir");

        let request =
            ExtensionRequest::new_run_command_request(&action.extension_id, &action.command)
                .set_args(action.args);

        write_extension_request(request);

        #[cfg(target_os = "linux")]
        {
            Command::new("sh")
                .arg("-c")
                .arg("./linux-extension")
                .current_dir(&extension_dir)
                .spawn()
                .expect("Error running extension");
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .arg("/C")
                .arg("start /min windows-extension.exe")
                .current_dir(&extension_dir)
                .creation_flags(FLAG_NO_WINDOW)
                .spawn()
                .expect("Error running extension");
        }
    });
}
