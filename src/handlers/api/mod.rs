// imports
use actix_web;

// (/api) route handler
#[actix_web::get("/api")]
async fn index_route() -> actix_web::Result<actix_web::HttpResponse> {
    return Ok(actix_web::HttpResponse::Ok().body("raetest API"));
}
