// modules
mod core;
mod handlers;
pub mod templates;
mod utils;

// imports
use axum;
use clap::Parser;
use std::sync::{Arc, RwLock};
use tokio;

#[tokio::main]
async fn main() {
    // get CLI arguments
    let args = core::app::CLIArgs::parse();

    // match sub-commands
    match &args.sub_commands {
        core::app::SubCommands::Serve(data) => {
            // application state for the main router
            let app_state = Arc::new(RwLock::new(
                match core::app::Application::new(data.clone()) {
                    Ok(safe_app) => safe_app,
                    Err(e) => {
                        eprintln!("Failed to create application state object, Error: {:#?}", e);
                        std::process::exit(1);
                    }
                },
            ));

            // main application router
            let app_router: axum::Router = axum::Router::new()
                .route(
                    // static files route
                    "/static/:filepath",
                    axum::routing::get(handlers::static_route),
                )
                .route("/health", axum::routing::get(handlers::health))
                .route("/", axum::routing::get(handlers::home_page))
                .route(
                    "/create-test",
                    axum::routing::get(handlers::create_test_page),
                )
                .route(
                    "/api/create-test",
                    axum::routing::post(handlers::api::create_test_route),
                )
                .route(
                    "/test/:test_id",
                    axum::routing::get(handlers::test_route::route),
                )
                .route(
                    "/api/create-test-page-meta",
                    axum::routing::get(handlers::api::create_test_page_meta),
                )
                .with_state(app_state);

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
    }
}
