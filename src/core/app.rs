//imports
use crate::core::database;

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub database: database::Database,
}
impl Application {
    pub fn new() -> Result<Application, String> {
        let database = match database::Database::new() {
            Ok(safe_db) => safe_db,
            Err(e) => return Err(e.to_string()),
        };
        return Ok(Application { database });
    }
}
