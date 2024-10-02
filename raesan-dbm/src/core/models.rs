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
