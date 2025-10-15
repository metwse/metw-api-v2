mod database;

mod redis;

mod config;

pub use config::Config;
pub use database::Database;
pub use redis::Redis;

use crate::service::{
    PostService, UserService,
    token_service::{AuthToken, TokenService, new_auth_token_service},
};
use std::sync::Arc;

/// Shared state for the application
#[derive(Clone)]
pub struct AppState {
    /// Application configuration
    pub config: Arc<Config>,
    /// Authentication token service
    pub auth_token_service: Arc<TokenService<AuthToken>>,
    /// User service
    pub user_service: Arc<UserService>,
    /// Post service
    pub post_service: Arc<PostService>,
}

/// Initializing database connections, builds app state.
pub async fn bootstrap(config: Config) -> AppState {
    let redis = Redis::new(&config.redis_url).await;
    let db = Database::new(&config.database_url).await;

    AppState {
        auth_token_service: Arc::new(new_auth_token_service(redis, &config.jwt_secret)),
        user_service: Arc::new(UserService::new(db.clone())),
        post_service: Arc::new(PostService::new(db)),
        config: Arc::new(config),
    }
}
