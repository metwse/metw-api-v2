use crate::{
    AppState,
    dto::gateway::{AuthUserDto, GatewayError, TokenDto},
    handlers::gateway_handler as gateway,
};
use axum::{Router, routing::post};
use utoipa::OpenApi;

/// Gateway API documentations
#[derive(OpenApi)]
#[openapi(
    paths(gateway::create_user, gateway::login),
    components(schemas(AuthUserDto, TokenDto, GatewayError))
)]
pub struct GatewayApiDoc;

/// Gateway routes
pub fn gateway_routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(gateway::create_user))
        .route("/login", post(gateway::login))
        .with_state(state)
}
