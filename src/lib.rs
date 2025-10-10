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

/// Utility functions
pub mod util;

/// Test helpers
#[cfg(test)]
pub mod testutil;
