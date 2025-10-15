use crate::{
    dto::posts::PostError,
    entity,
    response::{AppOk, AppResult},
    AppState,
};
use axum::extract::{Path, State};

/// Get post by ID.
///
/// Fethes one post from ID.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "Post object", body = entity::Post)
    ),
)]
pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<entity::Post> {
    if let Some(post) = state.post_service.get_post_by_id(id).await {
        AppOk(post).into()
    } else {
        Err(PostError::PostNotFound.into())
    }
}
