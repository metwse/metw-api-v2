use crate::{entity, state::Database};

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
            sqlx::query_as::<_, entity::Post>(
                r#"SELECT id, user_id, thread_id, replies_thread_id, content, is_edited, attachments
                FROM posts
                WHERE id = $1"#
            )
            .bind(id)
        )
    }

    pub async fn get_posts_of_thread_id(
        &self,
        thread_id: Option<i64>,
        limit: Option<u64>,
        before: Option<i64>,
    ) -> Vec<entity::Post> {
        let limit = std::cmp::min(limit.unwrap_or(32), 32) as i64;
        let before = if let Some(before) = before {
            before
        } else {
            i64::MAX
        };

        unwrap_fetch_all!(
            &self.db.pool(),
            if let Some(thread_id) = thread_id {
                sqlx::query_as::<_, entity::Post>(
                    r#"SELECT id, user_id, thread_id, replies_thread_id, content, is_edited, attachments
                    FROM posts
                    WHERE thread_id = $1 AND id < $2
                    ORDER BY id DESC
                    LIMIT $3"#
                )
                .bind(thread_id).bind(before).bind(limit)
            } else {
                sqlx::query_as::<_, entity::Post>(
                    r#"SELECT id, user_id, thread_id, replies_thread_id, content, is_edited, attachments
                    FROM posts
                    WHERE thread_id IS NULL AND id < $1
                    ORDER BY id DESC
                    LIMIT $2"#,
                )
                .bind(before).bind(limit)
            }
        )
    }

    pub async fn get_hot_posts(&self, limit: i64) -> Vec<entity::Post> {
        unwrap_fetch_all!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::Post>(
                r#"WITH
                    latest_replies AS (
                        SELECT COUNT(1) AS reply_count, thread_id
                        FROM posts
                        WHERE id > snowflake_like_base_past('7 days'::interval)
                        GROUP BY thread_id
                    ),
                    latest_likes AS (
                        SELECT COUNT(post_id) AS like_count, post_id
                        FROM relations.likes
                        WHERE id > snowflake_like_base_past('7 days'::interval)
                        GROUP BY post_id
                    )
                SELECT posts.* FROM posts
                LEFT JOIN latest_replies
                    ON latest_replies.thread_id = posts.replies_thread_id
                LEFT JOIN latest_likes
                    ON latest_likes.post_id = posts.id
                WHERE
                    (latest_replies.thread_id IS NOT NULL OR latest_likes.post_id IS NOT NULL) AND
                    id > snowflake_like_base_past('30 days'::interval) AND
                    posts.thread_id IS NULL
                ORDER BY array_length(attachments, 1) + reply_count * 2 + like_count DESC
                LIMIT $1"#
            ).bind(limit)
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

        let posts = repo.get_posts_of_thread_id(None, Some(10), None).await;

        for i in 1..=10i64 {
            assert_eq!(posts[i as usize - 1].id, 4021 - i);
        }

        let posts = repo.get_posts_of_thread_id(None, Some(5), Some(4011)).await;

        for i in 1..=5i64 {
            assert_eq!(posts[i as usize - 1].id, 4011 - i);
        }

        repo.get_hot_posts(128).await;
    }
}
