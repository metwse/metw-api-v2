use axum::{routing::get, Router};
use utoipa::OpenApi;
use crate::{entity, handlers::post_handler};

/// Posts API documentations
#[derive(OpenApi)]
#[openapi(
    paths(post_handler::get_post_by_id),
    components(schemas(entity::Post)),
    tags(
        (name = "post_handler", description = "Post/thread API")
    )
)]
pub struct PostsApiDoc;

/// Posts routes
pub fn posts_routes() -> Router {
    Router::new()
        .route("/", get(post_handler::get_post_by_id))
}
