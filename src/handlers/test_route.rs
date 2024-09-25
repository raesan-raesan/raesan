use crate::{core::models, handlers::templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use std::collections::HashMap;

// GET (/test) route handler
pub async fn route(
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
