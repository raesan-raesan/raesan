use crate::handlers::templates;
use askama::Template;
use axum::{self, response::IntoResponse};

// GET (/test/:test_id) route handler
pub async fn route(
    axum::extract::Path(test_id): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    println!("Test ID: {:#?}", test_id);
    // render HTML struct (PS: this whole thing upto the return, is a single let statement)
    let html = match (templates::TestPage {
        latex_content: match katex::render_with_opts(
            "\\frac{\\pi}{\\oint x^2 dx} \\oint \\frac{\\sin(\\phi)}{\\tan(\\phi - \\theta)} dx",
            match katex::Opts::builder()
                .output_type(katex::OutputType::Mathml)
                .build()
            {
                Ok(safe_builder_opts) => safe_builder_opts,
                Err(e) => {
                    println!("Failed to build latex builder options, Error: {:#?}", e);
                    return Err((
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        String::from("Failed to build latex builder options"),
                    ));
                }
            },
        ) {
            Ok(safe_latex) => safe_latex,
            Err(e) => {
                println!("Failed to render latex content, Error: {:#?}", e);
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to render latex content"),
                ));
            }
        },
    }
    .render())
    {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error: {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
        .into_response());
}
