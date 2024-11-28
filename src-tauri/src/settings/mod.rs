mod manager;
pub use manager::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub openai_api_key: String,
    pub pinecone_api_key: String,
    pub pinecone_index_host: String,
    pub pinecone_namespace: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            openai_api_key: String::new(),
            pinecone_api_key: String::new(),
            pinecone_index_host: String::new(),
            pinecone_namespace: String::from("book"),
        }
    }
}
