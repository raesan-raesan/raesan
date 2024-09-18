// imports
use crate::core::models;
use axum::{self, response::IntoResponse};
use serde_json;

// POST (/api/create-test) route
pub async fn create_test(
    // axum::extract::State(app): axum::extract::State<Arc<app::Application>>,
    axum::extract::Json(json): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // 💡 - we can use this API endpoint to actually create the tests and store them in the cookies
    // which are then fetched by (/test) route somehow with some mechanism to make sure some things
    // and parameters
    println!("{:#?}", json);
    match serde_json::to_string(&json) {
        Ok(safe_json) => {
            return Ok(safe_json.into_response());
        }
        Err(e) => {
            println!("Failed to seriaslize a struct into JSON, Error: {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to seriaslize a struct into JSON"),
            ));
        }
    }
}
