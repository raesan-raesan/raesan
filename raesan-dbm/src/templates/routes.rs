// imports
use askama_axum::Template;
use raesan_common::models;

// ----- `ClassPage` template object
#[derive(Template)]
#[template(path = "routes/class.html")]
pub struct ClassPage {
    pub classes: Vec<models::Class>,
}

// ----- `SubjectPage` template object
#[derive(Template)]
#[template(path = "routes/subject.html")]
pub struct SubjectPage {
    pub classes: Vec<models::Class>,
    pub subjects: Vec<models::Subject>,
}

// ----- `ChapterPage` template object
#[derive(Template)]
#[template(path = "routes/chapter.html")]
pub struct ChapterPage {
    pub subjects: Vec<models::Subject>,
    pub chapters: Vec<models::Chapter>,
}

// ----- `QuestionPage` template object
#[derive(Template)]
#[template(path = "routes/question.html")]
pub struct QuestionPage {
    pub chapters: Vec<models::Chapter>,
    pub questions: Vec<models::Question>,
}
