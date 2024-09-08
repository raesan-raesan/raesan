// imports
use crate::{core::app, handlers::templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use std::sync::Arc;

// (/create-test) route handler
pub async fn route() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // redirect users to first step when they try to access the /create-test page
    return Ok(axum::response::Redirect::to("/create-test/1").into_response());
}

// (/create-test/{step_number}) get request handler
pub async fn page(
    axum::extract::State(app): axum::extract::State<Arc<app::Application>>,
    axum::extract::Path(step_number): axum::extract::Path<u32>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct
    let html = match match step_number {
        1 => templates::CreateTestPageStep1 {
            class_list: app.database.get_class_list(),
        }
        .render(),
        2 => templates::CreateTestPageStep2 {}.render(),
        3 => templates::CreateTestPageStep3 {}.render(),
        4 => templates::CreateTestPageStep4 {}.render(),
        5 => templates::CreateTestPageStep5 {}.render(),
        _ => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                String::from("Bad Request!"),
            ))
        }
    } {
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
