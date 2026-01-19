use super::{AttachmentId, UserId};
use sqlx::FromRow;

newtype_id!(Post, Thread);

/// Post
#[derive(Clone, Debug, FromRow)]
pub struct Post {
    /// Unique identifier for the post
    pub id: PostId,
    /// The user that sent this post
    pub user_id: UserId,
    /// The thread that the post belongs to
    pub thread_id: Option<ThreadId>,
    /// Thread id for replying this post
    pub replies_thread_id: ThreadId,
    /// Whether or not the post has been edited
    pub is_edited: bool,
    /// Content
    pub content: String,
    /// List of attachment ids
    pub attachments: Vec<AttachmentId>,
}

/// Thread that posts are sent to
#[derive(Clone, Debug, FromRow)]
pub struct Thread {
    /// Unique identifier for the thread
    pub id: ThreadId,
    /// Id of the user that created this thread
    pub user_id: UserId,
}
