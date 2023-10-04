#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommunityTheme{
    repo: String,
    file: String,
    preview: String,
    name: String
}