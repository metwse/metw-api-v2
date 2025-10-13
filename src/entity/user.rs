use sqlx::{prelude::FromRow, types::BitVec};

/// User
#[derive(Clone, Debug, FromRow)]
pub struct User {
    /// Unique identifier for the user
    pub id: i64,
    /// Username
    pub username: String,
    /// Argon2-hashed password
    pub password: String,
    /// Bitset for administrative user flags
    pub flags: BitVec,
}

/// Email addresses used for the authentication
#[derive(Clone, Debug, FromRow)]
pub struct Email {
    /// Unique identifier for email
    pub id: i64,
    /// Id of the [`User`] that email belongs to
    pub user_id: i64,
    /// Email
    pub email: String,
    /// Whether or not the email has been verified
    pub is_verified: bool,
}

/// User's public profile
#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Profile {
    /// Id of the user that profile belongs to
    pub user_id: i64,
    /// A thread id for comments on user's wall
    pub comments_thread_id: i64,
    /// Attachment id for user's avatar
    pub avatar_id: Option<i64>,
    /// Attachment id for user's banner
    pub banner_id: Option<i64>,
    /// Biography
    pub bio: String,
}
