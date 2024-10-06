use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
};

use whiskers_launcher_core::{
    features::core::apps::{get_apps, App},
    paths::get_recent_apps_path,
    results::{
        CopyImageAction, CopyTextAction, OpenAppAction, OpenFormAction, OpenLinkAction,
        ResultAction, RunExtensionAction,
    },
    utils::{on_linux, on_windows},
};

pub fn cli_run_action(action_json: &str) {
    let action: ResultAction = serde_json::from_str(action_json).expect("Error parsing action");

    thread::spawn(move || match action.action_type {
        whiskers_launcher_core::results::ActionType::CopyText => {
            copy_text(action.copy_text_action.unwrap());
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
            open_form(action.open_form_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::RunExtension => {
            run_extension(action.run_extension_action.unwrap());
        }
        whiskers_launcher_core::results::ActionType::DoNothing => {}
    })
    .join()
    .expect("Error running action in thread");
}

fn copy_text(action: CopyTextAction) {}

fn copy_image(action: CopyImageAction) {}

fn open_link(action: OpenLinkAction) {
    open::that(action.link).expect("Error opening link");
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

fn open_form(action: OpenFormAction) {}

fn run_extension(action: RunExtensionAction) {}

fn get_recent_apps() -> Vec<App> {
    match fs::read(&get_recent_apps_path()) {
        Ok(bytes) => bincode::deserialize(&bytes).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}
