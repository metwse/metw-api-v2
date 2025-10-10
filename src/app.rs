use crate::state::AppState;
use axum::{Json, Router, routing::get};
use lazy_static::lazy_static;
use serde::Serialize;
use std::time::Instant;
use utoipa::{OpenApi, ToSchema};

/// Application status
#[derive(Serialize, ToSchema)]
struct ApiStatus {
    /// Message to clients.
    pub message: String,
    /// Recovery script that should be executed by clients if present.
    pub js: Option<String>,
    /// Uptime of the API server, in seconds.
    pub uptime: u64,
}

#[derive(OpenApi, ToSchema)]
#[openapi(paths(openapi, status), components(schemas(ApiStatus, ApiDoc)))]
struct ApiDoc;

lazy_static! {
    static ref STARTUP_TIME: Instant = Instant::now();
}

/// Status of the application.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Application status", body = ApiStatus)
    )
)]
async fn status() -> Json<ApiStatus> {
    Json(ApiStatus {
        message: String::from("OK"),
        js: None,
        uptime: Instant::now().duration_since(*STARTUP_TIME).as_secs(),
    })
}

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "OpenApi JSON", body = ApiDoc)
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// API router
pub async fn create_router(state: AppState) -> Router {
    let _ = *STARTUP_TIME;

    Router::new()
        .route("/", get(status))
        .route("/api-docs/openapi.json", get(openapi))
        .with_state(state)
}
