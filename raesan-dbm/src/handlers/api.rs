// imports
use axum::{self, response::IntoResponse};

// POST (/api/class) route handler
pub async fn create_class_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE CLASS");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE CLASS"),
    )
        .into_response());
}

// POST (/api/subject) route handler
pub async fn create_subject_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE SUBJECT");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE SUBJECT"),
    )
        .into_response());
}

// POST (/api/chapter) route handler
pub async fn create_chapter_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE CHAPTER");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE CHAPTER"),
    )
        .into_response());
}

// POST (/api/question) route handler
pub async fn create_question_route(
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("CREATE QUESTION");
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        String::from("CREATE QUESTION"),
    )
        .into_response());
}
