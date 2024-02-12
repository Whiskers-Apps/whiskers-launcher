use std::{fs, path::Path, process::Command};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tauri::{api::shell::open, AppHandle, Manager, Window, WindowBuilder, WindowUrl};
use whiskers_launcher_rs::{
    actions,
    api::extensions::{
        self, get_extension_dir, get_extension_results, send_extension_context, send_extension_dialog_action, send_extension_dialog_response, Context, DialogResponse, DialogResult
    },
    dialog::DialogField,
    indexing::get_indexed_apps,
    paths::{get_app_resources_icons_dir, get_temp_dir},
    results::{self, WhiskersResult},
    settings::get_settings,
};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_search_results(typed_text: String) -> Vec<WhiskersResult> {
    let mut results: Vec<WhiskersResult> = vec![];
    let settings = get_settings().unwrap();

    if typed_text.to_owned().is_empty() {
        return results;
    }

    if typed_text.contains("") {
        let mut keyword = "".to_string();
        let mut search_text = "".to_string();
        let mut has_keyword = false;

        for typed_char in typed_text.chars() {
            if typed_char == ' ' && !has_keyword {
                has_keyword = true;
            } else if !has_keyword {
                keyword += &typed_char.to_string();
            } else {
                search_text += &typed_char.to_string();
            }
        }

        search_text = search_text.trim().to_string();

        if has_keyword {
            let temp_dir = get_temp_dir().expect("Error getting temp dir");
            if !temp_dir.exists() {
                fs::create_dir_all(&temp_dir).expect("Error creating temp dir");
            }

            for extension in settings.extensions.to_owned() {
                if extension.keyword == keyword {
                    let path =
                        get_extension_dir(extension.id).expect("Could not find extension dir");

                    let context = Context::new(extensions::Action::GetResults)
                        .search_text(search_text.to_owned());

                    send_extension_context(context).expect("Error writing context");

                    if cfg!(target_os = "linux") {
                        let extension_run = Command::new("sh")
                            .arg("-c")
                            .arg("./extension")
                            .current_dir(&path)
                            .output()
                            .expect("Error running extension");

                        if extension_run.status.success() {
                            return get_extension_results()
                                .expect("Error getting extension results");
                        } else {
                            println!(
                                "Error running extension: {}",
                                String::from_utf8_lossy(&extension_run.stderr)
                            )
                        }
                    }

                    if cfg!(target_os = "windows") {}
                }
            }

            let mut default_search_icon_path =
                get_app_resources_icons_dir().expect("Error getting icons dir");
            default_search_icon_path.push("search.svg");

            for search_engine in settings.search_engines.to_owned() {
                if search_engine.keyword == keyword {
                    let url = search_engine.query.replace("%s", &search_text);
                    let url_action = actions::OpenUrl::new(url);

                    let text = format!("Search for {}", &typed_text);
                    let icon_path = match &search_engine.icon_path {
                        Some(path) => path.to_string(),
                        None => default_search_icon_path
                            .into_os_string()
                            .into_string()
                            .expect("Error getting search icon path"),
                    };

                    let text_result =
                        results::Text::new(text, actions::Action::OpenUrl(url_action))
                            .icon(icon_path)
                            .tint_icon(search_engine.tint_icon);

                    results.push(results::WhiskersResult::Text(text_result));
                    return results;
                }
            }
        }
    }

    let apps = get_indexed_apps().expect("Error getting apps");
    let matcher = SkimMatcherV2::default();
    let blacklist = settings.blacklist;

    for app in &apps {
        if !blacklist.contains(&app.exec_path) {
            if matcher.fuzzy_match(&app.name, &typed_text).is_some() {
                let open_app_action = actions::OpenApp::new(&app.exec_path);
                let icon_path = match &app.icon_path {
                    Some(path) => path.to_string(),
                    None => "".to_string(),
                };

                let text_result =
                    results::Text::new(&app.name, actions::Action::OpenApp(open_app_action))
                        .icon(icon_path)
                        .tint_icon(app.icon_path == None);

                results.push(WhiskersResult::Text(text_result));
            }
        }
    }

    // If no result is found, it uses the default search engine
    if results.len() == 0 {
        for search_engine in settings.search_engines.to_owned() {
            if search_engine.default {
                let url = search_engine.query.replace("%s", &typed_text);
                let url_action = actions::OpenUrl::new(url);
                let mut default_search_icon_path =
                    get_app_resources_icons_dir().expect("Error getting icons dir");
                default_search_icon_path.push("search.svg");

                let text = format!("Search for {}", &typed_text);
                let icon_path = match search_engine.icon_path {
                    Some(path) => path.to_string(),
                    None => default_search_icon_path
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                };

                let text_result = results::Text::new(text, actions::Action::OpenUrl(url_action))
                    .icon(icon_path)
                    .tint_icon(search_engine.tint_icon);

                results.push(results::WhiskersResult::Text(text_result));
                return results;
            }
        }
    }

    return results;
}

