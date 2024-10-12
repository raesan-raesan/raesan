// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// POST (/api/class) route handler
pub async fn create_class_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE CLASS");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE CLASS"),
    )
        .into_response());
}

// POST (/api/subject) route handler
pub async fn create_subject_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE SUBJECT");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE SUBJECT"),
    )
        .into_response());
}

// GET (/api/chapter) route handler
pub async fn get_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Query(query): axum::extract::Query<HashMap<String, u64>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // offset
    let offset = (match query.get("page") {
        Some(page_value) => {
            if *page_value == 0 {
                println!("Invalid query parameters");
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    String::from("Invalid query parameters"),
                ));
            } else {
                page_value
            }
        }
        None => {
            println!("Invalid query parameters");
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                String::from("Invalid query parameters"),
            ));
        }
    } - 1)
        * core::PAGE_SIZE as u64;

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

    let results: Vec<core::models::Chapter> = raesan_common::schema::chapter::dsl::chapter
        .limit(core::PAGE_SIZE.into())
        .offset(offset as i64)
        .load(&mut conn)
        .unwrap();

    return Ok(axum::Json(results).into_response());
}
// POST (/api/chapter) route handler
pub async fn create_chapter_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE CHAPTER");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE CHAPTER"),
    )
        .into_response());
}

// GET (/api/question) route handler
pub async fn get_question_handler(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Query(query): axum::extract::Query<HashMap<String, u64>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // offset
    let offset = (match query.get("page") {
        Some(page_value) => {
            if *page_value == 0 {
                println!("Invalid query parameters");
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    String::from("Invalid query parameters"),
                ));
            } else {
                page_value
            }
        }
        None => {
            println!("Invalid query parameters");
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                String::from("Invalid query parameters"),
            ));
        }
    } - 1)
        * core::PAGE_SIZE as u64;

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

    let results: Vec<core::models::Question> = raesan_common::schema::question::dsl::question
        .limit(core::PAGE_SIZE.into())
        .offset(offset as i64)
        .load(&mut conn)
        .unwrap();

    return Ok(axum::Json(results).into_response());
}
// POST (/api/question) route handler
pub async fn create_question_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE QUESTION");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE QUESTION"),
    )
        .into_response());
}
