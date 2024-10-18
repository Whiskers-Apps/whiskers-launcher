use std::{fs, path::PathBuf};

use eval::eval;
use sniffer_rs::sniffer::Sniffer;

use whiskers_launcher_core::{
    features::{
        core::{
            apps::{get_apps, App},
            extensions::write_extension_request,
            settings::{get_settings, SearchEngine},
        },
        extensions::{get_extension_dir, ExtensionRequest},
    },
    paths::{get_api_dir, get_app_resources_icons_dir, get_recent_apps_path},
    results::{
        CopyTextAction, OpenAppAction, OpenLinkAction, ResultAction, SearchResult, SearchResults,
    },
    utils::{get_search_query, on_windows},
};

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
use {
    std::os::windows::process::CommandExt, std::process::Command,
    whiskers_launcher_rs::utils::FLAG_NO_WINDOW,
};

#[tauri::command]
pub fn run_wallpaper_exists(path: PathBuf) -> bool {
    return path.exists();
}

#[tauri::command(rename_all = "snake_case")]
pub fn run_get_search_results(search_text: &str) -> SearchResults {
    let settings = get_settings();
    let mut results = Vec::<SearchResult>::new();
    let show_apps_as_grid = settings.show_apps_as_grid;

    if search_text.is_empty() {
        if on_windows() {
            return SearchResults::new_list_results(vec![]);
        }

        if settings.show_recent_apps {
            let recent_apps = get_recent_apps();

            for app in recent_apps {
                let open_app_action =
                    ResultAction::new_open_app_action(OpenAppAction::new(&app.id));

                let icon = if let Some(icon) = &app.icon {
                    PathBuf::from(icon)
                } else {
                    let mut icon_path = get_app_resources_icons_dir();
                    icon_path.push("question.svg");
                    icon_path
                };

                let mut text_result = SearchResult::new(&app.title, open_app_action).set_icon(icon);

                if !app.icon.is_some() {
                    text_result = text_result.set_accent_icon_tint()
                }

                results.push(text_result)
            }
        }

        return if show_apps_as_grid {
            SearchResults::new_grid_results(results)
        } else {
            SearchResults::new_list_results(results)
        };
    }

    if search_text.trim() == "*" {
        let app_results: Vec<SearchResult> = get_apps()
            .iter()
            .map(|a| get_app_result(a.clone()))
            .collect();

        return if show_apps_as_grid {
            SearchResults::new_grid_results(app_results)
        } else {
            SearchResults::new_list_results(app_results)
        };
    }

    let search_query = get_search_query(search_text);
    let keyword = search_query.keyword;

    // Search extensions and search engines
    if let Some(keyword) = keyword {
        if keyword == settings.search_keyword {
            if let Some(engine) = settings
                .search_engines
                .iter()
                .find(|e| e.id == settings.default_search_engine)
            {
                results.push(get_engine_result(
                    engine.to_owned(),
                    &search_query.search_text,
                ));

                return SearchResults::new_list_results(results);
            }
        }

        if let Some(engine) = settings
            .search_engines
            .iter()
            .find(|e| e.keyword == keyword)
        {
            results.push(get_engine_result(
                engine.to_owned(),
                &search_query.search_text,
            ));
            return SearchResults::new_list_results(results);
        }

        for extension_setting in settings.extensions {
            if extension_setting.setting_id == "keyword"
                && extension_setting.setting_value == keyword.to_owned()
            {
                let request = ExtensionRequest::new_get_results_request(
                    &extension_setting.extension_id,
                    &search_query.search_text,
                );

                if !get_api_dir().exists() {
                    fs::create_dir_all(get_api_dir()).expect("Error creating api directory");
                }

                write_extension_request(request);

                let extension_dir = get_extension_dir(&extension_setting.extension_id)
                    .expect("Error getting extension directory");

                #[cfg(target_os = "linux")]
                {
                    let extension_run = Command::new("sh")
                        .arg("-c")
                        .arg("./linux-extension")
                        .current_dir(&extension_dir)
                        .output()
                        .expect("Error running extension");

                    if extension_run.status.success() {
                        let json: String =
                            String::from_utf8_lossy(&extension_run.stdout).to_string();

                        let extension_results: SearchResults =
                            serde_json::from_str(&json).expect("Error parsing extension results");

                        return extension_results;
                    }
                }

                #[cfg(target_os = "windows")]
                {
                    let extension_run = Command::new("cmd")
                        .arg("/C")
                        .arg("start /min windows-extension.exe")
                        .current_dir(&extension_dir)
                        .creation_flags(FLAG_NO_WINDOW)
                        .status()
                        .expect("Error running extension");

                    if extension_run.success() {
                        let json: String =
                            String::from_utf8_lossy(&extension_run.stdout).to_string();

                        let extension_results: SearchResults =
                            serde_json::from_str(&json).expect("Error parsing extension results");

                        return extension_results;
                    }
                }
            }
        }
    }

    let sniffer = Sniffer::new();

    let apps = get_apps();
    let blacklist = settings.blacklist;

    for app in apps {
        if !blacklist.contains(&app.id) {
            if sniffer
                .clone()
                .matches(&app.title, &search_query.search_text)
            {
                results.push(get_app_result(app))
            }
        }
    }

    if results.is_empty() {
        if let Ok(result) = eval(search_text) {
            if result.is_number() {
                let mut calc_icon = get_app_resources_icons_dir();
                calc_icon.push("calculator.svg");

                results.push(
                    SearchResult::new(
                        result.to_string(),
                        ResultAction::new_copy_text_action(CopyTextAction::new(result.to_string())),
                    )
                    .set_icon(calc_icon)
                    .set_accent_icon_tint(),
                );

                return SearchResults::new_list_results(results);
            }
        }

        for engine in &settings.search_engines {
            if engine.id == settings.default_search_engine {
                results.push(get_engine_result(engine.to_owned(), search_text));
            }
        }
    }

    return if show_apps_as_grid {
        SearchResults::new_grid_results(results)
    } else {
        SearchResults::new_list_results(results)
    };
}

fn get_app_result(app: App) -> SearchResult {
    let open_app_action = ResultAction::new_open_app_action(OpenAppAction::new(&app.id));

    let icon_path = if let Some(icon_path) = &app.icon {
        PathBuf::from(icon_path)
    } else {
        let mut icon_path = get_app_resources_icons_dir();
        icon_path.push("question.svg");
        icon_path
    };

    let mut result = SearchResult::new(&app.title, open_app_action).set_icon(icon_path);

    if app.icon.is_none() {
        result = result.set_accent_icon_tint()
    }

    result
}

fn get_engine_result(engine: SearchEngine, search_text: impl Into<String>) -> SearchResult {
    let search_text = search_text.into();
    let link = engine.search_query.replace("%s", &search_text);
    let action = ResultAction::new_open_link_action(OpenLinkAction::new(link));
    let mut search_icon_path = get_app_resources_icons_dir();
    search_icon_path.push("search.svg");

    let icon_path = if let Some(icon_path) = &engine.icon_path {
        PathBuf::from(icon_path)
    } else {
        search_icon_path
    };

    let mut result = SearchResult::new(&engine.name, action)
        .set_description(format!("Search for {}", &search_text))
        .set_icon(icon_path);

    if engine.icon_path.is_none() || engine.tint_icon {
        result = result.set_accent_icon_tint()
    }

    result
}

pub fn get_recent_apps() -> Vec<App> {
    match fs::read(&get_recent_apps_path()) {
        Ok(bytes) => bincode::deserialize(&bytes).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}
