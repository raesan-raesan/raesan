//imports
use crate::core::models;
use serde_json;

// ----- `Dataset` struct
#[derive(Debug, Clone)]
pub struct Dataset {
    pub classes: models::Classes,
    pub subjects: models::Subjects,
    pub chapters: models::Chapters,
}
impl Dataset {
    // create a new `Dataset` struct
    pub fn new() -> Result<Dataset, String> {
        let classes_contents = match std::fs::read_to_string("./dataset/_classes.json") {
            Ok(safe_contents) => safe_contents,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let classes: models::Classes = match serde_json::from_str(&classes_contents) {
            Ok(safe_classes) => safe_classes,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let subjects_contents = match std::fs::read_to_string("./dataset/_subjects.json") {
            Ok(safe_contents) => safe_contents,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let subjects: models::Subjects = match serde_json::from_str(&subjects_contents) {
            Ok(safe_subjects) => safe_subjects,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let chapters_contents = match std::fs::read_to_string("./dataset/_chapters.json") {
            Ok(safe_contents) => safe_contents,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let chapters: models::Chapters = match serde_json::from_str(&chapters_contents) {
            Ok(safe_chapters) => safe_chapters,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        return Ok(Dataset {
            classes,
            subjects,
            chapters,
        });
    }
}
