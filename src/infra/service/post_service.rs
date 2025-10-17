use crate::{dto::posts::PostStatsDto, entity, repository::PostRepository, state::Database};

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

    /// Fetches posts's stats from its ID.
    pub async fn get_post_stats_by_id(&self, id: i64) -> Option<PostStatsDto> {
        self.repo.get_post_stats_by_id(id).await
    }

    /// Gets the latest posts in a thread.
    pub async fn get_latest_posts_of_thread(
        &self,
        thread_id: Option<i64>,
        limit: Option<u64>,
        before: Option<i64>,
    ) -> Vec<entity::Post> {
        self.repo
            .get_latest_posts_of_thread(thread_id, limit, before)
            .await
    }

    /// Gets the hot posts in a thread.
    pub async fn get_hot_posts_of_thread(
        &self,
        thread_id: Option<i64>,
        time_period: Option<u64>,
    ) -> Vec<entity::Post> {
        self.repo
            .get_hot_posts_of_thread(thread_id, time_period)
            .await
    }
}
