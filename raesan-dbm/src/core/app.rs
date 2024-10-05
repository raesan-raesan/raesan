// imports
use crate::core;
use clap::Parser;

// ----- `CLIArgs` object
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
pub struct CLIArgs {
    pub database_url: Option<String>,
}

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub database: core::database::Database,
}
impl Application {
    pub fn new() -> Result<Application, String> {
        // get CLI arguments
        let args = CLIArgs::parse();

        // database
        let database = match core::database::Database::new(args) {
            Ok(safe_db) => safe_db,
            Err(e) => return Err(e.to_string()),
        };

        return Ok(Application { database });
    }
}
