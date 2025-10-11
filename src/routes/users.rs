use axum::Router;
use utoipa::OpenApi;

/// Users API documentations
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "user_handler", description = "User API")
    )
)]
pub struct UsersApiDoc;

/// Users routes
pub fn users_routes() -> Router {
    Router::new()
}
