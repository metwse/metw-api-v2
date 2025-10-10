use sqlx::PgPool;

/// Postgres connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Creates a new [`PgPool`] wrapped with [`Database`].
    pub async fn new(con_url: &str) -> Database {
        let pool = PgPool::connect(con_url).await.unwrap();
        tracing::trace!("Connected to the PostgreSQL database");

        Database { pool }
    }

    /// Returns a clone of underlying PostgreSQL connection pool.
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
