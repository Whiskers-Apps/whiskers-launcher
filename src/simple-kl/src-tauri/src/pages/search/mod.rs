use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use simple_kl_rs::actions::{DialogAction, ExtensionAction, OpenApp, OpenInBrowser, ResultAction};
use simple_kl_rs::extensions::{ExtensionManifest, Parameters};
use simple_kl_rs::paths::{get_apps_index_path, get_dialog_action_path, get_extension_parameters_path, get_extension_path, get_extension_results_path, get_extensions_path, get_resources_directory};
use simple_kl_rs::results::{IconWithTextResult, SimpleKLResult};
use simple_kl_rs::settings::get_settings;
use tauri::{AppHandle, Window, WindowBuilder, WindowUrl};

//Imports only used in windows
#[cfg(target_os = "windows")]
use {
    simple_kl_rs::others::FLAG_NO_WINDOW,
    simple_kl_rs::paths::get_local_dir,
    std::os::windows::process::CommandExt,
};

//Imports only used in Linux
#[cfg(target_os = "linux")]
use {
    crate::structs::structs::AppIndex
};

#[tauri::command(rename_all = "snake_case")]
pub fn get_results(search_text: String) -> Vec<SimpleKLResult> {
    let split_search: Vec<&str> = search_text.split_whitespace().collect();
    let mut keyword = String::from("");
    let mut search_words = String::from("");
    let search_engines = get_settings().search_engines;

    for (index, word) in split_search.iter().enumerate() {
        if index == 0 {
            keyword = word.to_string();
        } else {
            search_words = search_words + word + " "
        }
    }

    search_words = String::from(search_words.trim_end());

    let settings = get_settings();
    let extensions = settings.extensions;

    for extension in extensions {
        if extension.keyword == keyword {
            return get_extension_results(extension.id, search_words);
        }
    }

    let mut search_svg_path = get_resources_directory().unwrap();
    search_svg_path.push("images");
    search_svg_path.push("search.svg");

    for search_engine in search_engines {
        if search_engine.keyword == keyword {
            let url = search_engine.query.replace("%s", &search_words);
            let mut result: Vec<SimpleKLResult> = Vec::new();

            result.push(SimpleKLResult::IconWithText(
                match search_engine.tint_icon {
                    true => {
                        IconWithTextResult::new_with_color(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                                .to_owned(),
                            format!("Search for {}", search_words).as_str(),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        )
                    }
                    false => {
                        IconWithTextResult::new(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                                .to_owned(),
                            &format!("Search for {}", search_words),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        )
                    }
                },
            ));

            return result;
        }
    }

    let app_results = get_apps_results(&search_text);

    return if &app_results.len() > &0 {
        app_results
    } else {
        //Returns a search result
        for search_engine in get_settings().search_engines {
            if search_engine.default {
                let url = search_engine.query.replace("%s", &search_text);
                let mut result: Vec<SimpleKLResult> = Vec::new();

                result.push(SimpleKLResult::IconWithText(
                    match search_engine.tint_icon {
                        true => IconWithTextResult::new_with_color(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                                .to_path_buf(),
                            format!("Search for {}", search_text).as_str(),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        ),
                        false => IconWithTextResult::new(
                            Path::new(&search_engine.icon.unwrap_or(
                                search_svg_path.into_os_string().into_string().unwrap(),
                            ))
                                .to_owned(),
                            &format!("Search for {}", search_text),
                            ResultAction::OpenInBrowser(OpenInBrowser { url }),
                        ),
                    },
                ));

                return result;
            }
        }

        vec![]
    };
}

pub fn get_apps_results(search_text: &str) -> Vec<SimpleKLResult> {
    let mut results: Vec<SimpleKLResult> = Vec::new();

    let indexed_apps_yaml = fs::read_to_string(get_apps_index_path().unwrap()).unwrap();
    let apps: Vec<AppIndex> = serde_yaml::from_str(&indexed_apps_yaml).unwrap();

    for app in apps {
        if SkimMatcherV2::default()
            .fuzzy_match(&app.name, search_text)
            .is_some()
        {
            let action = OpenApp::new(app.exec_path);

            results.push(SimpleKLResult::IconWithText(IconWithTextResult::new(
                Path::new(&app.icon_path).to_owned(),
                &app.name,
                ResultAction::OpenApp(action),
            )));
        }
    }

    return results;
}