#[tauri::command(rename_all = "snake_case")]
pub fn run_extension_action(
    extension_id: String,
    extension_action: String,
    args: Option<Vec<String>>,
) {
    let path = get_extension_dir(extension_id).expect("Could not find extension dir");

    let mut context =
        Context::new(extensions::Action::RunAction).extension_action(extension_action);

    if args.is_some() {
        context.custom_args(args.unwrap());
    }

    send_extension_context(context).expect("Error writing context");

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./extension")
            .current_dir(&path)
            .output()
            .expect("Error running extension");
    }

    if cfg!(target_os = "windows") {}
}

#[tauri::command(rename_all = "snake_case")]
pub async fn open_app(exec_path: String, window: Window) {
    window.hide().unwrap();

    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let desktop_file_dir = Path::new(&exec_path)
            .parent()
            .expect("Error reading parent directory")
            .to_owned();

        let desktop_file_name = Path::new(&exec_path)
            .file_name()
            .expect("Error getting app file name")
            .to_owned();

        std::thread::spawn(|| {
            Command::new("gtk-launch")
                .arg(desktop_file_name)
                .current_dir(desktop_file_dir)
                .spawn()
                .expect("");
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
}

#[tauri::command]
pub async fn open_url(url: String, app: AppHandle) {
    open(&app.shell_scope(), url, None).expect("Error opening url");
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_extension_dialog(
    extension_id: String,
    extension_action: String,
    title: String,
    primary_button_text: Option<String>,
    fields: Vec<DialogField>,
    args: Option<Vec<String>>,
    app: AppHandle,
) {
    WindowBuilder::new(
        &app,
        "extension-dialog",
        WindowUrl::App("extension-dialog".into()),
    )
    .title(&title)
    .inner_size(800.0, 800.0)
    .resizable(false)
    .build()
    .expect("Error opening settings window");

    let mut action = actions::Dialog::new(extension_id, title, extension_action, fields);

    if primary_button_text.is_some() {
        action.primary_button_text(primary_button_text.unwrap());
    }

    if args.is_some() {
        action.args(args.unwrap());
    }

    send_extension_dialog_action(action);
}

#[tauri::command]
pub fn get_extension_dialog_action() -> actions::Dialog {
    return extensions::get_extension_dialog_action().expect("Error getting dialog action");
}

#[tauri::command(rename_all = "snake_case")]
pub fn close_extension_dialog(
    extension_id: String,
    extension_action: String,
    args: Option<Vec<String>>,
    results: Vec<DialogResult>,
    window: Window,
) {

    let response = DialogResponse::new(results, args);
    send_extension_dialog_response(response);

    let context = Context::new(extensions::Action::RunAction).extension_action(extension_action);
    send_extension_context(context).expect("Error writing extension context");

    let extension_dir = get_extension_dir(extension_id).expect("Could not find extension dir");

    if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("./extension")
            .current_dir(&extension_dir)
            .output()
            .expect("Error running extension");
    }

    if cfg!(target_os = "windows") {}
    window.close().unwrap();
}
