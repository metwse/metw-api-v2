/// Post
#[derive(Clone, sqlx::FromRow)]
pub struct Post {
    /// Unique identifier for the post
    pub id: i64,
    /// The [`User`] that sent this post
    ///
    /// [`User`]: super::User
    pub user_id: i64,
    /// The [`Thread`] that the post belongs to
    pub thread_id: Option<i64>,
    /// [`Thread`] id for replying this post
    pub replies_thread_id: i64,
    /// Whether or not the post has been edited
    pub is_edited: bool,
    /// Content
    pub content: String,
    /// List of attachment ids
    pub attachments: Vec<i64>,
}

/// Thread that posts are sent to
#[derive(Clone, sqlx::FromRow)]
pub struct Thread {
    /// Unique identifier for the thread
    pub id: i64,
    /// Id of the [`User`] that created this thread
    ///
    /// [`User`]: super::User
    pub user_id: i64,
}
