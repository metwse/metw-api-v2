use crate::{entity, repository::PostRepository, state::Database};

/// Service struct for handling post-related operations.
pub struct PostService {
    repo: PostRepository,
}

impl PostService {
    /// Creates a new repository instance.
    pub fn new(db: Database) -> Self {
        Self {
            repo: PostRepository::new(db),
        }
    }

    /// Finds a post from its ID.
    pub async fn get_post_by_id(&self, id: i64) -> Option<entity::Post> {
        self.repo.get_post_by_id(id).await
    }

    /// Gets posts in a thread.
    pub async fn get_posts_of_thread_id(
        &self,
        thread_id: Option<i64>,
        limit: u64,
        before: Option<i64>,
    ) -> Vec<entity::Post> {
        self.repo
            .get_posts_of_thread_id(thread_id, limit, before)
            .await
    }
}
