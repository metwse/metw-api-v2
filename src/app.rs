use crate::{AppState, response::AppErrorDto, routes};
use axum::{Json, Router, routing::get};
use lazy_static::lazy_static;
use serde::Serialize;
use std::time::Instant;
use utoipa::{OpenApi, ToSchema};

/// Application status
#[derive(Serialize, ToSchema)]
struct ApiStatus {
    /// Message to clients
    pub message: String,
    /// Path of a patch script that should be executed by clients if present
    pub bootstrap_patch: Option<String>,
    /// Uptime of the API server, in seconds
    pub uptime: u64,
}

#[derive(OpenApi)]
#[openapi(
    paths(status),
    components(schemas(ApiStatus, AppErrorDto)),
    tags(
        (name = "default", description = "Miscellaneous uncategorized API endpoints"),
        (name = "auth", description = "User authentication endpoints"),
        (name = "posts", description = "Post/thread API"),
        (name = "users", description = "User API"),
    ),
    nest(
        (path = "/auth", api = routes::AuthApiDoc),
        (path = "/posts", api = routes::PostsApiDoc),
        (path = "/users", api = routes::UsersApiDoc),
    ),
    servers(
        (url = "http://localhost:1186", description = "Default development server")
    ),
)]
struct ApiDoc;

lazy_static! {
    static ref STARTUP_TIME: Instant = Instant::now();
}

/// Status of the application.
///
/// JavaScript file at `bootstrap_patch` used for front-end error recovery.
/// It runs before any front-end initialization and is useful when an
/// unrecoverable update prevents the front end from loading correctly.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, description = "Application status", body = ApiStatus)
    ),
    tag = "default"
)]
async fn status() -> Json<ApiStatus> {
    Json(ApiStatus {
        message: String::from("OK"),
        bootstrap_patch: None,
        uptime: Instant::now().duration_since(*STARTUP_TIME).as_secs(),
    })
}

/// Return JSON version of an OpenAPI schema
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// API router
pub async fn create_router(state: AppState) -> Router {
    let _ = *STARTUP_TIME;

    Router::new()
        .route("/", get(status))
        .route("/openapi.json", get(openapi))
        .with_state(state.clone())
        .nest("/auth", routes::auth_routes(state.clone()))
        .nest("/users", routes::user_routes(state.clone()))
        .nest("/posts", routes::post_routes(state.clone()))
}
