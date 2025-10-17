use crate::{AppState, dto::user::FullProfileDto, entity, handlers::user_handler as users};
use axum::{Router, routing::get};
use utoipa::OpenApi;

/// Users API documentations
#[derive(OpenApi)]
#[openapi(
    paths(
        users::get_user_by_id,
        users::get_user_by_username,
        users::get_profile_by_user_id,
        users::get_profile_by_username
    ),
    components(schemas(entity::User, FullProfileDto))
)]
pub struct UsersApiDoc;

/// Users routes
pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/{id}", get(users::get_user_by_id))
        .route("/@{username}", get(users::get_user_by_username))
        .route("/{id}/profile", get(users::get_profile_by_user_id))
        .route("/@{username}/profile", get(users::get_profile_by_username))
        .with_state(state)
}
