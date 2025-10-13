use crate::{entity, response::AppResult};

/// Get post by ID.
///
/// Fethes one post from ID.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "Post object", body = entity::Post)
    )
)]
pub async fn get_post_by_id() -> AppResult<entity::Post> {
    todo!()
}
