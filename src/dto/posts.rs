use serde::Serialize;
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

/// Stats for user profile
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct PostStatsDto {
    /// Comment count on user's wall
    pub comments: i64,
    /// Like count
    pub likes: i64,
}
