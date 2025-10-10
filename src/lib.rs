//! metw.cc API Version 2

#![forbid(unsafe_code, unused_must_use)]
#![warn(clippy::all, missing_docs)]
// Enable documentation for all features on docs.rs.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Database entities
pub mod entity;

/// Database manipulation service
pub mod service;

/// Data access repository
pub mod repository;

/// Global shared state
pub mod state;

/// API application
pub mod app;

/// Data transfer objects
pub mod dto;

/// Request handlers
pub mod handlers;

/// API routes
pub mod routes;

/// Utility functions
pub mod util;

/// API response
pub mod response;

/// Test helpers
#[cfg(test)]
pub mod testutil;

pub use app::create_router;
pub use state::{AppState, Config};
