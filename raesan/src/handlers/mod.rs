// models
pub mod api;
pub mod test_route;

// imports
use crate::{core, templates, utils};
use askama::Template;
use axum::{self, response::IntoResponse};
use diesel::{self, prelude::*};
use mime_guess;
use raesan_common::{models, schema, tables};
// use serde_json;
use std::sync::{Arc, RwLock};

// GET (/static) route handler
pub async fn static_route(
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // get static file content
    let file_contents = match utils::get_embedded_file(filepath.to_string()) {
        Some(some_file_contents) => match some_file_contents {
            Ok(safe_file_contents) => safe_file_contents,
            Err(e) => {
                println!(
                    "Failed to convert file contents into readable string format, Error: {:#?}",
                    e
                );
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to convert file contents into readable string format"),
                ));
            }
        },
        None => {
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                String::from("404 Not Found!"),
            ))
        }
    };

    // get the file type
    let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

    return Ok((
        [(axum::http::header::CONTENT_TYPE, file_type.to_string())],
        file_contents,
    )
        .into_response());
}

// GET (/) home page route handler
pub async fn home_page() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct
    let html = match (templates::routes::HomePage {}.render()) {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}

// GET (/create-test) route handlers
pub async fn create_test_page(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
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

    let classes = schema::classes::dsl::classes
        .select(tables::Class::as_select())
        .load(&mut conn)
        .expect("Error loading classes")
        .iter()
        .map(|element| models::Class {
            id: element.id.clone(),
            name: element.name,
            created_at: element.created_at,
            updated_at: element.updated_at,
        })
        .collect::<Vec<models::Class>>();
    let subjects = schema::subjects::dsl::subjects
        .select(tables::Subject::as_select())
        .load(&mut conn)
        .expect("Error loading subjects")
        .iter()
        .map(|element| {
            let curr_class = classes
                .iter()
                .find(|_class| _class.id == element.class_id)
                .unwrap();
            models::Subject {
                id: element.id.clone(),
                name: element.name.clone(),
                display_name: format!("{} - {}", curr_class.name.clone(), element.name),
                class_id: element.class_id.clone(),
                class_name: curr_class.name.clone(),
                created_at: element.created_at,
                updated_at: element.updated_at,
            }
        })
        .collect::<Vec<models::Subject>>();
    let chapters = schema::chapters::dsl::chapters
        .select(tables::Chapter::as_select())
        .load(&mut conn)
        .expect("Error loading chapters")
        .iter()
        .map(|element| {
            let curr_subject = subjects
                .iter()
                .find(|subject| subject.id == element.subject_id)
                .unwrap();
            let curr_class = classes
                .iter()
                .find(|_class| _class.id == curr_subject.class_id)
                .unwrap();
            models::Chapter {
                id: element.id.clone(),
                name: element.name.clone(),
                display_name: format!(
                    "{} - {} - {}",
                    curr_class.name.clone(),
                    curr_subject.name.clone(),
                    element.name
                ),
                subject_id: element.subject_id.clone(),
                subject_name: curr_subject.name.clone(),
                class_name: curr_class.name.clone(),
                created_at: element.created_at,
                updated_at: element.updated_at,
            }
        })
        .collect::<Vec<models::Chapter>>();

    let html = match (templates::routes::CreateTestPage {
        dataset_classes: classes,
        dataset_subjects: subjects,
        dataset_chapters: chapters,
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error: {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
