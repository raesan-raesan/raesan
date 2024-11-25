use crate::templates;
use askama::Template;
use axum::{self, response::IntoResponse};

// GET (/test/:test_id) route handler
pub async fn route(
    axum::extract::Path(test_id): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct (PS: this whole thing upto the return, is a single let statement)
    let html = match (templates::routes::TestPage { test_id }.render()) {
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
