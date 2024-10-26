// imports
use crate::core;
use askama_axum::Template;

// ----- `ClassPage` template object
#[derive(Template)]
#[template(path = "routes/class.html")]
pub struct ClassPage {
    pub classes: Vec<core::models::Class>,
}

// ----- `SubjectPage` template object
#[derive(Template)]
#[template(path = "routes/subject.html")]
pub struct SubjectPage {
    pub classes: Vec<core::models::Class>,
    pub subjects: Vec<core::models::Subject>,
}

// ----- `ChapterPage` template object
#[derive(Template)]
#[template(path = "routes/chapter.html")]
pub struct ChapterPage {
    pub subjects: Vec<core::models::Subject>,
    pub chapters: Vec<core::models::Chapter>,
}

// ----- `QuestionPage` template object
#[derive(Template)]
#[template(path = "routes/question.html")]
pub struct QuestionPage {
    pub chapters: Vec<core::models::Chapter>,
    pub questions: Vec<core::models::Question>,
}
