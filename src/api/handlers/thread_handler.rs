use crate::{
    AppState,
    dto::{PagitationQuery, TimePeriodQuery},
    entity,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};

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
    path = "/{id}/latest",
    responses(
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(PagitationQuery)
)]
pub async fn get_latest_posts_of_thread(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(PagitationQuery { limit, before }): Query<PagitationQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_latest_posts_of_thread(Some(id), limit, before)
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
        (status = OK, description = "Post list", body = Vec<entity::Post>),
    ),
    params(TimePeriodQuery)
)]
pub async fn get_hot_posts_of_thread(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(TimePeriodQuery { time_period }): Query<TimePeriodQuery>,
) -> Json<Vec<entity::Post>> {
    Json(
        state
            .post_service
            .get_hot_posts_of_thread(Some(id), time_period)
            .await,
    )
}
