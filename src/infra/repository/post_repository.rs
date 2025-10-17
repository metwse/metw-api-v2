use crate::{dto::posts::PostStatsDto, entity, state::Database};
use indoc::indoc;

/// Post data access repository
pub struct PostRepository {
    db: Database,
}

impl PostRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_post_by_id(&self, id: i64) -> Option<entity::Post> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as(indoc! {
                "SELECT
                    id, user_id, thread_id, replies_thread_id, content, is_edited, attachments
                FROM posts
                WHERE id = $1"
            })
            .bind(id)
        )
    }

    pub async fn get_post_stats_by_id(&self, id: i64) -> Option<PostStatsDto> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as(indoc! {
                "SELECT
                    COALESCE((
                        SELECT COUNT(1) FROM posts AS posts_
                        WHERE posts_.thread_id = posts.replies_thread_id
                        GROUP BY thread_id
                    ), 0) as comments,
                        COALESCE((
                        SELECT COUNT(1) FROM relations.likes
                        WHERE likes.post_id = posts.id
                        GROUP BY post_id
                    ), 0) as likes
                FROM posts WHERE posts.id = $1"
            })
            .bind(id)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::test_db;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn queries() {
        let repo = PostRepository::new(test_db().await);

        for i in 1..=20 {
            repo.get_post_by_id(i + 4000).await.unwrap();
        }
    }
}
