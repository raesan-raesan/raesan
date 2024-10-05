// imports
use crate::core;
use diesel;
use r2d2;
use std::{env, fs};

// ----- `Database` struct
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>,
}
impl Database {
    // create a new `Database` struct
    pub fn new(args: core::app::CLIArgs) -> Result<Database, String> {
        let database_url = match {
            if let Some(url) = args.database_url.as_deref() {
                // command line argument input
                Some(url.to_string())
            } else if let Ok(url) = env::var(core::DATABASE_URL_ENV_VAR) {
                // environment variable input
                Some(url.to_string())
            } else if let Ok(_) = fs::metadata(core::DATABASE_URL) {
                // .db in current directory
                Some(core::DATABASE_URL.to_string())
            } else {
                None
            }
        } {
            Some(url) => url,
            None => {
                println!(
                "Error: {:#?}",
                "No input .db file provided in CLI Arguments, ENV variables or current directory!"
            );
                std::process::exit(1);
            }
        };
        let conn_manager =
            diesel::r2d2::ConnectionManager::<diesel::sqlite::SqliteConnection>::new(&database_url);
        let pool = match r2d2::Pool::builder().build(conn_manager) {
            Ok(safe_pool) => safe_pool,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        return Ok(Database { pool });
    }
}
