mod core;
mod handlers;

use actix_files;
use actix_web;
use askama_actix::Template;
use katex;

#[derive(Template)]
#[template(path = "routes/index.html")]
struct HomePage {
    latex_content: String,
}

async fn home_page() -> actix_web::Result<actix_web::HttpResponse> {
    return Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            HomePage {
                latex_content: katex::render_with_opts("\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
                    katex::Opts::builder()
                        .output_type(katex::OutputType::Mathml)
                        .build()
                        .unwrap(),
                )
                .unwrap(),
            }
            .render()
            .unwrap(),
        ));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(|| {
        let app = actix_web::App::new()
            .service(actix_files::Files::new("/static", "./static")) // static files
            .route("/", actix_web::web::get().to(home_page));
        return app;
    });
    return server.bind((core::ADDRESS, core::PORT))?.run().await;
}
