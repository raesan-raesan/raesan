use crate::core;
use axum::{self, response::IntoResponse};
use axum_macros;
use diesel::{self, prelude::*};
use raesan_common::{models, schema, tables};
use rand::{self, prelude::*};
use serde;
use uuid;
// use serde_json;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CreateTestPageMeta {
    classes: Vec<models::Class>,
    subjects: Vec<models::Subject>,
    chapters: Vec<models::Chapter>,
}

// GET (/api/create-test-page-meta) route handler
#[axum_macros::debug_handler]
pub async fn create_test_page_meta(
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

    return Ok(axum::Json(CreateTestPageMeta {
        classes,
        subjects,
        chapters,
    })
    .into_response());
}

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
    let mut questions = schema::questions::dsl::questions
        .filter(schema::questions::chapter_id.eq_any(create_test_input.chapters.clone()))
        .select(tables::Question::as_select())
        .load(&mut conn)
        .expect("Failed to fetch questions");

    let subjects = schema::subjects::dsl::subjects
        .filter(schema::subjects::id.eq_any(create_test_input.subjects.clone()))
        .select(tables::Subject::as_select())
        .load(&mut conn)
        .expect("Failed to fetch subjects");
    let classes = schema::classes::dsl::classes
        .filter(schema::classes::id.eq_any(create_test_input.classes.clone()))
        .select(tables::Class::as_select())
        .load(&mut conn)
        .expect("Failed to fetch classes");

    // make the name of the test
    let mut test_name = String::new();
    // name by classes
    if classes.len() > 1 && test_name.trim().len() == 0 {
        test_name += "Classes: ";
        let mut i = 0;
        classes.iter().for_each(|element| {
            if i == 0 {
                test_name += format!("{}", element.name).as_str();
            } else {
                test_name += format!(", {}", element.name).as_str();
            }
            i += 1;
        });
    } else {
        // name by subjects
        let curr_class = classes
            .iter()
            .find(|element| element.id == subjects[0].class_id)
            .unwrap();
        test_name += format!("Class {} ", curr_class.name).as_str();
        let mut i = 0;
        subjects.iter().for_each(|element| {
            if i == 0 {
                test_name += format!("{}", element.name).as_str();
            } else {
                test_name += format!(", {}", element.name).as_str();
            }
            i += 1;
        });
    }

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

    return Ok((
        axum::http::StatusCode::OK,
        axum::response::Json(raesan_common::models::Test {
            id: uuid::Uuid::new_v4().to_string(),
            date: time::OffsetDateTime::now_utc().unix_timestamp(),
            name: test_name,
            questions,
        }),
    )
        .into_response());
}
