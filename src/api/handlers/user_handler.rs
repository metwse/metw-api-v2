use crate::{
    AppState,
    dto::user::{FullProfileDto, UserError, UserStatsDto, error_examples},
    entity,
    response::{AppOk, AppResult},
};
use axum::extract::{Path, State};

/// Gets an user by ID.
///
/// Fetches one user from its ID.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "User object", body = entity::User),
        error_examples::UserNotFoundDto
    ),
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<entity::User> {
    if let Some(user) = state.user_service.get_user_by_id(id).await {
        AppOk(user).into()
    } else {
        Err(UserError::UserNotFound.into())
    }
}

/// Gets an user by username.
///
/// Fetches one user from its username.
#[utoipa::path(
    get,
    path = "/@{username}",
    responses(
        (status = OK, description = "User object", body = entity::User),
        error_examples::UserNotFoundDto
    ),
)]
pub async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> AppResult<entity::User> {
    if username.len() > 20 || username.is_empty() {
        return Err(UserError::UserNotFound.into());
    }

    if let Some(user) = state.user_service.get_user_by_username(&username).await {
        AppOk(user).into()
    } else {
        Err(UserError::UserNotFound.into())
    }
}

/// Gets an user profile by user ID.
///
/// Fetches one user's profile from user ID.
#[utoipa::path(
    get,
    path = "/{id}/profile",
    responses(
        (status = OK, description = "User profile object", body = FullProfileDto),
        error_examples::UserNotFoundDto
    ),
)]
pub async fn get_profile_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<FullProfileDto> {
    if let Some(user) = state.user_service.get_profile_by_id(id).await {
        AppOk(user).into()
    } else {
        Err(UserError::UserNotFound.into())
    }
}

/// Gets an user profile by username.
///
/// Fetches one user's profile from username.
#[utoipa::path(
    get,
    path = "/@{username}/profile",
    responses(
        (status = OK, description = "User profile object", body = FullProfileDto),
        error_examples::UserNotFoundDto
    ),
)]
pub async fn get_profile_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> AppResult<FullProfileDto> {
    if username.len() > 20 || username.is_empty() {
        return Err(UserError::UserNotFound.into());
    }

    if let Some(user) = state.user_service.get_profile_by_username(&username).await {
        AppOk(user).into()
    } else {
        Err(UserError::UserNotFound.into())
    }
}

/// Gets an user's stats by user ID.
///
/// Fetches one user's stats by user ID.
#[utoipa::path(
    get,
    path = "/{id}/stats",
    responses(
        (status = OK, description = "User stats object", body = UserStatsDto),
        error_examples::UserNotFoundDto
    ),
)]
pub async fn get_user_stats_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<UserStatsDto> {
    if let Some(user_stats) = state.user_service.get_user_stats_by_id(id).await {
        AppOk(user_stats).into()
    } else {
        Err(UserError::UserNotFound.into())
    }
}
