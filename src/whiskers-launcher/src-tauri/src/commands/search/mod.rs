use std::{fs, path::Path, process::Command};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tauri::{AppHandle, Manager, Window, WindowBuilder};

use tokio::time::sleep;
use whiskers_launcher_rs::{
    action::{
        self, Action, CopyAction, DialogAction, ExtensionAction, OpenAppAction, OpenURLAction,
    },
    api::{
        apps::get_apps,
        extensions::{
            get_extension_dir, get_extension_response, write_dialog_request, write_dialog_response,
            write_extension_request, ActionContext, DialogResponse, DialogResult, ExtensionRequest,
        },
        settings,
    },
    indexing::App,
    paths::{get_api_dir, get_app_resources_icons_dir, get_recent_apps_path},
    result::{self, TextResult, WLResult},
    settings::SearchEngine,
    utils::get_search,
};

#[tauri::command]
pub async fn get_results(text: String) -> Vec<WLResult> {
    let settings = settings::get_settings();
    let mut results = Vec::<WLResult>::new();

    if text.is_empty() {
        if settings.show_recent_apps {
            let recent_apps = get_recent_apps();

            for app in recent_apps {
                let open_app_action = Action::new_open_app(OpenAppAction::new(&app.id));
                let mut text_result = TextResult::new(&app.title, open_app_action);

                if app.icon.is_some() {
                    text_result.icon(app.icon.unwrap());
                } else {
                    let mut icon_path = get_app_resources_icons_dir();
                    icon_path.push("question.svg");

                    text_result.icon(
                        icon_path
                            .into_os_string()
                            .into_string()
                            .expect("Error getting icon path"),
                    );
                    text_result.tint("accent");
                }

                results.push(WLResult::new_text(text_result))
            }
        }

        return results;
    }

    let search = get_search(&text);
    let keyword = search.keyword;

    // Search extensions and search engines
    if keyword.is_some() {
        for engine in &settings.search_engines {
            if engine.keyword == keyword.to_owned().unwrap() {
                results.push(get_engine_result(engine.to_owned(), &search.search_text));
                return results;
            }
        }

        for extension_setting in settings.extensions {
            if extension_setting.setting_id == "keyword"
                && extension_setting.setting_value == keyword.to_owned().unwrap()
            {
                let request = ExtensionRequest::new(
                    &extension_setting.extension_id,
                    ActionContext::ResultsRequest,
                )
                .search_text(&search.search_text);

                if !get_api_dir().exists() {
                    fs::create_dir_all(get_api_dir()).expect("Error creating api directory");
                }

                write_extension_request(request);

                let extension_dir = get_extension_dir(&extension_setting.extension_id)
                    .expect("Error getting extension directory");

                if cfg!(target_os = "linux") {
                    let extension_run = Command::new("sh")
                        .arg("-c")
                        .arg("./linux-extension")
                        .current_dir(&extension_dir)
                        .output()
                        .expect("Error running extension");

                    if extension_run.status.success() {
                        return get_extension_response().results;
                    }
                }

                #[cfg(target_os = "windows")]
                if cfg!(target_os = "windows") {
                    let extension_run = Command::new("cmd")
                        .arg("/C")
                        .arg("start /min windows-extension")
                        .current_dir(&extension_dir)
                        .creation_flags(FLAG_NO_WINDOW)
                        .output()
                        .unwrap();

                    if extension_run.status.success() {
                        return get_extension_results().expect("Error getting extension results");
                    }
                }
            }
        }
    }

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
                } else {
                    let mut icon_path = get_app_resources_icons_dir();
                    icon_path.push("question.svg");

                    text_result.icon(
                        icon_path
                            .into_os_string()
                            .into_string()
                            .expect("Error getting icon path"),
                    );
                    text_result.tint("accent");
                }

                results.push(WLResult::new_text(text_result))
            }
        }
    }

    if results.is_empty() {
        for engine in &settings.search_engines {
            if engine.id == settings.default_search_engine {
                results.push(get_engine_result(engine.to_owned(), &text));
            }
        }
    }

    results
}

