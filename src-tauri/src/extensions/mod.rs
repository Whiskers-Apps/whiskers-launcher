#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommunityExtension {
    pub id: String,
    #[serde(default = "get_empty_string")]
    pub name: String,
    #[serde(default = "get_empty_string")]
    pub description: String,
    #[serde(default = "get_empty_string")]
    pub repo: String,
    #[serde(default = "get_empty_string")]
    pub preview: String,
    #[serde(default = "get_empty_string_vec")]
    pub platforms: Vec<String>,
}

fn get_empty_string() -> String { return String::from(""); }

fn get_empty_string_vec() -> Vec<String> { return vec![] }