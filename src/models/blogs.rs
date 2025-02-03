use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Blogs {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub author_id: i32,
    pub status: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub view_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct BlogWithAuthor {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub author_id: i32,
    pub status: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub view_count: Option<i32>,

    pub author_name: String,
    pub author_image: Option<String>,
    pub author_description: Option<String>,
    #[sqlx(skip)]
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct HomePageTemp {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub author_id: i32,
    pub status: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub view_count: Option<i32>,

    pub author_name: String,
    pub author_image: Option<String>,

}

#[derive(Debug, Serialize)]
pub struct BlogsPageTemp {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub view_count: Option<i32>,
    pub author_id: i32,
    pub author_name: String,
    pub author_image: Option<String>,
    pub category: String,
}

impl Blogs {



    pub async fn find_all_posts_with_authors(pool: &PgPool) -> Result<Vec<HomePageTemp>, sqlx::Error> {
        sqlx::query_as::<_, HomePageTemp>(
            r#"
            SELECT
                posts.*,
                users.username as author_name,
                users.profile_image as author_image

            FROM posts
            LEFT JOIN users ON posts.author_id = users.id
            ORDER BY posts.updated_at DESC NULLS LAST
            "#
        )
            .fetch_all(pool)
            .await
    }

    pub async fn find_post_by_slug(pool: &PgPool, slug: &str) -> Result<Option<BlogWithAuthor>, sqlx::Error> {
        let mut post = sqlx::query_as::<_, BlogWithAuthor>(
            r#"
        SELECT
            posts.*,
            users.username as author_name,
            users.profile_image as author_image,
            users.bio as author_description

        FROM posts
        LEFT JOIN users ON posts.author_id = users.id
        WHERE posts.slug = $1
        LIMIT 1
        "#
        )
            .bind(slug)
            .fetch_optional(pool)
            .await?;

        if let Some(ref mut post) = post {
            let categories = sqlx::query_as::<_, Category>(
                r#"
            SELECT
                categories.id,
                categories.name,
                categories.slug,
                categories.description
            FROM categories
            INNER JOIN categories_relationships ON categories.id = categories_relationships.term_id
            WHERE categories_relationships.post_id = $1
            "#
            )
                .bind(post.id)
                .fetch_all(pool)
                .await?;

            post.categories = categories;
        }

        Ok(post)
    }




}