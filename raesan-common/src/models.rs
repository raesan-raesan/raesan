use serde;

// ----- `Class` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub id: String,
    pub name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub class_id: String,
    pub class_name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `Chapter` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub subject_id: String,
    pub subject_name: String,
    pub class_name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `Question` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_name: String,
    pub subject_name: String,
    pub class_name: i32,
    pub chapter_id: String,
    pub created_at: i64,
    pub updated_at: i64,
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

// ----- `TestQuestion` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestQuestion {
    pub id: String,
    pub body: String,
}
