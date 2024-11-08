// imports
use diesel;
use r2d2;

// ----- `Database` struct
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>,
}
impl Database {
    // create a new `Database` struct
    pub fn new(database_url: String) -> Result<Database, String> {
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
