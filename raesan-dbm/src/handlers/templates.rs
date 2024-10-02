// imports
use askama_axum::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}
