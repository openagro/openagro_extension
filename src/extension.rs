use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub extension_id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub author: String,
    pub version: String,
    pub data: Vec<String>,
    pub depends_on: Vec<String>,
    pub application: bool,
}

pub trait Extension {
    fn migrate(&self);
}

impl Extension for ExtensionMetadata {
    fn migrate(&self) {
        // do migration to database
    }   
}