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
}
