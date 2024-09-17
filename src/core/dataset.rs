//imports
use crate::core::models;
use serde_json;

// ----- `Dataset` struct
#[derive(Debug, Clone)]
pub struct Dataset {
    pub classes: models::Classes,
}
impl Dataset {
    // create a new `Dataset` struct
    pub fn new() -> Result<Dataset, String> {
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
        return Ok(Dataset { classes });
    }
    // get the list of available classes in the dataset
    pub fn get_class_list(&self) -> Vec<String> {
        return self
            .classes
            .classes
            .iter()
            .map(|class| class.id.to_string())
            .collect();
    }
    // get the list of available subjects in the dataset according to selected classes
    pub fn get_subject_list(&self, _class_list: Vec<u32>) -> Vec<String> {
        let subject_list: Vec<String> = Vec::new();
        return subject_list;
    }
    // get the list of available chapters in the dataset according to selected subjects
    pub fn get_chapter_list(&self, _subject_list: Vec<String>) -> Vec<String> {
        let chapter_list: Vec<String> = Vec::new();
        return chapter_list;
    }
}
