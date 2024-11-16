// imports
use crate::core;

// Sync JSON dataset with SQLite database
pub fn sync_dataset(data: core::app::SyncDataset) -> Result<(), String> {
    println!("{:#?}", data);
    return Ok(());
}
