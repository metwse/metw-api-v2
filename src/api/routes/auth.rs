use crate::{
    AppState,
    dto::auth::{AuthError, AuthUserDto, TokenDto},
    handlers::auth_handler as auth,
};
use axum::{Router, routing::post};
use utoipa::OpenApi;

/// Authentication API documentations
#[derive(OpenApi)]
#[openapi(
    paths(auth::register, auth::login),
    components(schemas(AuthUserDto, TokenDto, AuthError))
)]
pub struct AuthApiDoc;

/// Authentication routes
pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .with_state(state)
}
