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
