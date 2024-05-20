use std::{path::Path, process::Command};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tauri::Window;
use whiskers_launcher_rs::{
    action::{self, Action, OpenAppAction},
    api::{apps::get_apps, settings},
    indexing::App,
    result::{self, TextResult, WLResult},
    utils::get_search,
};

#[tauri::command]
pub async fn get_results(text: String) -> Vec<WLResult> {
    let settings = settings::get_settings();
    let mut results = Vec::<WLResult>::new();

    if text.is_empty() {
        if settings.show_recent_apps {}

        return results;
    }

    let search = get_search(&text);
    let matcher = SkimMatcherV2::default();

    let apps = get_apps();
    let blacklist = settings.blacklist;

    for app in apps {
        if !blacklist.contains(&app.id) {
            if matcher
                .fuzzy_match(&app.title, &search.search_text)
                .is_some()
            {
                let open_app_action = Action::new_open_app(OpenAppAction::new(&app.id));
                let mut text_result = TextResult::new(&app.title, open_app_action);

                if app.icon.is_some() {
                    text_result.icon(app.icon.unwrap());
                }

                results.push(WLResult::new_text(text_result))
            }
        }
    }

    results
}

#[tauri::command]
pub async fn run_action(result: WLResult, window: Window) {
    let result_action: Option<Action> = match result.result_type {
        result::ResultType::Text => Some(result.text.unwrap().action),
        result::ResultType::TitleAndDescription => {
            Some(result.title_and_description.unwrap().action)
        }
        result::ResultType::Divider => None,
    };

    if result_action.is_none() {
        return;
    }

    let action = result_action.unwrap();

    match action.action_type {
        action::ActionType::OpenApp => open_app(action.open_app.unwrap(), window.to_owned()),
        action::ActionType::OpenURL => {}
        action::ActionType::Copy => {}
        action::ActionType::Extension => {}
        action::ActionType::Dialog => {}
        action::ActionType::Ignore => {}
    };
}

fn get_app(id: String) -> App {
    get_apps().iter().find(|a| a.id == id).unwrap().to_owned()
}

pub fn open_app(action: OpenAppAction, window: Window) {
    let app = get_app(action.id);

    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let desktop_file_dir = Path::new(&app.path)
            .parent()
            .expect("Error reading parent directory")
            .to_owned();

        let desktop_file_name = Path::new(&app.path)
            .file_name()
            .expect("Error getting app file name")
            .to_owned();

        std::thread::spawn(|| {
            Command::new("gtk-launch")
                .arg(desktop_file_name)
                .current_dir(desktop_file_dir)
                .spawn()
                .expect("Error opening app");
        });
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let path = Path::new(&exec_path).to_owned();

        std::thread::spawn(move || {
            let script = format!(
                "invoke-item '{}'",
                path.into_os_string().into_string().unwrap()
            );

            powershell_script::run(&script).expect("Error opening app");
        });
    }

    window.close().expect("Error closing window");
}
