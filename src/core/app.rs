//imports
use crate::core::dataset;

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub dataset: dataset::Dataset,
}
impl Application {
    pub fn new() -> Result<Application, String> {
        let dataset = match dataset::Dataset::new() {
            Ok(safe_db) => safe_db,
            Err(e) => return Err(e.to_string()),
        };
        return Ok(Application { dataset });
    }
}
