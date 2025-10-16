use crate::{
    AppState,
    dto::{
        PagitationQuery,
        posts::{PostError, error_examples},
    },
    entity,
    response::{AppOk, AppResult},
};
use axum::{
    Json,
    extract::{Path, Query, State},
};

/// Gets a post by ID.
///
/// Fethes one post from its ID.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "Post object", body = entity::Post),
        error_examples::PostNotFoundDto
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

/// Gets a posts on main thread
///
/// List of latest posts in main thread.
#[utoipa::path(
    get,
    path = "/main",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(PagitationQuery)
)]
pub async fn get_posts(
    State(state): State<AppState>,
    Query(pagitation): Query<PagitationQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_posts_of_thread_id(None, pagitation.limit, pagitation.before)
            .await,
    )
}

/// Gets a posts in a thread
///
/// List of posts in a thread.
#[utoipa::path(
    get,
    path = "/threads/{thread_id}",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(PagitationQuery)
)]
pub async fn get_posts_of_thread_id(
    State(state): State<AppState>,
    Path(thread_id): Path<i64>,
    Query(pagitation): Query<PagitationQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_posts_of_thread_id(Some(thread_id), pagitation.limit, pagitation.before)
            .await,
    )
}
