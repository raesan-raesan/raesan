use crate::core;
use axum::{self, response::IntoResponse};
use axum_macros;
use diesel::{self, prelude::*};
use raesan_common::{models, schema, tables};
use rand::{self, prelude::*};
use std::sync::{Arc, RwLock};
use uuid;

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
