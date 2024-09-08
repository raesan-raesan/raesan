//imports
use crate::core::models;
use serde_json;

// ----- `Database` struct
#[derive(Debug, Clone)]
pub struct Database {
    pub classes: models::Classes,
}
impl Database {
    pub fn new() -> Result<Database, String> {
        let contents = match std::fs::read_to_string("./dataset/_base.json") {
            Ok(safe_contents) => safe_contents,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let classes: models::Classes = match serde_json::from_str(&contents) {
            Ok(safe_classes) => safe_classes,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        return Ok(Database { classes });
    }
    pub fn get_class_list(&self) -> Vec<String> {
        return self
            .classes
            .classes
            .iter()
            .map(|class| class.id.to_string())
            .collect();
    }
}
