// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::{models, schema, tables};
use std::sync::{Arc, RwLock};
use time;
use uuid;

// POST (/api/subject) route handler
pub async fn create_subject_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<models::Subject>,
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

    let results: tables::Subject = diesel::insert_into(schema::subjects::dsl::subjects)
        .values(tables::Subject {
            id: uuid::Uuid::new_v4().to_string(),
            name: json.name,
            class_id: json.class_id,
            created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
            updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
        })
        .get_result(&mut conn)
        .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(models::Subject {
            id: results.id,
            name: results.name,
            display_name: json.display_name,
            class_id: results.class_id,
            class_name: json.class_name,
            created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
            updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
        }),
    )
        .into_response());
}

// POST (/api/subject/json) route handler
pub async fn json_to_subject_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<Vec<models::Subject>>,
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

    for element in input_data.iter_mut() {
        let curr_class: tables::Class = match schema::classes::dsl::classes
            .filter(schema::classes::name.eq(element.class_name.clone()))
            .select(tables::Class::as_select())
            .first(&mut conn)
        {
            Ok(safe_results) => safe_results,
            Err(e) => {
                println!("Failed to validate records from JSON data, Error {:#?}", e);
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    String::from("Failed to validate records from JSON data"),
                ));
            }
        };
        element.id = uuid::Uuid::new_v4().to_string();
        element.class_id = curr_class.id;
        element.created_at = time::OffsetDateTime::now_utc().unix_timestamp();
        element.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    }
    let mut new_records: Vec<tables::Subject> = Vec::new();
    input_data.iter().for_each(|element| {
        new_records.push(
            diesel::insert_into(schema::subjects::dsl::subjects)
                .values(tables::Subject {
                    id: element.id.clone(),
                    name: element.name.clone(),
                    class_id: element.class_id.clone(),
                    created_at: element.created_at,
                    updated_at: element.updated_at,
                })
                .get_result(&mut conn)
                .unwrap(),
        );
    });
    // UNWRAP: just sent back `input_data` as result
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(input_data),
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
    diesel::delete(
        schema::subjects::dsl::subjects.filter(schema::subjects::dsl::id.eq(subject_id)),
    )
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

// PATCH (/api/subject) route handler
pub async fn update_subject_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<models::Subject>,
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

    let mut input_data = json.clone();
    input_data.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    let _result: tables::Subject = tables::Subject {
        id: input_data.id.clone(),
        name: input_data.name.clone(),
        class_id: input_data.class_id.clone(),
        created_at: input_data.created_at,
        updated_at: input_data.updated_at,
    }
    .save_changes(&mut conn)
    .unwrap();

    // UNWRAP: just sent back `input_data` as result
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(input_data),
    )
        .into_response());
}
