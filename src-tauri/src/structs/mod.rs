pub mod structs{
    use serde::{Serialize, Deserialize};


    #[derive(Debug, Serialize, Deserialize)]
    pub struct AppIndex{
        pub icon_path: String,
        pub desktop_path: String,
        pub name: String
    }
}