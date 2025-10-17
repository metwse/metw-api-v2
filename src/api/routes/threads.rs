use crate::{AppState, entity, handlers::thread_handler as threads};
use axum::{Router, routing::get};
use utoipa::OpenApi;

/// Posts API documentations
#[derive(OpenApi)]
#[openapi(
    paths(
        threads::get_latest_posts_of_thread,
        threads::get_hot_posts_of_thread,
        threads::get_latest_posts,
        threads::get_hot_posts,
    ),
    components(schemas(entity::Post))
)]
pub struct ThreadsApiDoc;

/// Posts routes
pub fn thread_routes(state: AppState) -> Router {
    Router::new()
        .route("/latest", get(threads::get_latest_posts))
        .route("/hot", get(threads::get_hot_posts))
        .route("/{id}/latest", get(threads::get_latest_posts_of_thread))
        .route("/{id}/hot", get(threads::get_hot_posts_of_thread))
        .with_state(state)
}
