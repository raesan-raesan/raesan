// imports
use crate::{core::app, handlers::templates};
use actix_web;
use askama::Template;

// (/create-test) route handler
#[actix_web::get("")]
async fn route() -> actix_web::Result<actix_web::web::Redirect> {
    // redirect users to first step when they try to access the /create-test page
    return Ok(actix_web::web::Redirect::to("/create-test/1"));
}

// (/create-test/{step_number}) page route handler
#[actix_web::get("/{step_number}")]
async fn page(
    app: actix_web::web::Data<app::Application>,
    path: actix_web::web::Path<u32>,
) -> actix_web::Result<actix_web::HttpResponse> {
    // variable declarations
    let step_number = path.into_inner();

    // render HTML struct
    let html = match match step_number {
        1 => templates::CreateTestPageStep1 {
            class_list: app.database.get_class_list(),
        }
        .render(),
        2 => templates::CreateTestPageStep2 {}.render(),
        3 => templates::CreateTestPageStep3 {}.render(),
        4 => templates::CreateTestPageStep4 {}.render(),
        5 => templates::CreateTestPageStep5 {}.render(),
        _ => {
            return Ok(actix_web::HttpResponse::BadRequest().body("Bad request"));
        }
    } {
        Ok(safe_html) => safe_html,
        Err(_) => {
            return Ok(actix_web::HttpResponse::InternalServerError().body("Something went wrong!"));
        }
    };

    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}
