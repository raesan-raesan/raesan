// modules
mod core;
mod handlers;
mod utils;

// imports
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // main actix_web server
    let server = actix_web::HttpServer::new(|| {
        return actix_web::App::new()
            .app_data(actix_web::web::Data::new(core::app::Application::new()))
            .service(handlers::static_route) // server static files
            .service(handlers::home_page)
            .service(handlers::create_test_route)
            .service(handlers::create_test_page)
            .service(handlers::test_page);
    });

    println!("Running on {}:{}", core::ADDRESS, core::PORT);

    // running the server after binding it to the specific address and port
    return server.bind((core::ADDRESS, core::PORT))?.run().await;
}
