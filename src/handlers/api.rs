// imports
use crate::core::{app, models};
use axum::{self, response::IntoResponse};
use serde_json;
use std::sync::Arc;

// POST (/api/create-test) route
pub async fn create_test(
    axum::extract::State(app): axum::extract::State<Arc<app::Application>>,
    axum::extract::Json(mut json): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("{:#?}", json);
    if json.curr_step == 2 {
        json.subjects = app.dataset.get_subject_list(json.classes.clone());
    }
    if json.curr_step == 3 {
        json.chapters = app.dataset.get_chapter_list(json.subjects.clone());
    }
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
