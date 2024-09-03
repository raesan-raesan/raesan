// imports
use crate::{core::app, utils};
use actix_web;
use askama_actix::Template;
use katex;
use mime_guess;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
struct HomePage {}
// ----- `CreateTestPage` template object
#[derive(Template)]
#[template(path = "routes/create-test.html")]
struct CreateTestPage {
    class_list: Vec<String>,
    subject_list: Vec<String>,
}
// ----- `TestPage` template object
#[derive(Template)]
#[template(path = "routes/test.html")]
struct TestPage {
    latex_content: String,
}

// (/static) route handler
#[actix_web::get("/static/{_:.*}")]
async fn static_route(
    filepath: actix_web::web::Path<String>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let file_contents = match utils::get_embedded_file(filepath.to_string()) {
        Some(some_file_contents) => match some_file_contents {
            Ok(safe_file_contents) => safe_file_contents,
            Err(_) => {
                return Ok(
                    actix_web::HttpResponse::InternalServerError().body("Internal Server Error!")
                )
            }
        },
        None => return Ok(actix_web::HttpResponse::NotFound().body("404 Not Found")),
    };

    let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

    return Ok(actix_web::HttpResponse::Ok()
        .content_type(file_type)
        .body(file_contents));
}

// (/) home page route handler
#[actix_web::get("/")]
async fn home_page() -> actix_web::Result<actix_web::HttpResponse> {
    let html = match (HomePage {}.render()) {
        Ok(safe_html) => safe_html,
        Err(_) => {
            return Ok(actix_web::HttpResponse::InternalServerError().body("Something went wrong!"));
        }
    };

    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

// (/create-test) home page route handler
#[actix_web::get("/create-test")]
async fn create_test_page(
    app: actix_web::web::Data<app::Application>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let html = match (CreateTestPage {
        class_list: app.database.get_class_list(),
        subject_list: app.database.get_subject_list(),
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(_) => {
            return Ok(actix_web::HttpResponse::InternalServerError().body("Something went wrong!"));
        }
    };

    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}

// (/test) route handler
#[actix_web::get("/test")]
async fn test_page() -> actix_web::Result<actix_web::HttpResponse> {
    let html = match (TestPage {
        latex_content: katex::render_with_opts(
            "\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
            katex::Opts::builder()
                .output_type(katex::OutputType::Mathml)
                .build()
                .unwrap(),
        )
        .unwrap(),
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(_) => {
            return Ok(actix_web::HttpResponse::InternalServerError().body("Something went wrong!"));
        }
    };

    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html));
}
