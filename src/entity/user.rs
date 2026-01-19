use super::{AttachmentId, ThreadId};
use sqlx::{types::BitVec, FromRow};

newtype_id!(User, Email);

/// User
#[derive(Clone, Debug, FromRow)]
pub struct User {
    /// Unique identifier for the user
    pub id: UserId,
    /// Username
    pub username: String,
    /// Bitset for administrative user flags
    pub flags: BitVec,
}

/// Email addresses used for the authentication
#[derive(Clone, Debug, FromRow)]
pub struct Email {
    /// Unique identifier for email
    pub id: EmailId,
    /// Id of the [`User`] that email belongs to
    pub user_id: UserId,
    /// Email
    pub email: String,
    /// Whether or not the email has been verified
    pub is_verified: bool,
}

/// User's public profile
#[derive(Clone, Debug, FromRow)]
pub struct Profile {
    /// Id of the user that profile belongs to
    pub user_id: UserId,
    /// A thread id for comments on user's wall
    pub comments_thread_id: ThreadId,
    /// Attachment id for user's avatar
    pub avatar_id: Option<AttachmentId>,
    /// Attachment id for user's banner
    pub banner_id: Option<AttachmentId>,
    /// Biography
    pub bio: String,
}
