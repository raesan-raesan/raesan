// modules
mod core;
mod handlers;
mod utils;

// imports
use axum;
use clap::Parser;
use dotenvy::dotenv;
use std::{env, fs, sync::Arc};
use tokio;

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    database_url: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut database_url: Option<String> = None;
    let args = Args::parse();
    if let Some(url) = args.database_url.as_deref() {
        // command line argument input
        database_url = Some(url.to_string());
    } else if let Ok(url) = env::var(core::DATABASE_URL_ENV_VAR) {
        // environment variable input
        database_url = Some(url.to_string());
    } else if let Ok(_) = fs::metadata(core::DATABASE_URL) {
        // .db in current directory
        database_url = Some(core::DATABASE_URL.to_string())
    }

    if database_url == None {
        println!(
            "Error: {:#?}",
            "No input .db file provided in CLI Arguments, ENV variables or current directory!"
        );
    }

    let app_router: axum::Router = axum::Router::new()
        .route(
            // static files route
            "/static/:filepath",
            axum::routing::get(handlers::static_route),
        )
        .route("/", axum::routing::get(handlers::home_page))
        .with_state(Arc::new(match core::app::Application::new() {
            // supplying the main router with main application state
            Ok(safe_app) => safe_app,
            Err(e) => {
                eprintln!("Failed to create application state object, Error: {:#?}", e);
                std::process::exit(1);
            }
        }));

    // bind a `TcpListener` to an address and port
    let listener = match tokio::net::TcpListener::bind(
        core::ADDRESS.to_string() + ":" + core::PORT.to_string().as_str(),
    )
    .await
    {
        Ok(safe_listener) => safe_listener,
        Err(e) => {
            eprintln!("Failed to bind TcpListener to address, Error: {:#?}", e);
            std::process::exit(1);
        }
    };

    // ----- announce the application startup -----
    println!("running on {}:{}", core::ADDRESS, core::PORT);

    // actually start the server listener
    match axum::serve(listener, app_router).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Failed to start the server listener for the application, Error: {:#?}",
                e
            );
            std::process::exit(1);
        }
    };
}
