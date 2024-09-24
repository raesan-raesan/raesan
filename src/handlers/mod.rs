// models
pub mod templates;

// imports
use crate::{
    core::{app, models},
    utils,
};
use askama::Template;
use axum::{self, response::IntoResponse};
use mime_guess;
use std::{collections::HashMap, sync::Arc};

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
pub async fn home_page() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
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

// GET (/create-test) route handlers
pub async fn create_test_route(
    axum::extract::State(app): axum::extract::State<Arc<app::Application>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let html = match (templates::CreateTestPage {
        dataset_classes: app.dataset.classes.clone().classes,
        dataset_subjects: app.dataset.subjects.clone().subjects,
        dataset_chapters: app.dataset.chapters.clone().chapters,
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error: {:#?}", e);
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

// GET (/test) route handler
pub async fn test_page(
    axum::extract::Query(query_params): axum::extract::Query<HashMap<String, String>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let create_test_input: models::CreateTestInput =
        match serde_json::from_str(match query_params.get("create_test_input") {
            Some(some_query_param) => some_query_param,
            None => {
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("No Query Params Provided"),
                ));
            }
        }) {
            Ok(safe_create_test_input) => safe_create_test_input,
            Err(e) => {
                println!("Failed to PARSE query parameters, Error: {:#?}", e);
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to PARSE query parameters"),
                ));
            }
        };

    println!("{:#?}", create_test_input);
    // render HTML struct (PS: this whole thing upto the return, is a single let statement)
    let html = match (templates::TestPage {
        latex_content: match katex::render_with_opts(
            "\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
            match katex::Opts::builder()
                .output_type(katex::OutputType::Mathml)
                .build()
            {
                Ok(safe_builder_opts) => safe_builder_opts,
                Err(e) => {
                    println!("Failed to build latex builder options, Error: {:#?}", e);
                    return Err((
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        String::from("Failed to build latex builder options"),
                    ));
                }
            },
        ) {
            Ok(safe_latex) => safe_latex,
            Err(e) => {
                println!("Failed to render latex content, Error: {:#?}", e);
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to render latex content"),
                ));
            }
        },
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error: {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
        .into_response());
}
