use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use sqlx::PgPool;
use serde_json::json;

use crate::models::api::Api;
#[derive(Deserialize)]
pub struct CommentData {
    post_id: i32,
    username: String,
    content: String,
}

// cache will be added in future versions


pub async fn comments(
    pool: web::Data<PgPool>,
    post_id: web::Path<i32>
) -> Result<HttpResponse, actix_web::Error> {
    let posts = match Api::find_comments(&pool, &post_id).await {
        Ok(posts) => posts,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    if posts.is_empty() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": false,
            "code": 2,
            "message": "No comments found",
            "data": []
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "code": 1,
        "message": "Comments found successfully",
        "data": posts
    })))
}
pub async fn addcomment(
    pool: web::Data<PgPool>,
    comment_data: web::Json<CommentData>,
) -> Result<HttpResponse, actix_web::Error> {
    let post = match Api::add_comment(
        &pool,
        &comment_data.post_id,
        &comment_data.username,
        &comment_data.content
    ).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": false,
                "code": 0,
                "message": "Internal server error"
            })));
        }
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "code": 1,
        "message": "Comment added successfully",
        "data": post
    })))
}
