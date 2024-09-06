use askama::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `CreateTestPage` template objects
#[derive(Template)]
#[template(path = "routes/create-test/1.html")]
pub struct CreateTestPageStep1 {
    pub class_list: Vec<String>,
}
#[derive(Template)]
#[template(path = "routes/create-test/2.html")]
pub struct CreateTestPageStep2 {}
#[derive(Template)]
#[template(path = "routes/create-test/3.html")]
pub struct CreateTestPageStep3 {}
#[derive(Template)]
#[template(path = "routes/create-test/4.html")]
pub struct CreateTestPageStep4 {}
#[derive(Template)]
#[template(path = "routes/create-test/5.html")]
pub struct CreateTestPageStep5 {}

// ----- `TestPage` template object
#[derive(Template)]
#[template(path = "routes/test.html")]
pub struct TestPage {
    pub latex_content: String,
}
