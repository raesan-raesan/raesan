// modules
pub mod create_test;
pub mod templates;

// imports
use crate::utils;
use actix_web;
use askama::Template;
use katex;
use mime_guess;

// (/static) route handler
#[actix_web::get("/static/{_:.*}")]
async fn static_route(
    filepath: actix_web::web::Path<String>,
) -> actix_web::Result<actix_web::HttpResponse> {
    // get static file content
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

    // get the file type
    let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

    return Ok(actix_web::HttpResponse::Ok()
        .content_type(file_type)
        .body(file_contents));
}

// (/) home page route handler
#[actix_web::get("/")]
async fn home_page() -> actix_web::Result<actix_web::HttpResponse> {
    // render HTML struct
    let html = match (templates::HomePage {}.render()) {
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
    // render HTML struct
    let html = match (templates::TestPage {
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
