// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::schema;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

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

    let results: Vec<core::models::Chapter> = schema::chapters::dsl::chapters
        .limit(core::PAGE_SIZE.into())
        .offset(offset as i64)
        .load(&mut conn)
        .unwrap();

    return Ok(axum::Json(results).into_response());
}

// POST (/api/chapter) route handler
pub async fn create_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<core::models::Chapter>,
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
    let results: core::models::Chapter = diesel::insert_into(schema::chapters::dsl::chapters)
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

// POST (/api/chapter/json) route handler
pub async fn json_to_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<Vec<core::models::Chapter>>,
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
    let mut new_records: Vec<core::models::Chapter> = Vec::new();
    input_data.iter().for_each(|element| {
        new_records.push(
            diesel::insert_into(schema::chapters::dsl::chapters)
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

// DELETE (/api/chapter/:chapter_id) route handler
pub async fn delete_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Path(chapter_id): axum::extract::Path<String>,
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

    // delete the chapter
    diesel::delete(
        schema::chapters::dsl::chapters.filter(schema::chapters::dsl::id.eq(chapter_id)),
    )
    .execute(&mut conn)
    .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("DELETE CHAPTER"),
    )
        .into_response());
}

// PATCH (/api/chapter) route handler
pub async fn update_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<core::models::Chapter>,
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

    let subject: core::models::Subject = schema::subjects::dsl::subjects
        .filter(schema::subjects::name.eq(json.clone().subject_name))
        .filter(schema::subjects::class_name.eq(json.clone().class_name))
        .select(core::models::Subject::as_select())
        .first(&mut conn)
        .unwrap();
    let mut input_data = json.clone();
    input_data.subject_id = subject.id;
    let result: core::models::Chapter = input_data.save_changes(&mut conn).unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(result),
    )
        .into_response());
}
