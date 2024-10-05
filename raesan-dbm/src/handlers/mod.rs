// models
pub mod templates;

// imports
use crate::{core, utils};
use askama::Template;
use axum::{self, response::IntoResponse};
use diesel::{self, prelude::*};
use mime_guess;
use raesan_common;
use std::sync::{Arc, RwLock};

// GET (/static) route handler
pub async fn static_route(
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // get static file content
    let file_contents = match utils::get_embedded_file(filepath.to_string()) {
        Some(some_file_contents) => match some_file_contents {
            Ok(safe_file_contents) => safe_file_contents,
            Err(e) => {
                println!(
                    "Failed to convert file contents into readable string format, Error: {:#?}",
                    e
                );
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to convert file contents into readable string format"),
                ));
            }
        },
        None => {
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                String::from("404 Not Found!"),
            ))
        }
    };

    // get the file type
    let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

    return Ok((
        [(axum::http::header::CONTENT_TYPE, file_type.to_string())],
        file_contents,
    )
        .into_response());
}

// GET (/) home page route handler
pub async fn home_page(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // database connection
    let mut conn = match match app_state.write() {
        Ok(safe_app_state) => safe_app_state,
        Err(e) => {
            println!("Failed to get application state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get application state"),
            ));
        }
    }
    .database
    .pool
    .get()
    {
        Ok(safe_conn) => safe_conn,
        Err(e) => {
            println!("Failed to get database connection, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get database connection"),
            ));
        }
    };

    let results = raesan_common::schema::class::dsl::class
        .select(core::models::Class::as_select())
        .load(&mut conn)
        .expect("Error loading classes");
    println!("Classes: {:#?}", results);

    // render HTML struct
    let html = match (templates::HomePage {}.render()) {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
