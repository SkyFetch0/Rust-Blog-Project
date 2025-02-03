use actix_web::{web, HttpResponse};
use crate::handlers::api;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
           .route("/test", web::get().to(|| async {
                HttpResponse::Ok().body("API Test Working!")
            }))
            .service(

                web::scope("/comments")
                    .route("/get/{post_id}", web::get().to(api::comments))
                    .route("/add", web::post().to(api::addcomment))

            )
    );
}