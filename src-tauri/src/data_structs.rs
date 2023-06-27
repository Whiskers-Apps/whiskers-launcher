use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SimpleKLResult {
    pub icon: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub text: Option<String>,
    pub action: Option<Action>,
}

#[derive(Serialize, Debug)]
pub enum Action {
    OpenApp(OpenAppAction),
    OpenInBrowser(OpenInBrowserAction),
}

#[derive(Serialize, Debug)]
pub struct OpenAppAction {
    pub exec_path: String,
}

#[derive(Serialize, Debug)]
pub struct OpenInBrowserAction {
    pub url: String,
}

impl SimpleKLResult {
    pub fn new() -> Self {
        return SimpleKLResult {
            icon: Some("".to_string()),
            title: None,
            description: None,
            text: None,
            action: None,
        };
    }
}
