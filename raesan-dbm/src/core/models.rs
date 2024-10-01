// imports
use serde;

// ----- `Classes` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Classes {
    pub classes: Vec<Class>,
}
// ----- `Class` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub id: String,
    pub name: u32,
}
// ----- `Subjects` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subjects {
    pub subjects: Vec<Subject>,
}
// ----- `Subject` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub subject_id: String,
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

// ----- `ChapterJSON` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChapterJSON {
    pub id: String,
    pub name: String,
    pub questions: Vec<Question>,
}

// ----- `Question` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Question {
    pub id: String,
    pub chapter_id: String,
    pub body: String,
    pub kind: String,
    pub answer: String,
    pub solution: String,
}

// ----- `Test` struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Test {
    pub id: String,
    pub total_questions: u32,
    pub questions: Vec<Question>,
}
