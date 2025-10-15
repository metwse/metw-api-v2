use crate::{entity, state::Database};
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
    }
}
