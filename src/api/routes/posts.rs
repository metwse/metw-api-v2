use crate::{AppState, entity, handlers::post_handler as posts};
use axum::{Router, routing::get};
use utoipa::OpenApi;

/// Posts API documentations
#[derive(OpenApi)]
#[openapi(
    paths(posts::get_post_by_id, posts::get_posts_of_thread_id, posts::get_posts),
    components(schemas(entity::Post))
)]
pub struct PostsApiDoc;

/// Posts routes
pub fn post_routes(state: AppState) -> Router {
    Router::new()
        .route("/{id}", get(posts::get_post_by_id))
        .route("/main", get(posts::get_posts))
        .route("/threads/{thread_id}", get(posts::get_posts_of_thread_id))
        .with_state(state)
}
