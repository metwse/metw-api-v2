use crate::{
    dto::{posts::PostDto, PagitationQuery, TimePeriodQuery},
    entity::{PostId, ThreadId},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};

/// Gets latest posts on the main thread
///
/// List of latest posts in main thread.
#[utoipa::path(
    get,
    path = "/latest",
    responses(
        (status = OK, description = "Post list", body = Vec<PostDto>),
    ),
    params(PagitationQuery)
)]
pub async fn get_latest_posts(
    State(state): State<AppState>,
    Query(PagitationQuery { limit, before }): Query<PagitationQuery>,
) -> Json<Vec<PostDto>> {
    Json(
        state
            .post_service
            .get_latest_posts_of_thread(None, limit, before.map(PostId))
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
        (status = OK, description = "Post list", body = Vec<PostDto>),
    ),
    params(TimePeriodQuery)
)]
pub async fn get_hot_posts(
    State(state): State<AppState>,
    Query(TimePeriodQuery { time_period }): Query<TimePeriodQuery>,
) -> Json<Vec<PostDto>> {
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
    path = "/{id}/latest",
    responses(
        (status = OK, description = "Post list", body = Vec<PostDto>),
    ),
    params(PagitationQuery)
)]
pub async fn get_latest_posts_of_thread(
    State(state): State<AppState>,
    Path(id): Path<ThreadId>,
    Query(PagitationQuery { limit, before }): Query<PagitationQuery>,
) -> Json<Vec<PostDto>> {
    Json(
        state
            .post_service
            .get_latest_posts_of_thread(Some(id), limit, before.map(PostId))
            .await,
    )
}

/// Gets hot posts in a thread
///
/// List of posts in a thread.
#[utoipa::path(
    get,
    path = "/{id}/hot",
    responses(
        (status = OK, description = "Post list", body = Vec<PostDto>),
    ),
    params(TimePeriodQuery)
)]
pub async fn get_hot_posts_of_thread(
    State(state): State<AppState>,
    Path(id): Path<ThreadId>,
    Query(TimePeriodQuery { time_period }): Query<TimePeriodQuery>,
) -> Json<Vec<PostDto>> {
    Json(
        state
            .post_service
            .get_hot_posts_of_thread(Some(id), time_period)
            .await,
    )
}
