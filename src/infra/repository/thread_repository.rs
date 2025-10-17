use crate::{entity, state::Database};
use indoc::indoc;
use sqlx::PgTransaction;

/// Thread data access repository
pub struct ThreadRepository {
    db: Database,
}

impl ThreadRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_thread_by_id(&self, id: i64) -> Option<entity::Thread> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::Thread>(
                r#"SELECT id, user_id
                FROM threads
                WHERE id = $1"#
            )
            .bind(id)
        )
    }

    pub async fn create_thread(
        &self,
        tx: &mut PgTransaction<'_>,
        thread: entity::Thread,
    ) -> Option<()> {
        unwrap_execute!(
            &mut **tx,
            sqlx::query(
                r#"INSERT INTO threads (id, user_id)
                VALUES
                    ($1, $2)"#,
            )
            .bind(thread.id)
            .bind(thread.user_id)
        )?;

        Some(())
    }

    pub async fn get_latest_posts(
        &self,
        thread_id: Option<i64>,
        limit: Option<u64>,
        before: Option<i64>,
    ) -> Vec<entity::Post> {
        let limit = std::cmp::min(limit.unwrap_or(32), 32) as i64;
        let before = before.unwrap_or(i64::MAX);

        let sql = format!(
            indoc! {
                "SELECT
                    id, user_id, thread_id, replies_thread_id, content, is_edited, attachments
                FROM posts
                WHERE thread_id {}
                AND id < $1
                ORDER BY id DESC
                LIMIT $2"
            },
            if thread_id.is_some() {
                "= $3"
            } else {
                "IS NULL"
            }
        );

        let mut query = sqlx::query_as(&sql).bind(before).bind(limit);

        if let Some(thread_id) = thread_id {
            query = query.bind(thread_id);
        }

        unwrap_fetch_all!(&self.db.pool(), query)
    }

    pub async fn get_hot_posts(
        &self,
        thread_id: Option<i64>,
        time_period: Option<u64>,
    ) -> Vec<entity::Post> {
        let time_period = time_period.unwrap_or(7).clamp(1, 30);

        let sql = format!(
            indoc! {
                "WITH
                    latest_replies AS (
                        SELECT COUNT(1) AS reply_count, thread_id
                        FROM posts
                        WHERE id > snowflake_like_base_past($1::interval)
                        GROUP BY thread_id
                    ),
                    latest_likes AS (
                        SELECT COUNT(post_id) AS like_count, post_id
                        FROM relations.likes
                        WHERE id > snowflake_like_base_past($1::interval)
                        GROUP BY post_id
                    )
                SELECT posts.* FROM posts
                LEFT JOIN latest_replies
                    ON latest_replies.thread_id = posts.replies_thread_id
                LEFT JOIN latest_likes
                    ON latest_likes.post_id = posts.id
                WHERE
                    posts.id > snowflake_like_base_past($1::interval) AND
                    (
                        latest_replies.thread_id IS NOT NULL OR
                        latest_likes.post_id IS NOT NULL
                    ) AND
                    posts.thread_id {}
                ORDER BY array_length(attachments, 1) + reply_count * 2 + like_count DESC
                LIMIT 32"
            },
            if thread_id.is_some() {
                "= $2"
            } else {
                "IS NULL"
            }
        );

        let mut query = sqlx::query_as(&sql).bind(format!("{} days", time_period));

        if let Some(thread_id) = thread_id {
            query = query.bind(thread_id);
        }

        unwrap_fetch_all!(&self.db.pool(), query)
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
        let repo = ThreadRepository::new(test_db().await);

        for i in 1..=20 {
            repo.get_thread_by_id(i + 2000).await.unwrap();
            repo.get_thread_by_id(i + 3000).await.unwrap();
        }

        let posts = repo.get_latest_posts(None, Some(10), None).await;

        for i in 1..=10i64 {
            assert_eq!(posts[i as usize - 1].id, 4021 - i);
        }

        let posts = repo
            .get_latest_posts(None, Some(5), Some(4011))
            .await;

        for i in 1..=5i64 {
            assert_eq!(posts[i as usize - 1].id, 4011 - i);
        }

        repo.get_hot_posts(None, Some(7)).await;
    }
}
