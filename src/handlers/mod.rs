// models
pub mod api;
pub mod create_test;
pub mod templates;

// imports
use crate::utils;
use askama::Template;
use axum::{self, response::IntoResponse};
use mime_guess;

// (/static) route handler
pub async fn static_route(
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // get static file content
    let file_contents = match utils::get_embedded_file(filepath.to_string()) {
        Some(some_file_contents) => match some_file_contents {
            Ok(safe_file_contents) => safe_file_contents,
            Err(_) => {
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Internal Server Error"),
                ))
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

// (/) home page route handler
pub async fn home_page() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct
    let html = match (templates::HomePage {}.render()) {
        Ok(safe_html) => safe_html,
        Err(_) => {
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

// (/test) route handler
pub async fn test_page() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct (PS: this whole thing upto the return, is a single let statement)
    let html = match (templates::TestPage {
        latex_content: match katex::render_with_opts(
            "\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
            match katex::Opts::builder()
                .output_type(katex::OutputType::Mathml)
                .build()
            {
                Ok(safe_builder_opts) => safe_builder_opts,
                Err(_) => {
                    return Err((
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        String::from("Failed to build latex builder options"),
                    ))
                }
            },
        ) {
            Ok(safe_latex) => safe_latex,
            Err(_) => {
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to render latex content"),
                ))
            }
        },
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(_) => {
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
