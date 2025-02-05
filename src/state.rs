use sqlx::PgPool;
use moka::future::Cache;
use std::sync::Arc;
use crate::models::blogs::BlogWithAuthor;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub post_cache: Arc<Cache<String, BlogWithAuthor>>,
}
