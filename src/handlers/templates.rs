// imports
use crate::core::models;
use askama_axum::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `CreateTestPage` template objects
#[derive(Template)]
#[template(path = "routes/create-test.html")]
pub struct CreateTestPage {
    pub dataset: models::Classes,
}

// ----- `TestPage` template object
#[derive(Template)]
#[template(path = "routes/test.html")]
pub struct TestPage {
    pub latex_content: String,
}
