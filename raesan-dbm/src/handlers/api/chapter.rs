// imports
use crate::core;
use axum::{self, response::IntoResponse};
use diesel::prelude::*;
use raesan_common::{models, schema, tables};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use time;

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

    let results = schema::chapters::table
        .inner_join(
            schema::subjects::table.on(schema::chapters::subject_id.eq(schema::subjects::id)),
        )
        .inner_join(schema::classes::table.on(schema::subjects::class_id.eq(schema::classes::id)))
        .select((
            schema::chapters::all_columns,
            schema::subjects::all_columns,
            schema::classes::all_columns,
        ))
        .limit(core::PAGE_SIZE.into())
        .offset(offset as i64)
        .load::<(tables::Chapter, tables::Subject, tables::Class)>(&mut conn)
        .unwrap()
        .iter()
        .map(|element| models::Chapter {
            id: element.0.id.clone(),
            name: element.0.name.clone(),
            display_name: format!(
                "{} - {} - {}",
                element.2.name, element.1.name, element.0.name
            ),
            subject_id: element.0.subject_id.clone(),
            subject_name: element.1.name.clone(),
            class_name: element.2.name,
            created_at: element.0.created_at,
            updated_at: element.0.updated_at,
        })
        .collect::<Vec<models::Chapter>>();

    return Ok(axum::Json(results).into_response());
}

// POST (/api/chapter) route handler
pub async fn create_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<models::Chapter>,
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

    let results: tables::Chapter = diesel::insert_into(schema::chapters::dsl::chapters)
        .values(tables::Chapter {
            id: uuid::Uuid::new_v4().to_string(),
            name: json.name.clone(),
            subject_id: json.subject_id.clone(),
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
        axum::Json(models::Chapter {
            id: results.id,
            name: results.name,
            display_name: json.display_name,
            subject_id: results.subject_id,
            subject_name: json.subject_name,
            class_name: json.class_name,
            created_at: results.created_at,
            updated_at: results.updated_at,
        }),
    )
        .into_response());
}

// POST (/api/chapter/json) route handler
pub async fn json_to_chapter_route(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Json(json): axum::extract::Json<Vec<models::Chapter>>,
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
        let curr_class = raesan_common::schema::classes::dsl::classes
            .filter(schema::classes::name.eq(element.class_name))
            .select(tables::Class::as_select())
            .first(&mut conn)
            .unwrap();
        let curr_subject = raesan_common::schema::subjects::dsl::subjects
            .filter(schema::subjects::name.eq(element.subject_name.clone()))
            .filter(schema::subjects::class_id.eq(curr_class.id))
            .select(tables::Subject::as_select())
            .first(&mut conn)
            .unwrap();
        element.id = uuid::Uuid::new_v4().to_string();
        element.subject_id = curr_subject.id;
        element.created_at = time::OffsetDateTime::now_utc().unix_timestamp();
        element.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    }
    input_data.iter().for_each(|element| {
        diesel::insert_into(schema::chapters::dsl::chapters)
            .values(tables::Chapter {
                id: element.id.clone(),
                name: element.name.clone(),
                subject_id: element.subject_name.clone(),
                created_at: element.created_at,
                updated_at: element.updated_at,
            })
            .get_result::<tables::Chapter>(&mut conn)
            .unwrap();
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
    axum::extract::Json(json): axum::extract::Json<models::Chapter>,
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

    let _result: tables::Chapter = tables::Chapter {
        id: json.id.clone(),
        name: json.name.clone(),
        subject_id: json.subject_id.clone(),
        created_at: json.created_at,
        updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
    }
    .save_changes(&mut conn)
    .unwrap();

    // UNWRAP: just sent back `input_data` as result
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        axum::Json(json),
    )
        .into_response());
}
