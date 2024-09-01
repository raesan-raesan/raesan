use serde;

// ----- `Classes` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Classes {
    pub classes: Vec<Class>,
}
// ----- `Class` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Class {
    pub id: u32,
    pub subjects: Vec<Subject>,
}
// ----- `Subject` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    pub name: String,
    pub chapters: Vec<Chapter>,
}
// ----- `Chapter` model struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: u32,
    pub name: String,
}
