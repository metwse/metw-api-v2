//! metw.cc API Version 2

#![forbid(unsafe_code, unused_must_use)]
#![warn(clippy::all, missing_docs)]
// Enable documentation for all features on docs.rs.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod snowflake;

pub use snowflake::snowflake;

/// Database entities
pub mod entity;

/// Global shared state
pub mod state;

/// API application
pub mod app;

/// Data transfer objects
pub mod dto;

/// Utility functions
pub mod util;

/// API response
pub mod response;

/// Service and repository implementations
pub mod infra;

/// Routes and their handlers
pub mod api;

/// Test helpers
#[cfg(test)]
pub mod testutil;

pub use app::create_router;
pub use state::{AppState, Config};

pub use api::{handlers, routes};
pub use infra::{repository, service};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

lazy_static::lazy_static! {
    /// Year zero of metw.cc.
    ///
    /// 2022 Aug 12, 00:00:00 (`1660262400000`)
    pub static ref EPOCH: u64 = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 8, 12).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    ).and_utc().timestamp_millis() as u64;
}
