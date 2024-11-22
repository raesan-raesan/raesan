use crate::schema;
use diesel;
use serde;

// ----- `ClassTable` model struct
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
#[diesel(table_name=schema::classes)]
pub struct ClassTable {
    pub id: String,
    pub name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `SubjectTable` model struct
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
    diesel::Associations,
)]
#[diesel(belongs_to(ClassTable))]
#[diesel(table_name=schema::subjects)]
pub struct SubjectTable {
    pub id: String,
    pub name: String,
    pub class_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `ChapterTable` model struct
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
    diesel::Associations,
)]
#[diesel(belongs_to(SubjectTable))]
#[diesel(table_name=schema::chapters)]
pub struct ChapterTable {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `QuestionTable` model struct
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
    diesel::Associations,
)]
#[diesel(belongs_to(ChapterTable))]
#[diesel(table_name=schema::questions)]
pub struct QuestionTable {
    pub id: String,
    pub body: String,
    pub chapter_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `CreateTestInppt` struct
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

// ----- `TestQuestion` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestQuestion {
    pub id: String,
    pub body: String,
}
