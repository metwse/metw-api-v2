use crate::{
    dto::{
        posts::{error_examples, PostError},
        PagitationQuery, TimePeriodQuery,
    },
    entity,
    response::{AppOk, AppResult},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
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

/// Gets latest posts on the main thread
///
/// List of latest posts in main thread.
#[utoipa::path(
    get,
    path = "/latest",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(PagitationQuery)
)]
pub async fn get_latest_posts(
    State(state): State<AppState>,
    Query(PagitationQuery { limit, before }): Query<PagitationQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_latest_posts_of_thread(None, limit, before)
            .await,
    )
}

/// Gets hot posts on the main thread
///
/// List of hot posts in main thread.
#[utoipa::path(
    get,
    path = "/hot",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(TimePeriodQuery)
)]
pub async fn get_hot_posts(
    State(state): State<AppState>,
    Query(TimePeriodQuery { time_period }): Query<TimePeriodQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_hot_posts_of_thread(None, time_period)
            .await,
    )
}

/// Gets the posts in a thread
///
/// List of posts in a thread.
#[utoipa::path(
    get,
    path = "/threads/{thread_id}/latest",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(PagitationQuery)
)]
pub async fn get_latest_posts_of_thread(
    State(state): State<AppState>,
    Path(thread_id): Path<i64>,
    Query(PagitationQuery { limit, before }): Query<PagitationQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_latest_posts_of_thread(Some(thread_id), limit, before)
            .await,
    )
}

/// Gets hot posts in a thread
///
/// List of posts in a thread.
#[utoipa::path(
    get,
    path = "/threads/{thread_id}/hot",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(TimePeriodQuery)
)]
pub async fn get_hot_posts_of_thread(
    State(state): State<AppState>,
    Path(thread_id): Path<i64>,
    Query(TimePeriodQuery { time_period }): Query<TimePeriodQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_hot_posts_of_thread(Some(thread_id), time_period)
            .await,
    )
}
