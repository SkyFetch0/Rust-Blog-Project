use actix_web::web;
use crate::handlers::blogs;

pub fn blog_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(blogs::index))
            .route("/blog", web::get().to(blogs::blogs_page))
            .route("/blog/", web::get().to(blogs::blogs_page))
            .route("/blog/{slug}", web::get().to(blogs::postinfo))
    );
}