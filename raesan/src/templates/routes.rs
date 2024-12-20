// imports
use askama_axum::Template;
use raesan_common::models;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `CreateTestPage` template objects
#[derive(Template)]
#[template(path = "routes/create-test.html")]
pub struct CreateTestPage {
    pub dataset_classes: Vec<models::Class>,
    pub dataset_subjects: Vec<models::Subject>,
    pub dataset_chapters: Vec<models::Chapter>,
}

// ----- `TestPage` template object
#[derive(Template)]
#[template(path = "routes/test.html")]
pub struct TestPage {
    pub test_id: String,
}
