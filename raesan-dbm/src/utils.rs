// imports
use crate::core;
use diesel::{self, prelude::*};
use raesan_common::schema;
use rust_embed;
use serde_json;
use std::sync::{Arc, RwLock};

// ----- `StaticAssets` object
#[derive(rust_embed::Embed)]
#[folder = "static"]
pub struct StaticAssets;

// get files contents from embedded files i.e `static` directory
pub fn get_embedded_file(filepath: String) -> Option<Result<String, String>> {
    match StaticAssets::get(filepath.as_str()) {
        Some(file_content) => {
            return Some(match String::from_utf8(file_content.data.to_vec()) {
                Ok(safe_value) => Ok(safe_value),
                Err(e) => Err(e.to_string()),
            });
        }
        None => {
            return None;
        }
    }
}

// generate database records for testing
pub fn generate_database_records_for_testing(
    app_state: Arc<RwLock<core::app::Application>>,
) -> Result<(), String> {
    let mut conn = match match app_state.write() {
        Ok(safe_app_state) => safe_app_state,
        Err(e) => {
            return Err(e.to_string());
        }
    }
    .database
    .pool
    .get()
    {
        Ok(safe_conn) => safe_conn,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let classes_json_string = r#"[
        {
            "id": "",
            "name": 11
        },
        {
            "id": "",
            "name": 12
        }
        ]"#;

    match diesel::insert_into(schema::class::dsl::class)
        .values(
            match serde_json::from_str::<Vec<core::models::Class>>(classes_json_string) {
                Ok(safe_class_vec) => safe_class_vec,
                Err(e) => return Err(e.to_string()),
            }
            .into_iter()
            .map(|mut element| {
                element.id = uuid::Uuid::new_v4().to_string();
                element
            })
            .collect::<Vec<core::models::Class>>(),
        )
        .execute(&mut conn)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let class_11_subjects_json_string = r#"[
        {
          "id": "",
          "name": "Physics",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Chemistry",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Maths",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Biology",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "English",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Physical Education",
          "class_id": "",
          "class_name": 0
        }
    ]"#;

    let class_11 = match schema::class::dsl::class
        .filter(schema::class::name.eq(11))
        .limit(5)
        .select(core::models::Class::as_select())
        .first(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::subject::dsl::subject)
        .values(
            match serde_json::from_str::<Vec<core::models::Subject>>(class_11_subjects_json_string)
            {
                Ok(safe_class_vec) => safe_class_vec,
                Err(e) => return Err(e.to_string()),
            }
            .into_iter()
            .map(|mut element| {
                element.id = uuid::Uuid::new_v4().to_string();
                element.class_id = class_11.id.to_string();
                element.class_name = class_11.name;
                element
            })
            .collect::<Vec<core::models::Subject>>(),
        )
        .execute(&mut conn)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let class_12_subjects_json_string = r#"[
        {
          "id": "",
          "name": "Physics",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Chemistry",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Maths",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Biology",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "English",
          "class_id": "",
          "class_name": 0
        },
        {
          "id": "",
          "name": "Physical Education",
          "class_id": "",
          "class_name": 0
        }
    ]"#;

    let class_12 = match schema::class::dsl::class
        .filter(schema::class::name.eq(12))
        .limit(5)
        .select(core::models::Class::as_select())
        .first(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::subject::dsl::subject)
        .values(
            match serde_json::from_str::<Vec<core::models::Subject>>(class_12_subjects_json_string)
            {
                Ok(safe_class_vec) => safe_class_vec,
                Err(e) => return Err(e.to_string()),
            }
            .into_iter()
            .map(|mut element| {
                element.id = uuid::Uuid::new_v4().to_string();
                element.class_id = class_12.id.to_string();
                element.class_name = class_12.name;
                element
            })
            .collect::<Vec<core::models::Subject>>(),
        )
        .execute(&mut conn)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let results = raesan_common::schema::class::dsl::class
        .select(core::models::Class::as_select())
        .load(&mut conn)
        .expect("Error loading classes");
    println!("Classes: {:#?}", results);
    let results = raesan_common::schema::subject::dsl::subject
        .select(core::models::Subject::as_select())
        .load(&mut conn)
        .expect("Error loading subjects");
    println!("Subjects: {:#?}", results);
    return Ok(());
}
