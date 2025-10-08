//! metw.cc API Version 2

#![forbid(unsafe_code, unused_must_use)]
#![warn(clippy::all, missing_docs)]
// Enable documentation for all features on docs.rs.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;


/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "Status", body = String)
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// API router
pub fn api() -> Router {
    Router::new()
        .route("/", get(async || Json("ok")))
        .route("/api-docs/openapi.json", get(openapi))
}
