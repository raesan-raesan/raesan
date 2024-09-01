//imports
use crate::core::models;
use serde_json;

// ----- `Database` struct
#[derive(Debug, Clone)]
pub struct Database {
    pub classes: models::Classes,
}
impl Database {
    pub fn new() -> Database {
        let contents = std::fs::read_to_string("./dataset/_base.json").unwrap();
        let classes: models::Classes = serde_json::from_str(&contents).unwrap();
        return Database { classes };
    }
}
