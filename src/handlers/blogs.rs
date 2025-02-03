use actix_web::{web, HttpResponse, Responder, Result};
use askama::Template;
use sqlx::PgPool;

use crate::models::Blogs;
use crate::templates::blogs::{BlogTemplate, IndexTemplate, PostInfoTemplate};



pub async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let posts = match Blogs::find_all_posts_with_authors(&pool).await {
        Ok(posts) => posts,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let template = IndexTemplate { posts };
    let html = template.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}


pub async fn blogs_page(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let posts = match Blogs::find_all_posts_with_authors(&pool).await {
        Ok(posts) => posts,
        Err(e) => {
            eprintln!("Database Error: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
            }
    };
    let template = BlogTemplate{posts};
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html))
}

pub async fn postinfo(
    pool: web::Data<PgPool>,
    slug: web::Path<String>,
) -> impl Responder {
    let slug = slug.into_inner();

    match Blogs::find_post_by_slug(&pool, &slug).await {
        Ok(Some(post)) => {
            let template = PostInfoTemplate { post };
            let html = template.render().unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

