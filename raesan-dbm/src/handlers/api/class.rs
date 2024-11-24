// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::{models, schema, tables};
use std::sync::{Arc, RwLock};
use time;
use uuid;

// POST (/api/class) route handler
pub async fn create_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<models::Class>,
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

    let results: tables::Class = diesel::insert_into(schema::classes::dsl::classes)
        .values(tables::Class {
            id: uuid::Uuid::new_v4().to_string(),
            name: json.name,
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
        axum::Json(models::Class {
            id: results.id,
            name: results.name,
            created_at: results.created_at,
            updated_at: results.updated_at,
        }),
    )
        .into_response());
}

// POST (/api/class/json) route handler
pub async fn json_to_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<Vec<models::Class>>,
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

    let mut new_records: Vec<models::Class> = Vec::new();
    json.iter().for_each(|element| {
        let data: tables::Class = diesel::insert_into(schema::classes::dsl::classes)
            .values(tables::Class {
                id: uuid::Uuid::new_v4().to_string(),
                name: element.name,
                created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
                updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
            })
            .get_result(&mut conn)
            .unwrap();
        new_records.push(models::Class {
            id: data.id,
            name: data.name,
            created_at: data.created_at,
            updated_at: data.updated_at,
        });
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
    diesel::delete(schema::classes::dsl::classes.filter(schema::classes::dsl::id.eq(class_id)))
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

// PATCH (/api/class) route handler
pub async fn update_class_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<models::Class>,
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

    let result: tables::Class = tables::Class {
        id: json.id,
        name: json.name,
        created_at: json.created_at,
        updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
    }
    .save_changes(&mut conn)
    .unwrap();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(models::Class {
            id: result.id,
            name: result.name,
            created_at: result.created_at,
            updated_at: result.updated_at,
        }),
    )
        .into_response());
}