fn get_engine_result(engine: SearchEngine, search_text: impl Into<String>) -> WLResult {
    let search_text = search_text.into();
    let url = engine.search_query.replace("%s", &search_text);
    let action = Action::new_open_url(OpenURLAction::new(url));
    let result_text = format!("Search on {} for {}", &engine.name, &search_text);
    let mut search_icon_path = get_app_resources_icons_dir();
    search_icon_path.push("search.svg");

    let mut result = TextResult::new(result_text, action);

    if engine.icon_path.is_none() {
        result.icon(
            search_icon_path
                .into_os_string()
                .into_string()
                .expect("Error converting path to string"),
        );

        result.tint("accent");
    } else {
        result.icon(engine.icon_path.unwrap());

        if engine.tint_icon {
            result.tint("accent");
        }
    }

    WLResult::new_text(result)
}

#[tauri::command]
pub async fn run_action(result: WLResult, window: Window, app: AppHandle) {
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
        action::ActionType::OpenURL => open_url(action.open_url.unwrap(), window.to_owned()),
        action::ActionType::Copy => {
            copy_text(action.copy.unwrap(), window.to_owned(), app.to_owned())
        }
        action::ActionType::Extension => {
            run_extension_action(action.extension.unwrap(), window.to_owned())
        }
        action::ActionType::Dialog => {
            open_dialog(action.dialog.unwrap(), window.to_owned(), app.to_owned()).await
        }

        action::ActionType::Ignore => {}
    };
}

fn get_app(id: String) -> App {
    get_apps().iter().find(|a| a.id == id).unwrap().to_owned()
}

fn open_app(action: OpenAppAction, window: Window) {
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

    // Adds app to the recent apps list

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

    window.close().expect("Error closing window");
}

fn get_recent_apps() -> Vec<App> {
    let recent_apps_path = get_recent_apps_path();
    let recent_apps_bytes = fs::read(&recent_apps_path);

    match recent_apps_bytes {
        Ok(bytes) => bincode::deserialize(&bytes).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn open_url(action: OpenURLAction, window: Window) {
    open::that(&action.url).expect("Error opening url");
    window.close().expect("Error closing window");
}

fn copy_text(action: CopyAction, window: Window, app: AppHandle) {
    let clipboard = app.state::<tauri_plugin_clipboard::ClipboardManager>();

    clipboard
        .write_text(action.text)
        .expect("Error writing to clipboard");

    window.close().expect("Error closing window");
}

fn run_extension_action(action: ExtensionAction, window: Window) {
    let extension_dir =
        get_extension_dir(&action.extension_id).expect("Error getting extension dir");

    let mut request = ExtensionRequest::new(&action.extension_id, ActionContext::RunAction)
        .extension_action(action.action);

    if action.args.is_some() {
        request.args(action.args.unwrap());
    }

    write_extension_request(request);

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./linux-extension")
            .current_dir(&extension_dir)
            .output()
            .expect("Error running extension");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let extension_run = Command::new("cmd")
            .arg("/C")
            .arg("start /min windows-extension")
            .current_dir(&extension_dir)
            .creation_flags(FLAG_NO_WINDOW)
            .output()
            .unwrap();
    }

    window.close().expect("Error closing window");
}

async fn open_dialog(action: DialogAction, window: Window, app: AppHandle) {
    write_dialog_request(action.to_owned());

    WindowBuilder::new(
        &app,
        "extension-dialog",
        tauri::WindowUrl::App("/dialogs/extension-dialog".parse().unwrap()),
    )
    .title(&action.title)
    .inner_size(800.0, 700.0)
    .max_inner_size(800.0, 700.0)
    .fullscreen(false)
    .maximizable(false)
    .center()
    .build()
    .expect("Error opening dialog window");

    sleep(tokio::time::Duration::from_millis(200)).await;

    window.close().expect("Error closing window");
}

#[tauri::command]
pub async fn get_dialog_request() -> DialogAction {
    whiskers_launcher_rs::api::extensions::get_dialog_request()
}

#[tauri::command]
pub async fn run_dialog_action(action: DialogAction, results: Vec<DialogResult>, window: Window) {
    let response = DialogResponse {
        results,
        args: action.args,
    };

    write_dialog_response(response);

    let extension_dir =
        get_extension_dir(&action.extension_id).expect("Error getting extension dir");

    let request = ExtensionRequest::new(&action.extension_id, ActionContext::RunAction)
        .extension_action(action.action);

    write_extension_request(request);

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./linux-extension")
            .current_dir(&extension_dir)
            .output()
            .expect("Error running extension");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let extension_run = Command::new("cmd")
            .arg("/C")
            .arg("start /min windows-extension")
            .current_dir(&extension_dir)
            .creation_flags(FLAG_NO_WINDOW)
            .output()
            .unwrap();
    }

    window.close().expect("Error closing window");
}
