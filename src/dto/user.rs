use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
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

/// User's minimal profile including its id and username
#[serde_as]
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct UserDto {
    /// User's ID
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Username
    pub username: String,
    /// Bitset for user flags as hexadecimal number
    #[schema(value_type = String)]
    #[serde(serialize_with = "crate::enc::bitvec_as_hex")]
    pub flags: BitVec,
    /// A thread id for comments on user's wall
    #[schema(value_type = Option<String>)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub avatar_id: Option<i64>,
}

/// User's profile including its id and username
#[serde_as]
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct FullProfileDto {
    /// User's ID
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Username
    pub username: String,
    /// Bitset for user flags as hexadecimal number
    #[schema(value_type = String)]
    #[serde(serialize_with = "crate::enc::bitvec_as_hex")]
    pub flags: BitVec,
    /// A thread id for comments on user's wall
    #[schema(value_type = String)]
    #[serde_as(as = "DisplayFromStr")]
    pub comments_thread_id: i64,
    /// Attachment id for user's avatar
    #[schema(value_type = Option<String>)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub avatar_id: Option<i64>,
    /// Attachment id for user's banner
    #[schema(value_type = Option<String>)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub banner_id: Option<i64>,
    /// Biography
    pub bio: String,
}

/// Stats for user profile
#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct UserStatsDto {
    /// Comment count on user's wall
    pub comments: i64,
    /// Follower count
    pub followers: i64,
    /// Following count count
    pub follows: i64,
}
