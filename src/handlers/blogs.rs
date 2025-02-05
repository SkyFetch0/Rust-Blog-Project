use actix_web::{web, HttpResponse, Responder, Result};
use askama::Template;
use sqlx::PgPool;
use moka::future::Cache;
use std::sync::Arc;

use crate::models::Blogs;
use crate::templates::blogs::{BlogTemplate, IndexTemplate, PostInfoTemplate};
use crate::state::AppState;

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
    data: web::Data<AppState>,
    slug: web::Path<String>,
) -> impl Responder {
    let slug_str = slug.into_inner();

    if let Some(cached_post) = data.post_cache.get(&slug_str).await {
        let template = PostInfoTemplate { post: cached_post };
        let html = template.render().unwrap();
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html);
    }

    match Blogs::find_post_by_slug(&data.pool, &slug_str).await {
        Ok(Some(post)) => {
            // Gelen post'u cache'e ekleyelim
            data.post_cache.insert(slug_str.clone(), post.clone()).await;
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