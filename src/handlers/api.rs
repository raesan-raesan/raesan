// imports
use crate::core::models;
use axum::{self, response::IntoResponse};
use serde_json;

// POST (/api/create-test) route
pub async fn create_test(
    axum::extract::Json(json): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("{:#?}", json);
    if json.curr_step == 2 {
        println!("we got class inputs, we need to send subject inputs");
    }
    if json.curr_step == 3 {
        println!("we got subject inputs, we need to send chapter inputs");
    }
    return Ok((serde_json::to_string(&json).unwrap()).into_response());
}
