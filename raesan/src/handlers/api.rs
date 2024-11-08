use crate::core::models;
use axum::{self, response::IntoResponse};
use axum_extra;
use serde_json;
use time;

// POST (/api/create-test) route handler
pub async fn create_test_route(
    axum::extract::Json(create_test_input): axum::extract::Json<models::CreateTestInput>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let cookie = axum_extra::extract::cookie::Cookie::build((
        "create_test_input",
        serde_json::to_string(&create_test_input).unwrap(),
    ))
    .path("/")
    .expires(time::OffsetDateTime::now_utc() + time::Duration::days(30));

    let cookie_jar = axum_extra::extract::cookie::CookieJar::new().add(cookie.clone());
    return Ok((cookie_jar, axum::http::StatusCode::OK).into_response());
}
