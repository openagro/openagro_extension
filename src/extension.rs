use serde::{Deserialize, Serialize};

use crate::assets::Assets;

/// # Extension Metadata
/// 
/// Struct type to define extension as a openagro extension
/// and specify extension metadata.
/// 
/// **Example :** 
/// 
/// Put this pice of code on your `src/lib.rs`
/// 
/// ```
/// pub func metadata() -> ExtensionMetadata {
///     ExtensionMetadata {
///         extension_id: "openagro.your.extension.name".to_string(),
///         name: "Your Extension Name".to_string(),
///         description: Some("Your Extension Description".to_string()),
///         category: Some("Your Extension Category"),
///         author: "your name".to_string(),
///         version: "your.app.version".to_string(),
///         depends_on: vec![
///             "other.extension.is.needed.to.run.this.extension".to_string(),    
///         ],
///         data: vec![
///             include_str!("../path/to/yourfile.js").to_string(),
///             include_str!("path/to/yourfile").to_string(),
///         ],
///         application: true,
///     }
/// }
/// ```
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub extension_id: String, // extension id
    pub name: String, // extension name
    // extension description
    pub description: Option<String>,
    pub category: Option<String>,
    pub author: String,
    pub version: String,
    pub data: Vec<Assets>,
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