// imports
use crate::core;
// use diesel::{self, prelude::*};
// use r2d2;
// use raesan_common::schema;
// use serde_json;
// use std::{fs, io, path::Path};

// generate database records for testing
pub fn export_dataset(data: core::app::ExportDataset) -> Result<(), String> {
    // let database = match core::database::Database::new(data.database) {
    //     Ok(safe_db) => safe_db,
    //     Err(e) => return Err(e.to_string()),
    // };
    // let mut conn = match database.pool.get() {
    //     Ok(safe_conn) => safe_conn,
    //     Err(e) => {
    //         return Err(e.to_string());
    //     }
    // };
    println!("{:#?}", data);
    return Ok(());
}
