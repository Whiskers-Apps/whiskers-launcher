#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommunityExtension{
    pub id: String,
    pub name: String,
    pub description: String,
    pub repo: String,
    pub preview: String
}