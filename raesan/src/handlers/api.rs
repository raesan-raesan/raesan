use crate::core::{self, models};
use axum::{self, response::IntoResponse};
use axum_macros;
use diesel::{self, prelude::*};
use raesan_common;
use rand::{self, prelude::*};
use std::sync::{Arc, RwLock};
// use axum_extra;
// use serde_json;
// use time;

// POST (/api/create-test) route handler
#[axum_macros::debug_handler]
pub async fn create_test_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(create_test_input): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // database connection
    let mut conn = match match app_state.write() {
        Ok(safe_app_state) => safe_app_state,
        Err(e) => {
            println!("Failed to get application state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get application state"),
            ));
        }
    }
    .database
    .pool
    .get()
    {
        Ok(safe_conn) => safe_conn,
        Err(e) => {
            println!("Failed to get database connection, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get database connection"),
            ));
        }
    };

    // get the questions
    let mut questions = raesan_common::schema::questions::dsl::questions
        .filter(raesan_common::schema::questions::chapter_id.eq_any(create_test_input.chapters))
        .select(models::Question::as_select())
        .load(&mut conn)
        .expect("Failed to fetch questions");

    // make the test
    let mut rng = rand::thread_rng();
    for i in 0..create_test_input.format.total_questions {
        let random_index = rng.gen_range(i..questions.len().try_into().unwrap());
        questions.swap(i.try_into().unwrap(), random_index.try_into().unwrap());
    }
    let questions = questions
        .into_iter()
        .take(create_test_input.format.total_questions.try_into().unwrap())
        .map(|question| models::TestQuestion {
            id: question.id,
            body: question.body,
        })
        .collect::<Vec<models::TestQuestion>>();

    return Ok((axum::http::StatusCode::OK, axum::response::Json(questions)).into_response());
}
