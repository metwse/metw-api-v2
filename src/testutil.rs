use crate::{
    Config,
    state::{Database, Redis},
};

/// Initializes PostgreSQL connection pool from test environment file.
pub async fn test_db() -> Database {
    let config = Config::from_env(Some(".env.test")).unwrap();
    Database::new(&config.database_url).await
}

/// Initializes Redis connection manager from test environment file.
pub async fn test_redis() -> Redis {
    let config = Config::from_env(Some(".env.test")).unwrap();
    Redis::new(&config.redis_url).await
}
