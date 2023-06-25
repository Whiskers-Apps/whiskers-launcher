use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SimpleKLResult {
    pub icon: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub text: Option<String>,
}
