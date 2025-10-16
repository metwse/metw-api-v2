use serde::Serialize;
use sqlx::{prelude::FromRow, types::BitVec};
use utoipa::ToSchema;

api_errors!(
    UserError,
    responses(
        UserNotFound = (
            status = NOT_FOUND,
            description = "Could not find the user.",
            variants = (UserNotFound = "User not found.")
        ),
    )
);

/// User's profile including its id and username
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct FullProfileDto {
    /// User's ID
    pub id: i64,
    /// Username
    pub username: String,
    /// Bitset for administrative user flags
    #[serde(serialize_with = "crate::util::serialize_bitvec_as_bytes")]
    #[schema(value_type = Vec<u8>)]
    pub flags: BitVec,
    /// A thread id for comments on user's wall
    pub comments_thread_id: i64,
    /// Attachment id for user's avatar
    pub avatar_id: Option<i64>,
    /// Attachment id for user's banner
    pub banner_id: Option<i64>,
    /// Biography
    pub bio: String,
}
