use serde::Deserialize;
use utoipa::IntoParams;

#[macro_use]
mod macros;

/// Auth DTOs
pub mod auth;

/// Post DTOs
pub mod posts;

/// User DTOs
pub mod user;

/// Limit and before query params.
#[derive(Deserialize, IntoParams)]
pub struct PagitationQuery {
    /// Total element count
    pub limit: Option<u64>,
    /// Elements before that id
    pub before: Option<i64>,
}
