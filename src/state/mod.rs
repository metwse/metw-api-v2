mod database;

mod redis;

mod config;

pub use config::Config;
pub use database::Database;
pub use redis::Redis;

use std::sync::Arc;

/// Shared state for the application
#[derive(Clone)]
pub struct AppState {
    /// Application configuration
    pub config: Arc<Config>,
    /// Redis connection wrapper
    pub redis: Redis,
    /// Database connection wrapper
    pub database: Database,
}

/// Initializing database connections, builds app state.
pub async fn bootstrap(config: Config) -> AppState {
    let redis = Redis::new(&config.redis_url).await;
    let database = Database::new(&config.database_url).await;

    AppState {
        config: Arc::new(config),
        redis,
        database,
    }
}
