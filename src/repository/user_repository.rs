use crate::{database::Database, entity};

/// User data access repository
pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    /// Creates a new repository instance.
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Finds an user from its ID.
    pub async fn find(&self, id: i64) -> Option<entity::User> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::User>(
                r#"SELECT id, username, password, flags
                FROM users
                WHERE id = $1"#,
            )
            .bind(id)
        )
    }

    /// Finds an user from its username.
    pub async fn find_by_username(&self, username: &str) -> Option<entity::User> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::User>(
                r#"SELECT id, username, password, flags
                FROM users
                WHERE username = $1::varchar"#,
            )
            .bind(username)
        )
    }

    /// Fetches user's profile from its ID.
    pub async fn fetch_profile(&self, user_id: i64) -> Option<entity::Profile> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::Profile>(
                r#"SELECT user_id, comments_thread_id, avatar_id, banner_id, bio
                FROM profiles
                WHERE user_id = $1"#,
            )
            .bind(user_id)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_queries() {
        let db = Database::new(env::var("DATABASE_URL").unwrap()).await;

        let repo = UserRepository::new(db);

        repo.find(1).await;
        repo.find_by_username("metw").await;
        repo.fetch_profile(1).await;
    }
}
