// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::schema;
use std::sync::{Arc, RwLock};
use uuid;

// POST (/api/subject) route handler
pub async fn create_subject_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<core::models::Subject>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let mut input_data = json.clone();
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

    input_data.id = uuid::Uuid::new_v4().to_string();
    let results: core::models::Subject = diesel::insert_into(schema::subject::dsl::subject)
        .values(input_data)
        .get_result(&mut conn)
        .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(results),
    )
        .into_response());
}

// DELETE (/api/subject/:subject_id) route handler
pub async fn delete_subject_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Path(subject_id): axum::extract::Path<String>,
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

    // delete the subject
    diesel::delete(schema::subject::dsl::subject.filter(schema::subject::dsl::id.eq(subject_id)))
        .execute(&mut conn)
        .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("DELETE SUBJECT"),
    )
        .into_response());
}

// POST (/api/subject/json) route handler
pub async fn json_to_subject_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE SUBJECT FROM JSON");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE SUBJECT FROM JSON"),
    )
        .into_response());
}
