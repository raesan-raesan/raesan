//imports
use crate::core::database;

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub database: database::Database,
}
impl Application {
    pub fn new() -> Application {
        let database = database::Database::new();
        return Application { database };
    }
}
