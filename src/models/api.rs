use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;
use html_escape::encode_text;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Api {
    pub id: Option<i32>,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Api {
    pub async fn find_comments(pool: &PgPool, post_id: &i32) -> Result<Vec<Api>, sqlx::Error> {
        let comments = sqlx::query_as::<_, Api>(
            r#"
            SELECT * FROM comments WHERE post_id = $1 ORDER BY created_at DESC
            "#
        )
            .bind(post_id)
            .fetch_all(pool)
            .await?;

        Ok(comments)
    }

    pub async fn add_comment(
        pool: &PgPool,
        post_id: &i32,
        username: &String,
        content: &String
    ) -> Result<Api, sqlx::Error> {
        let clean_content = encode_text(content);
        let clean_username = encode_text(username);

        let comment = sqlx::query_as::<_, Api>(
            r#"
        INSERT INTO comments (post_id, user_id, author_name, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#
        )
            .bind(post_id)
            .bind(1)
            .bind(clean_username)
            .bind(clean_content)
            .fetch_one(pool)
            .await?;

        Ok(comment)
    }
}