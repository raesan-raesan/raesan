// imports
use crate::core;
use diesel::{self, prelude::*};
use r2d2;
use raesan_common::schema;
use rust_embed;
use serde_json;
use std::{
    fs,
    path::Path,
    sync::{Arc, RwLock},
};

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
#[allow(dead_code)]
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

    // classes
    let mut classes_json_string = String::new();
    let classes_json_file = "dataset/_classes.json";
    match fs::metadata(classes_json_file) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(classes_json_file) {
                    Ok(safe_contents) => {
                        classes_json_string = safe_contents.to_string();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    };
    match diesel::insert_into(schema::class::dsl::class)
        .values(
            match serde_json::from_str::<Vec<core::models::Class>>(classes_json_string.as_str()) {
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
    };

    // subjects
    let mut subjects_json_string = String::new();
    let subjects_json_file = "dataset/_subjects.json";
    match fs::metadata(subjects_json_file) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(subjects_json_file) {
                    Ok(safe_contents) => {
                        subjects_json_string = safe_contents.to_string();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    };
    let classes = match schema::class::dsl::class
        .limit(2)
        .select(core::models::Class::as_select())
        .load(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::subject::dsl::subject)
        .values(
            match serde_json::from_str::<Vec<core::models::Subject>>(subjects_json_string.as_str())
            {
                Ok(safe_subject_vec) => safe_subject_vec,
                Err(e) => return Err(e.to_string()),
            }
            .into_iter()
            .map(|mut element| {
                element.id = uuid::Uuid::new_v4().to_string();
                element.class_id = classes
                    .iter()
                    .find(|class_element| class_element.name == element.class_name)
                    .unwrap()
                    .clone()
                    .id;
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

    // chapters
    let chapters_path = Path::new("dataset/chapters");
    if chapters_path.is_dir() {
        match fs::read_dir(chapters_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let loop_conn = match match app_state.write() {
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
                        match insert_chapters(
                            loop_conn,
                            String::from("dataset/chapters/")
                                + entry.file_name().to_string_lossy().to_string().as_str(),
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    } else {
        return Err("The provided path for generating database records of chapters table is not a directory".to_string());
    }

    // print the final database state
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
    let results = raesan_common::schema::chapter::dsl::chapter
        .select(core::models::Chapter::as_select())
        .load(&mut conn)
        .expect("Error loading chapters");
    println!("Chapters: {:#?}", results);
    return Ok(());
}

pub fn insert_chapters(
    mut conn: r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>,
    >,
    chapters_json_file: String,
) -> Result<(), String> {
    let mut chapters_json_string = String::new();
    match fs::metadata(chapters_json_file.clone()) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(chapters_json_file.clone()) {
                    Ok(safe_contents) => {
                        chapters_json_string = safe_contents.to_string();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    };
    let chapters_json_vec =
        match serde_json::from_str::<Vec<core::models::Chapter>>(chapters_json_string.as_str()) {
            Ok(safe_chapter_vec) => safe_chapter_vec,
            Err(e) => return Err(e.to_string()),
        };
    let curr_subject: core::models::Subject = match schema::subject::dsl::subject
        .filter(schema::subject::class_name.eq(chapters_json_vec[0].class_name))
        .filter(schema::subject::name.eq(chapters_json_vec[0].subject_name.clone()))
        .select(core::models::Subject::as_select())
        .first(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::chapter::dsl::chapter)
        .values(
            chapters_json_vec
                .clone()
                .into_iter()
                .map(|mut element| {
                    element.id = uuid::Uuid::new_v4().to_string();
                    element.subject_id = curr_subject.id.clone();
                    element
                })
                .collect::<Vec<core::models::Chapter>>(),
        )
        .execute(&mut conn)
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e.to_string());
        }
    }
}
