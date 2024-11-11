// imports
use crate::core;
use diesel::{self, prelude::*};
use r2d2;
use raesan_common::schema;
use serde_json;
use std::{fs, io, path::Path};

// generate database records for testing
pub fn generate_database_records(data: core::app::GenerateDatabaseRecords) -> Result<(), String> {
    let database = match core::database::Database::new(data.database) {
        Ok(safe_db) => safe_db,
        Err(e) => return Err(e.to_string()),
    };
    let mut conn = match database.pool.get() {
        Ok(safe_conn) => safe_conn,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    // classes
    let mut classes_json_string = String::new();
    let classes_json_file = format!("{}/classes.json", data.dataset);
    match fs::metadata(&classes_json_file) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(&classes_json_file) {
                    Ok(safe_contents) => {
                        classes_json_string = safe_contents.to_string();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    };
    match diesel::insert_into(schema::classes::dsl::classes)
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

    println!("Successfully created class records from the dataset");

    // subjects
    let mut subjects_json_string = String::new();
    let subjects_json_file = format!("{}/subjects.json", data.dataset);
    match fs::metadata(&subjects_json_file) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(&subjects_json_file) {
                    Ok(safe_contents) => {
                        subjects_json_string = safe_contents.to_string();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    };
    let classes = match schema::classes::dsl::classes
        .select(core::models::Class::as_select())
        .load(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::subjects::dsl::subjects)
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
    println!("Successfully created subject records from the dataset");

    // chapters
    let chapters_dir = format!("{}/chapters", data.dataset);
    let chapters_path = Path::new(&chapters_dir);
    if chapters_path.is_dir() {
        match fs::read_dir(chapters_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let loop_conn = match database.pool.get() {
                            Ok(safe_conn) => safe_conn,
                            Err(e) => return Err(e.to_string()),
                        };
                        match insert_chapters(loop_conn, entry.path().to_string_lossy().to_string())
                        {
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
    println!("Successfully created chapter records from the dataset");

    // questions
    let questions_dir = format!("{}/questions", data.dataset);
    let questions_path = Path::new(&questions_dir);
    if questions_path.is_dir() {
        match fs::read_dir(questions_path) {
            Ok(subjects) => {
                for subject in subjects {
                    if let Ok(subject) = subject {
                        match fs::read_dir(subject.path()) {
                            Ok(chapters) => {
                                for chapter in chapters {
                                    if let Ok(chapter) = chapter {
                                        let loop_conn = match database.pool.get() {
                                            Ok(safe_conn) => safe_conn,
                                            Err(e) => return Err(e.to_string()),
                                        };
                                        match insert_questions(
                                            loop_conn,
                                            chapter.path().to_string_lossy().to_string(),
                                        ) {
                                            Ok(_) => {}
                                            Err(e) => return Err(e.to_string()),
                                        }
                                    }
                                }
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    } else {
        return Err("The provided path for generating database records of chapters table is not a directory".to_string());
    }
    println!("Successfully created chapter records from the dataset");

    // print the final database state
    let results = raesan_common::schema::classes::dsl::classes
        .select(core::models::Class::as_select())
        .load(&mut conn)
        .expect("Error loading classes");
    println!("Classes: {:#?}", results);
    let results = raesan_common::schema::subjects::dsl::subjects
        .select(core::models::Subject::as_select())
        .load(&mut conn)
        .expect("Error loading subjects");
    println!("Subjects: {:#?}", results);
    let results = raesan_common::schema::chapters::dsl::chapters
        .select(core::models::Chapter::as_select())
        .load(&mut conn)
        .expect("Error loading chapters");
    println!("Chapters: {:#?}", results);
    let results = raesan_common::schema::questions::dsl::questions
        .select(core::models::Question::as_select())
        .load(&mut conn)
        .expect("Error loading questions");
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
    let curr_subject: core::models::Subject = match schema::subjects::dsl::subjects
        .filter(schema::subjects::class_name.eq(chapters_json_vec[0].class_name))
        .filter(schema::subjects::name.eq(chapters_json_vec[0].subject_name.clone()))
        .select(core::models::Subject::as_select())
        .first(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::chapters::dsl::chapters)
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

pub fn insert_questions(
    mut conn: r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>,
    >,
    questions_json_file: String,
) -> Result<(), String> {
    let mut questions_json_string = String::new();
    match fs::metadata(questions_json_file.clone()) {
        Ok(safe_metadata) => {
            if safe_metadata.is_file() {
                match fs::read_to_string(questions_json_file.clone()) {
                    Ok(safe_contents) => {
                        questions_json_string = safe_contents.to_string();
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::UnexpectedEof {
                            return Ok(());
                        } else {
                            return Err(e.to_string());
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    };
    if questions_json_string.trim().is_empty() {
        return Ok(());
    }
    let questions_json_vec =
        match serde_json::from_str::<Vec<core::models::Question>>(questions_json_string.as_str()) {
            Ok(safe_questions_vec) => safe_questions_vec,
            Err(e) => {
                return Err(e.to_string());
            }
        };
    let curr_chapter: core::models::Chapter = match schema::chapters::dsl::chapters
        .filter(schema::chapters::class_name.eq(questions_json_vec[0].class_name))
        .filter(schema::chapters::subject_name.eq(questions_json_vec[0].subject_name.clone()))
        .filter(schema::chapters::name.eq(questions_json_vec[0].chapter_name.clone()))
        .select(core::models::Chapter::as_select())
        .first(&mut conn)
    {
        Ok(safe_results) => safe_results,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    match diesel::insert_into(schema::questions::dsl::questions)
        .values(
            questions_json_vec
                .clone()
                .into_iter()
                .map(|mut element| {
                    element.id = uuid::Uuid::new_v4().to_string();
                    element.chapter_id = curr_chapter.id.clone();
                    element
                })
                .collect::<Vec<core::models::Question>>(),
        )
        .execute(&mut conn)
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e.to_string());
        }
    }
}
