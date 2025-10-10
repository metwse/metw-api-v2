use std::env;

/// API configuration
pub struct Config {
    /// Postgres database connection string
    pub database_url: String,
    /// Redis connection string
    pub redis_url: String,
}

impl Config {
    /// Loads application configuration from environment.
    pub fn from_env(
        env_filename: Option<impl AsRef<std::path::Path>>,
    ) -> Result<Self, env::VarError> {
        if let Some(env_filename) = env_filename {
            dotenv::from_filename(env_filename).ok();
        }

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
        })
    }
}
