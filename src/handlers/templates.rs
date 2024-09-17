// imports
use askama_axum::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `CreateTestPage` template objects
#[derive(Template)]
#[template(path = "routes/create-test.html")]
pub struct CreateTestPage {
    pub class_list: Vec<String>,
}

// ----- `TestPage` template object
#[derive(Template)]
#[template(path = "routes/test.html")]
pub struct TestPage {
    pub latex_content: String,
}
