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
}

// ----- `Serve` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Serve the application in various modes")]
pub struct Serve {
    #[arg(long, help = "path location of database")]
    pub database: String,
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
