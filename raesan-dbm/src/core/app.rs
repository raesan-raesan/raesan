// imports
use crate::core;
use clap::{Args, Parser, Subcommand};

// ----- `CLIArgs` struct
#[derive(Parser, Debug, Clone)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub sub_commands: SubCommands,
}

// ----- `SubCommands` for the CLIArgs
#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    Serve(Serve),
    GenerateDatabaseRecords(GenerateDatabaseRecords),
    ExportDataset(ExportDataset),
    SyncDataset(SyncDataset),
}

// ----- `Serve` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Serve the application in various modes")]
pub struct Serve {
    #[arg(long, help = "path location of database")]
    pub database: String,
}

// ----- `GenerateDatabaseRecords` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Generate SQLite database records from JSON dataset")]
pub struct GenerateDatabaseRecords {
    #[arg(long, help = "path location of database")]
    pub database: String,
    #[arg(long, help = "path location of dataset")]
    pub dataset: String,
}
// ----- `ExportDatabase` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Export JSON dataset from SQLite database")]
pub struct ExportDataset {
    #[arg(long, help = "path location of database")]
    pub database: String,
    #[arg(long, help = "path location of dataset")]
    pub dataset: String,
}
// ----- `SyncDataset` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Sync JSON dataset with SQLite database")]
pub struct SyncDataset {
    #[arg(long, help = "path location of database")]
    pub database: String,
    #[arg(long, help = "path location of dataset")]
    pub dataset: String,
}

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub database: core::database::Database,
}
impl Application {
    pub fn new(data: Serve) -> Result<Application, String> {
        // database
        let database = match core::database::Database::new(data.database) {
            Ok(safe_db) => safe_db,
            Err(e) => return Err(e.to_string()),
        };

        return Ok(Application { database });
    }
}
