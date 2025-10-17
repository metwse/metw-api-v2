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
    /// Limit of element count
    pub limit: Option<u64>,
    /// Elements before that id
    pub before: Option<i64>,
}

/// Time period query params.
#[derive(Deserialize, IntoParams)]
pub struct TimePeriodQuery {
    /// Time period, in days
    pub time_period: Option<u64>,
}
