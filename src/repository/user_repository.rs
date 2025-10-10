use crate::{state::Database, entity};

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
    pub async fn get_user_by_id(&self, id: i64) -> Option<entity::User> {
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
    pub async fn get_user_by_username(&self, username: &str) -> Option<entity::User> {
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
    pub async fn get_profile_by_user_id(&self, user_id: i64) -> Option<entity::Profile> {
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
    use crate::state::Config;
    use super::*;

    #[tokio::test]
    async fn test_queries() {
        let config = Config::from_env(Some(".env.test")).unwrap();

        let db = Database::new(&config.database_url).await;

        let repo = UserRepository::new(db);

        for i in 1..=9 {
            repo.get_user_by_id(i + 1000).await.unwrap();
            repo.get_user_by_username(&format!("user0{i}")).await.unwrap();
            repo.get_profile_by_user_id(i + 1000).await.unwrap();
        }

        assert!(repo.get_user_by_id(999).await.is_none());
    }
}