pub fn get_extension_results(id: String, search_text: String) -> Vec<SimpleKLResult> {
    let mut parameters_file = File::create(&get_extension_parameters_path().unwrap()).unwrap();
    let parameters = Parameters::new_get_results(search_text);
    let parameters_yaml =
        serde_yaml::to_string(&parameters).expect("Error converting parameters yaml");

    parameters_file
        .write_all(&parameters_yaml.as_bytes())
        .unwrap();

    parameters_file.flush().unwrap();

    if let Ok(folders) = fs::read_dir(&get_extensions_path().unwrap()) {
        for folder in folders {
            if let Ok(folder) = folder {
                let folder_path = folder.path().into_os_string().into_string().unwrap();
                let manifest_file_path = &format!("{}/manifest.yml", folder_path);

                if let Ok(mut manifest_file) = File::open(manifest_file_path) {
                    let mut manifest_yaml = String::from("");
                    manifest_file.read_to_string(&mut manifest_yaml).unwrap();

                    let manifest: ExtensionManifest = serde_yaml::from_str(&manifest_yaml).unwrap();

                    if manifest.id == id {
                        if cfg!(target_os = "linux") {
                            let extension_run = Command::new("sh")
                                .arg("-c")
                                .arg("./extension")
                                .current_dir(&folder_path)
                                .output()
                                .expect("Error running extension");

                            if extension_run.status.success() {
                                let mut extension_results_yaml = String::from("");
                                let mut extensions_results_file =
                                    File::open(&get_extension_results_path().unwrap()).unwrap();

                                extensions_results_file
                                    .read_to_string(&mut extension_results_yaml)
                                    .unwrap();

                                let results: Vec<SimpleKLResult> =
                                    serde_yaml::from_str(&extension_results_yaml).unwrap();

                                return results;
                            } else {
                                println!(
                                    "Error running extension: {}",
                                    String::from_utf8_lossy(&extension_run.stderr)
                                )
                            }
                        }

                        #[cfg(target_os = "windows")]
                        if cfg!(target_os = "windows") {
                            let extension_run = Command::new("cmd")
                                .arg("/C")
                                .arg("start extension.exe")
                                .current_dir(&folder_path)
                                .creation_flags(FLAG_NO_WINDOW)
                                .output()
                                .expect("Error running extension");

                            if extension_run.status.success() {
                                let mut extension_results_yaml = String::from("");
                                let mut extensions_results_file =
                                    File::open(&get_extension_results_path().unwrap()).unwrap();

                                extensions_results_file
                                    .read_to_string(&mut extension_results_yaml)
                                    .unwrap();

                                let results: Vec<SimpleKLResult> =
                                    serde_yaml::from_str(&extension_results_yaml).unwrap();

                                return results;
                            } else {
                                println!(
                                    "Error running extension: {}",
                                    String::from_utf8_lossy(&extension_run.stderr)
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    return Vec::new();
}

#[tauri::command(rename_all = "snake_case")]
pub async fn run_action(
    action_type: String,
    action_json: String,
    window: Window,
    app_handle: AppHandle,
) {
    match action_type.as_str() {
        "OpenApp" => {
            window.hide().unwrap();

            let action: OpenApp =
                serde_json::from_str(&action_json).expect("Error getting action from JSON");

            #[cfg(target_os = "linux")]
            if cfg!(target_os = "linux") {
                let desktop_file_dir = Path::new(&action.desktop_path)
                    .parent()
                    .expect("Error reading parent directory")
                    .to_owned();

                let desktop_file_name = Path::new(&action.desktop_path)
                    .file_name()
                    .expect("")
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
                let path = Path::new(&action.desktop_path).to_owned();

                std::thread::spawn(move || {
                    let script = format!(
                        "invoke-item '{}'",
                        path.into_os_string().into_string().unwrap()
                    );

                    powershell_script::run(&script).expect("Error opening app");
                });
            }
        }
        "OpenInBrowser" => {
            let action: OpenInBrowser = serde_json::from_str(&action_json).unwrap();
            open::that(action.url).expect("Error opening url");
        }
        "CopyToClipboard" => {}
        "ExtensionAction" => {
            let action: ExtensionAction = serde_json::from_str(&action_json).unwrap();

            let parameters =
                Parameters::new_action(action.action, action.args.unwrap_or(Vec::new()));

            let mut parameters_file = File::create(&get_extension_parameters_path().unwrap())
                .expect("Error opening parameters file");

            let parameters_yaml =
                serde_yaml::to_string(&parameters).expect("Error converting parameters yaml");

            parameters_file
                .write_all(&parameters_yaml.as_bytes())
                .unwrap();

            parameters_file.flush().unwrap();

            if cfg!(target_os = "linux") {
                Command::new("sh")
                    .arg("-c")
                    .arg("./extension")
                    .current_dir(get_extension_path(&action.extension_id).unwrap())
                    .output()
                    .expect("Error running extension action");
            }

            #[cfg(target_os = "windows")]
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg("start extension.exe")
                    .current_dir(get_extension_path(&action.extension_id).unwrap())
                    .creation_flags(FLAG_NO_WINDOW)
                    .output()
                    .expect("Error running extension action");
            }
        }
        "DialogAction" => {
            //Writes a file with all the fields for the dialog
            let action: DialogAction =
                serde_json::from_str(&action_json).expect("Error getting action from JSON");

            let action_yaml =
                serde_yaml::to_string(&action).expect("Error creating dialog action yaml");

            let mut dialog_action_file = File::create(get_dialog_action_path().unwrap())
                .expect("Error creating dialog action file");

            dialog_action_file
                .write_all(action_yaml.as_bytes())
                .expect("Error writing dialog action file");

            dialog_action_file
                .flush()
                .expect("Error closing dialog action file");

            //Opens the dialog window
            WindowBuilder::new(
                &app_handle,
                "extension_dialog",
                WindowUrl::App("extension-dialog".parse().unwrap()),
            )
                .max_inner_size(300.0, 800.0)
                .resizable(false)
                .title(action.title)
                .build()
                .expect("Error spawning extension dialog");
        }
        _ => {}
    };

    window.close().unwrap();
}