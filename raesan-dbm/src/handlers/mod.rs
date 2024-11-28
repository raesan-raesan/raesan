// models
pub mod api;

// imports
use crate::{core, templates, utils};
use askama::Template;
use axum::{self, response::IntoResponse};
use diesel::{self, prelude::*};
use mime_guess;
use raesan_common::{self, models, schema, tables};
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
    return Ok(axum::response::Redirect::to("/class").into_response());
}

// GET(/class) page route handler
pub async fn class_page(
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

    let results = raesan_common::schema::classes::dsl::classes
        .limit(core::PAGE_SIZE.into())
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
        .collect::<Vec<_>>();

    // render HTML struct
    let html = match (templates::routes::ClassPage { classes: results }.render()) {
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

// GET(/subject) page route handler
pub async fn subject_page(
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

    let classes = raesan_common::schema::classes::dsl::classes
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
        .collect::<Vec<_>>();
    let subjects = raesan_common::schema::subjects::dsl::subjects
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
                display_name: format!("{} - {}", curr_class.name, element.name.clone()),
                class_id: element.class_id.clone(),
                class_name: curr_class.name,
                created_at: element.created_at,
                updated_at: element.updated_at,
            }
        })
        .collect::<Vec<_>>();

    // render HTML struct
    let html = match (templates::routes::SubjectPage { classes, subjects }.render()) {
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

// GET(/chapter) page route handler
pub async fn chapter_page(
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

    let classes = raesan_common::schema::classes::dsl::classes
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
        .collect::<Vec<_>>();
    let subjects = raesan_common::schema::subjects::dsl::subjects
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
                display_name: format!("{} - {}", curr_class.name, element.name.clone()),
                class_id: element.class_id.clone(),
                class_name: curr_class.name,
                created_at: element.created_at,
                updated_at: element.updated_at,
            }
        })
        .collect::<Vec<_>>();
    let chapters = raesan_common::schema::chapters::dsl::chapters
        .limit(core::PAGE_SIZE.into())
        .select(tables::Chapter::as_select())
        .load(&mut conn)
        .unwrap()
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
                    curr_class.name,
                    curr_subject.name.clone(),
                    element.name.clone()
                ),
                subject_id: element.subject_id.clone(),
                subject_name: curr_subject.name.clone(),
                class_name: curr_class.name,
                created_at: element.created_at,
                updated_at: element.updated_at,
            }
        })
        .collect::<Vec<_>>();

    // render HTML struct
    let html = match (templates::routes::ChapterPage { subjects, chapters }.render()) {
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

// GET(/question) page route handler
pub async fn question_page(
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

    let chapters = schema::chapters::table
        .inner_join(
            schema::subjects::table.on(schema::chapters::subject_id.eq(schema::subjects::id)),
        )
        .inner_join(schema::classes::table.on(schema::subjects::class_id.eq(schema::classes::id)))
        .select((
            schema::chapters::all_columns,
            schema::subjects::all_columns,
            schema::classes::all_columns,
        ))
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

    let questions = schema::questions::table
        .inner_join(
            schema::chapters::table.on(schema::questions::chapter_id.eq(schema::chapters::id)),
        )
        .inner_join(
            schema::subjects::table.on(schema::chapters::subject_id.eq(schema::subjects::id)),
        )
        .inner_join(schema::classes::table.on(schema::subjects::class_id.eq(schema::classes::id)))
        .select((
            schema::questions::all_columns,
            schema::chapters::all_columns,
            schema::subjects::all_columns,
            schema::classes::all_columns,
        ))
        .limit(core::PAGE_SIZE.into())
        .load::<(
            tables::Question,
            tables::Chapter,
            tables::Subject,
            tables::Class,
        )>(&mut conn)
        .unwrap()
        .iter()
        .map(|element| models::Question {
            id: element.0.id.clone(),
            body: element.0.body.clone(),
            chapter_id: element.0.chapter_id.clone(),
            chapter_name: element.1.name.clone(),
            subject_name: element.2.name.clone(),
            class_name: element.3.name,
            created_at: element.0.created_at,
            updated_at: element.0.updated_at,
        })
        .collect::<Vec<models::Question>>();

    // render HTML struct
    let html = match (templates::routes::QuestionPage {
        chapters,
        questions,
    }
    .render())
    {
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
