use redis::aio::ConnectionManager;
use sqlx::PgPool;

/// Redis connection wrapper
#[derive(Clone)]
pub struct Redis {
    con: ConnectionManager,
}

/// Postgres connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Redis {
    /// Creates a new [`ConnectionManager`] wrapped with [`Redis`].
    pub async fn new(con_url: String) -> Redis {
        let client = redis::Client::open(con_url).unwrap();
        let con = client.get_connection_manager().await.unwrap();

        Redis { con }
    }

    /// Returns a clone of underlying connection manager.
    pub fn client(&self) -> ConnectionManager {
        self.con.clone()
    }
}

impl Database {
    /// Creates a new [`PgPool`] wrapped with [`Database`].
    pub async fn new(con_url: String) -> Database {
        let pool = PgPool::connect(&con_url).await.unwrap();

        Database { pool }
    }

    /// Returns a clone of underlying PostgreSQL connection pool.
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
