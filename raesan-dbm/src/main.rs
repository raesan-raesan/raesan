// modules
mod core;
mod handlers;
mod templates;
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
            println!("CLI Args: {:#?}", data);
            let app_router: axum::Router = axum::Router::new()
                .route(
                    // static files route
                    "/static/:filepath",
                    axum::routing::get(handlers::static_route),
                )
                .route("/", axum::routing::get(handlers::home_page))
                .route("/class", axum::routing::get(handlers::class_page))
                .route("/subject", axum::routing::get(handlers::subject_page))
                .route("/chapter", axum::routing::get(handlers::chapter_page))
                .route("/question", axum::routing::get(handlers::question_page))
                .route(
                    "/api/class",
                    axum::routing::post(handlers::api::class::create_class_route),
                )
                .route(
                    "/api/class/json",
                    axum::routing::post(handlers::api::class::json_to_class_route),
                )
                .route(
                    "/api/class/:class_id",
                    axum::routing::delete(handlers::api::class::delete_class_route),
                )
                .route(
                    "/api/class",
                    axum::routing::patch(handlers::api::class::update_class_route),
                )
                .route(
                    "/api/subject",
                    axum::routing::post(handlers::api::subject::create_subject_route),
                )
                .route(
                    "/api/subject/json",
                    axum::routing::post(handlers::api::subject::json_to_subject_route),
                )
                .route(
                    "/api/subject/:subject_id",
                    axum::routing::delete(handlers::api::subject::delete_subject_route),
                )
                .route(
                    "/api/subject",
                    axum::routing::patch(handlers::api::subject::update_subject_route),
                )
                .route(
                    "/api/chapter",
                    axum::routing::get(handlers::api::chapter::get_chapter_route),
                )
                .route(
                    "/api/chapter",
                    axum::routing::post(handlers::api::chapter::create_chapter_route),
                )
                .route(
                    "/api/chapter/json",
                    axum::routing::post(handlers::api::chapter::json_to_chapter_route),
                )
                .route(
                    "/api/chapter/:chapter_id",
                    axum::routing::delete(handlers::api::chapter::delete_chapter_route),
                )
                .route(
                    "/api/chapter",
                    axum::routing::patch(handlers::api::chapter::update_chapter_route),
                )
                .route(
                    "/api/question",
                    axum::routing::get(handlers::api::question::get_question_handler),
                )
                .route(
                    "/api/question",
                    axum::routing::post(handlers::api::question::create_question_route),
                )
                .route(
                    "/api/question/json",
                    axum::routing::post(handlers::api::question::json_to_question_route),
                )
                .route(
                    "/api/question/:question_id",
                    axum::routing::delete(handlers::api::question::delete_question_route),
                )
                .route(
                    "/api/question",
                    axum::routing::patch(handlers::api::question::update_question_route),
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
        core::app::SubCommands::GenerateDatabaseRecords(data) => {
            match utils::generate_database_records_for_testing(data.clone()) {
                Ok(_) => {}
                Err(e) => {
                    eprint!(
                        "Failed to generate temporary database records for testing, Error: {:#?}",
                        e.to_string()
                    );
                    std::process::exit(1);
                }
            };
        }
        core::app::SubCommands::ExportDataset(data) => {
            println!(
                "Exporting JSON dataset from database:{:#?}, to:{:#?}",
                data.database, data.dataset
            );
        }
    }
}
