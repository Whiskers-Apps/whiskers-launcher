use whiskers_launcher_core::{
    features::{
        core::extensions::{get_form_request, write_form_response},
        extensions::FormResponse,
    },
    results::OpenFormAction,
};

#[tauri::command()]
pub fn run_get_form_request() -> OpenFormAction {
    return get_form_request();
}

#[tauri::command()]
pub fn run_write_form_response(response: FormResponse) {
    write_form_response(response);
}