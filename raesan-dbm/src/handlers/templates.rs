// imports
use askama_axum::Template;

// ----- `ClassPage` template object
#[derive(Template)]
#[template(path = "routes/class.html")]
pub struct ClassPage {}

// ----- `SubjectPage` template object
#[derive(Template)]
#[template(path = "routes/subject.html")]
pub struct SubjectPage {}

// ----- `ChapterPage` template object
#[derive(Template)]
#[template(path = "routes/chapter.html")]
pub struct ChapterPage {}

// ----- `QuestionPage` template object
#[derive(Template)]
#[template(path = "routes/question.html")]
pub struct QuestionPage {}
