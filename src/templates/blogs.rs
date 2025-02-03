use askama::Template;
use crate::models::blogs::{BlogWithAuthor, HomePageTemp};
use crate::filters;

#[derive(Template)]
#[template(path = "blog/home.html")]
pub struct IndexTemplate {
    pub posts: Vec<HomePageTemp>,
}

#[derive(Template)]
#[template(path = "blog/blogs.html")]
pub struct BlogTemplate {
    pub posts: Vec<HomePageTemp>,
}

#[derive(Template)]
#[template(path = "blog/post.html")]
pub struct PostInfoTemplate {
    pub post: BlogWithAuthor,
}

