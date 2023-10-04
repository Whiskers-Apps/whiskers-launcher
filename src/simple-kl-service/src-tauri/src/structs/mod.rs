
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub icon_path: String,
    pub exec_path: String,
    pub name: String,
}
