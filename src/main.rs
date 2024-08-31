// modules
mod core;
mod handlers;
mod utils;

// imports
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // base application
    let app = core::app::Application::new(core::app::Config {
        port: core::PORT,
        address: core::ADDRESS.to_string(),
    });

    // main actix_web server
    let server = actix_web::HttpServer::new(|| {
        return actix_web::App::new()
            .service(handlers::static_route) // server static files
            .service(handlers::home_page)
            .service(handlers::test_page)
            .service(handlers::api::index_route);
    });

    println!("Running on {}:{}", app.config.address, app.config.port);

    // running the server after binding it to the specific address and port
    return server
        .bind((app.config.address, app.config.port))?
        .run()
        .await;
}
