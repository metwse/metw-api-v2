use crate::{dto::user::{FullProfileDto, UserDto}, handlers::user_handler as users, AppState};
use axum::{Router, routing::get};
use utoipa::OpenApi;

/// Users API documentations
#[derive(OpenApi)]
#[openapi(
    paths(
        users::get_user_by_id,
        users::get_user_by_username,
        users::get_profile_by_id,
        users::get_profile_by_username,
        users::get_user_stats_by_id,
        users::get_follows,
        users::get_followers,
    ),
    components(schemas(UserDto, FullProfileDto))
)]
pub struct UsersApiDoc;

/// Users routes
pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/{id}", get(users::get_user_by_id))
        .route("/@{username}", get(users::get_user_by_username))
        .route("/{id}/profile", get(users::get_profile_by_id))
        .route("/@{username}/profile", get(users::get_profile_by_username))
        .route("/{id}/stats", get(users::get_user_stats_by_id))
        .route("/{id}/follows", get(users::get_follows))
        .route("/{id}/followers", get(users::get_followers))
        .with_state(state)
}
