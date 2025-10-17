use crate::{entity, handlers::post_handler as posts, AppState};
use axum::{routing::get, Router};
use utoipa::OpenApi;

/// Posts API documentations
#[derive(OpenApi)]
#[openapi(
    paths(
        posts::get_post_by_id,
        posts::get_latest_posts,
        posts::get_latest_posts_of_thread,
        posts::get_hot_posts,
        posts::get_hot_posts_of_thread,
    ),
    components(schemas(entity::Post))
)]
pub struct PostsApiDoc;

/// Posts routes
pub fn post_routes(state: AppState) -> Router {
    Router::new()
        .route("/{id}", get(posts::get_post_by_id))
        .route("/hot", get(posts::get_hot_posts))
        .route("/latest", get(posts::get_latest_posts))
        .route(
            "/threads/{thread_id}/latest",
            get(posts::get_latest_posts_of_thread),
        )
        .route(
            "/threads/{thread_id}/hot",
            get(posts::get_hot_posts_of_thread),
        )
        .with_state(state)
}
