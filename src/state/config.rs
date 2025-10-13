use std::env;

/// API configuration
#[derive(Debug)]
pub struct Config {
    /// Postgres database connection string
    pub database_url: String,
    /// Redis connection string
    pub redis_url: String,
    /// Secret for signing JWTs
    pub jwt_secret: String,
    /// Whether or not to allow account registrations.
    pub allow_account_creation: bool
}

impl Config {
    /// Loads application configuration from environment.
    pub fn from_env(
        env_filename: Option<impl AsRef<std::path::Path>>,
    ) -> Result<Self, env::VarError> {
        if let Some(env_filename) = env_filename {
            dotenv::from_filename(env_filename).ok();
        }

        let config = Self {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            allow_account_creation: env::var("DISABLE_ACCOUNT_CREATION").is_err(),
        };

        tracing::info!(?config, "Config loaded");

        Ok(config)
    }
}
