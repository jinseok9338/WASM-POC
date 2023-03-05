#![allow(unused_imports)]

use leptos::{ServerFnError, server};
use lazy_static::lazy_static;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use leptos::ServerFn;

// Dummy API
lazy_static! {
    static ref POSTS: Vec<Post> = vec![
        Post {
            id: 0,
            title: "My first post".to_string(),
            content: "This is my first post".to_string(),
        },
        Post {
            id: 1,
            title: "My second post".to_string(),
            content: "This is my second post".to_string(),
        },
        Post {
            id: 2,
            title: "My third post".to_string(),
            content: "This is my third post".to_string(),
        },
    ];
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMetadata {
    pub id: usize,
    pub title: String,
}

#[server(ListPostMetadata, "/api")]
pub async fn list_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {

    Ok(POSTS
        .iter()
        .map(|data| PostMetadata {
            id: data.id,
            title: data.title.clone(),
        })
        .collect())
}

#[server(GetPost, "/api")]
pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {

    Ok(POSTS.iter().find(|post| post.id == id).cloned())
}