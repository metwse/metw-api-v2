use crate::{
    AppState,
    dto::posts::{PostDto, PostError, PostStatsDto, error_examples},
    response::{AppOk, AppResult},
};
use axum::extract::{Path, State};

/// Gets a post by ID.
///
/// Fethes one post from its ID.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "Post object", body = PostDto),
        error_examples::PostNotFoundDto
    ),
)]
pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<PostDto> {
    if let Some(post) = state.post_service.get_post_by_id(id).await {
        AppOk(post).into()
    } else {
        Err(PostError::PostNotFound.into())
    }
}

/// Gets a post's stats by ID.
///
/// Fethes stats of a post from its ID.
#[utoipa::path(
    get,
    path = "/{id}/stats",
    responses(
        (status = OK, description = "Post stats object", body = PostStatsDto),
        error_examples::PostNotFoundDto
    ),
)]

pub async fn get_post_stats_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<PostStatsDto> {
    if let Some(post) = state.post_service.get_post_stats_by_id(id).await {
        AppOk(post).into()
    } else {
        Err(PostError::PostNotFound.into())
    }
}
