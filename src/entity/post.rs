use sqlx::prelude::FromRow;

/// Post
#[derive(Clone, Debug, FromRow)]
pub struct Post {
    /// Unique identifier for the post
    pub id: i64,
    /// The user that sent this post
    pub user_id: i64,
    /// The thread that the post belongs to
    pub thread_id: Option<i64>,
    /// Thread id for replying this post
    pub replies_thread_id: i64,
    /// Whether or not the post has been edited
    pub is_edited: bool,
    /// Content
    pub content: String,
    /// List of attachment ids
    pub attachments: Vec<i64>,
}

/// Thread that posts are sent to
#[derive(Clone, Debug, FromRow)]
pub struct Thread {
    /// Unique identifier for the thread
    pub id: i64,
    /// Id of the user that created this thread
    pub user_id: i64,
}
