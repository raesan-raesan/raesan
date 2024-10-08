// modules
mod core;
mod handlers;
mod utils;

// imports
use axum;
use dotenvy::dotenv;
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // application state for the main router
    let app_state = Arc::new(RwLock::new(match core::app::Application::new() {
        Ok(safe_app) => safe_app,
        Err(e) => {
            eprintln!("Failed to create application state object, Error: {:#?}", e);
            std::process::exit(1);
        }
    }));

    match utils::generate_database_records_for_testing(app_state.clone()) {
        Ok(_) => {}
        Err(e) => {
            eprint!(
                "Failed to generate temporary database records for testing, Error: {:#?}",
                e.to_string()
            );
            std::process::exit(1);
        }
    };

    // main application router
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
            axum::routing::post(handlers::api::create_class_route),
        )
        .route(
            "/api/subject",
            axum::routing::post(handlers::api::create_subject_route),
        )
        .route(
            "/api/chapter",
            axum::routing::post(handlers::api::create_chapter_route),
        )
        .route(
            "/api/question",
            axum::routing::post(handlers::api::create_question_route),
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
