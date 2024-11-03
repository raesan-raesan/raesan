// imports
use diesel;
use raesan_common;
use serde;

// ----- `Class` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=raesan_common::schema::classes)]
pub struct Class {
    pub id: String,
    pub name: i32,
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
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=raesan_common::schema::subjects)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub class_id: String,
    pub class_name: i32,
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
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=raesan_common::schema::chapters)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub subject_id: String,
    pub subject_name: String,
    pub class_name: i32,
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
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=raesan_common::schema::questions)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_name: String,
    pub subject_name: String,
    pub class_name: i32,
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
