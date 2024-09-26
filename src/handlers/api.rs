use crate::core::models;
use axum::{self, response::IntoResponse};

// POST (/api/create-test) route handler
pub async fn create_test_route(
    axum::extract::Json(create_test_input): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("{:#?}", create_test_input);
    return Ok(axum::response::Redirect::to("/").into_response());
}
