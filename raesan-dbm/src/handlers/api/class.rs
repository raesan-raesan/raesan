// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::schema;
use std::sync::{Arc, RwLock};
use uuid;

// POST (/api/class) route handler
pub async fn create_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<core::models::Class>,
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
    let results: core::models::Class = diesel::insert_into(schema::class::dsl::class)
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

// POST (/api/class/json) route handler
pub async fn json_to_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<Vec<core::models::Class>>,
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

    input_data
        .iter_mut()
        .for_each(|element| element.id = uuid::Uuid::new_v4().to_string());
    let mut new_records: Vec<core::models::Class> = Vec::new();
    input_data.iter().for_each(|element| {
        new_records.push(
            diesel::insert_into(schema::class::dsl::class)
                .values(element)
                .get_result(&mut conn)
                .unwrap(),
        );
    });
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(new_records),
    )
        .into_response());
}

// DELETE (/api/class/:class_id) route handler
pub async fn delete_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Path(class_id): axum::extract::Path<String>,
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

    // delete the class
    diesel::delete(schema::class::dsl::class.filter(schema::class::dsl::id.eq(class_id)))
        .execute(&mut conn)
        .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("DELETE CLASS"),
    )
        .into_response());
}
