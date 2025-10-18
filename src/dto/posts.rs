use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::entity;

api_errors!(
    PostError,
    responses(
        PostNotFound = (
            status = NOT_FOUND,
            description = "Could not find the post.",
            variants = (PostNotFound = "Post not found.")
        ),
    )
);

/// Post data transfer object
#[serde_as]
#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct PostDto {
    /// Unique identifier for the post
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// The user that sent this post
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub user_id: i64,
    /// The thread that the post belongs to
    #[schema(value_type = Option<String>)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub thread_id: Option<i64>,
    /// Thread id for replying this post
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub replies_thread_id: i64,
    /// Whether or not the post has been edited
    pub is_edited: bool,
    /// Content
    pub content: String,
    /// List of attachment ids
    #[schema(value_type = Vec<String>)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub attachments: Vec<i64>,
}

/// Stats for user profile
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct PostStatsDto {
    /// Comment count on user's wall
    pub comments: i64,
    /// Like count
    pub likes: i64,
}

impl Into<PostDto> for entity::Post {
    fn into(self) -> PostDto {
        PostDto {
            id: self.id,
            user_id: self.user_id,
            thread_id: self.thread_id,
            replies_thread_id: self.replies_thread_id,
            is_edited: self.is_edited,
            content: self.content,
            attachments: self.attachments,
        }
    }
}
