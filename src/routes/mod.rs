mod api;
mod blogs;

use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "errors/404.html")]
struct NotFoundTemplate {}

async fn not_found() -> HttpResponse {
    let template = NotFoundTemplate {};
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(template.render().unwrap_or_else(|_| String::from("404 - Sayfa Bulunamadı")))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(api::api_routes)
            .configure(blogs::blog_routes)
    )
        .default_service(
            web::route().to(not_found)
        );
}