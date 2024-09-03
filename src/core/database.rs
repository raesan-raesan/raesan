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
    // ----- problem 1: you have to make sure that all the subjects of the selected 2 classes are displayed -----
    pub fn get_class_list(&self) -> Vec<String> {
        return self
            .classes
            .classes
            .iter()
            .map(|class| class.id.to_string())
            .collect();
    }
    // ----- problem: you have to make sure subjects that are in any two(or more) classes don't repeat -----
    pub fn get_subject_list(&self) -> Vec<String> {
        let subject_list: Vec<String> = Vec::new();
        return subject_list;
    }
}
