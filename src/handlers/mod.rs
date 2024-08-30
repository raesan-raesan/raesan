// modules
pub mod api;

// imports
use actix_web;
use askama_actix::Template;
use katex;

// ----- HomePage template object
#[derive(Template)]
#[template(path = "routes/index.html")]
struct HomePage {}
// ----- TestPage template object
#[derive(Template)]
#[template(path = "routes/test.html")]
struct TestPage {
    latex_content: String,
}

// (/) home page route handler
#[actix_web::get("/")]
async fn home_page() -> actix_web::Result<actix_web::HttpResponse> {
    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HomePage {}.render().unwrap()));
}

// (/test) route handler
#[actix_web::get("/test")]
async fn test_page() -> actix_web::Result<actix_web::HttpResponse> {
    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(TestPage{
                latex_content: katex::render_with_opts("\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
                    katex::Opts::builder()
                        .output_type(katex::OutputType::Mathml)
                        .build()
                        .unwrap(),
                ).unwrap()
        }.render().unwrap()));
}
