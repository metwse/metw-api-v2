use crate::{
    dto::gateway::{AuthUserDto, TokenDto},
    handlers::gateway_handler,
};
use axum::{Router, routing::post};
use utoipa::OpenApi;

/// Gateway API documentations
#[derive(OpenApi)]
#[openapi(
    paths(gateway_handler::create_user, gateway_handler::login),
    components(schemas(AuthUserDto, TokenDto)),
    tags(
        (name = "Gateway", description = "User authentication endpoints")
    )
)]
pub struct GatewayApiDoc;

/// Gateway routes
pub fn gateway_routes() -> Router {
    Router::new()
        .route("/register", post(gateway_handler::create_user))
        .route("/login", post(gateway_handler::login))
}
