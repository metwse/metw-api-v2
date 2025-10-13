use crate::AppState;
use axum::Router;
use utoipa::OpenApi;

/// Users API documentations
#[derive(OpenApi)]
#[openapi()]
pub struct UsersApiDoc;

/// Users routes
pub fn user_routes(state: AppState) -> Router {
    Router::new().with_state(state)
}
