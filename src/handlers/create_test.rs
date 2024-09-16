// imports
use crate::{core::app, handlers::templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use std::sync::Arc;

// (/create-test) route handler
pub async fn route(
    axum::extract::State(app): axum::extract::State<Arc<app::Application>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let html = match (templates::CreateTestPage {
        class_list: app.database.get_class_list(),
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(_) => {
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ))
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
