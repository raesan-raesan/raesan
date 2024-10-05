// imports
use diesel;
use raesan_common;
use serde;

// ----- `Classes` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Classes {
    pub classes: Vec<Class>,
}

// ----- `Class` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::class)]
pub struct Class {
    pub id: String,
    pub name: i32,
}

// ----- `Subjects` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subjects {
    pub subjects: Vec<Subject>,
}

// ----- `Subject` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::subject)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub class_id: String,
}

// ----- `Chapters` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapters {
    pub chapters: Vec<Chapter>,
}

// ----- `Chapter` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::chapter)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub subject_id: String,
}

// ----- `Question` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::question)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_id: String,
}

// ----- `CreateTestInput` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateTestInput {
    pub curr_step: u32,
    pub classes: Vec<String>,
    pub subjects: Vec<String>,
    pub chapters: Vec<String>,
    pub format: TestFormatInput,
}

// ----- `TestFormatInput` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestFormatInput {
    pub total_questions: u32,
}
