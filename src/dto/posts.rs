use crate::entity::{self, AttachmentId, PostId, ThreadId, UserId};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

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
    pub id: PostId,
    /// The user that sent this post
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub user_id: UserId,
    /// The thread that the post belongs to
    #[schema(value_type = Option<String>)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub thread_id: Option<ThreadId>,
    /// Thread id for replying this post
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub replies_thread_id: ThreadId,
    /// Whether or not the post has been edited
    pub is_edited: bool,
    /// Content
    pub content: String,
    /// List of attachment ids
    #[schema(value_type = Vec<String>)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub attachments: Vec<AttachmentId>,
}

/// Stats for user profile
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct PostStatsDto {
    /// Comment count on user's wall
    pub comments: i64,
    /// Like count
    pub likes: i64,
}

impl From<entity::Post> for PostDto {
    fn from(post: entity::Post) -> PostDto {
        PostDto {
            id: post.id,
            user_id: post.user_id,
            thread_id: post.thread_id,
            replies_thread_id: post.replies_thread_id,
            is_edited: post.is_edited,
            content: post.content,
            attachments: post.attachments,
        }
    }
}
