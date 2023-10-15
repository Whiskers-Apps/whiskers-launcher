pub mod structs{
    use serde::{Serialize, Deserialize};


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AppIndex{
        pub icon_path: String,
        pub exec_path: String,
        pub name: String
    }
}