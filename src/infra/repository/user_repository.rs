use crate::{dto::user::FullProfileDto, entity, state::Database};
use sqlx::PgTransaction;

/// User data access repository
pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_user_by_id(&self, id: i64) -> Option<entity::User> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::User>(
                r#"SELECT id, username, flags
                FROM users
                WHERE id = $1"#,
            )
            .bind(id)
        )
    }

    pub async fn get_user_by_username(&self, username: &str) -> Option<entity::User> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, entity::User>(
                r#"SELECT id, username, flags
                FROM users
                WHERE username = $1::varchar"#,
            )
            .bind(username)
        )
    }

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

    pub async fn get_full_profile_by_user_id(&self, user_id: i64) -> Option<FullProfileDto> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, FullProfileDto>(
                r#"SELECT id, username, flags, comments_thread_id, avatar_id, banner_id, bio
                FROM profiles LEFT JOIN users ON user_id = users.id
                WHERE user_id = $1"#,
            )
            .bind(user_id)
        )
    }

    pub async fn get_user_password_hash_by_user_id(&self, id: i64) -> Option<String> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, (String,)>("SELECT password_hash FROM users WHERE id = $1")
                .bind(id)
        )
        .map(|row| row.0)
    }

    pub async fn get_user_password_hash_by_username(&self, username: &str) -> Option<String> {
        unwrap_fetch_one!(
            &self.db.pool(),
            sqlx::query_as::<_, (String,)>("SELECT password_hash FROM users WHERE username = $1")
                .bind(username)
        )
        .map(|row| row.0)
    }

    pub async fn create_user(
        &self,
        tx: &mut PgTransaction<'_>,
        user: entity::User,
        password_hash: String,
    ) -> Option<()> {
        unwrap_execute!(
            &mut **tx,
            sqlx::query(
                r#"INSERT INTO users (id, username, password_hash, flags)
                    VALUES
                ($1, $2, $3, $4)"#,
            )
            .bind(user.id)
            .bind(user.username)
            .bind(password_hash)
            .bind(user.flags)
        )?;

        Some(())
    }

    pub async fn create_profile(
        &self,
        tx: &mut PgTransaction<'_>,
        profile: entity::Profile,
    ) -> Option<()> {
        unwrap_execute!(
            &mut **tx,
            sqlx::query(
                r#"INSERT INTO profiles (user_id, comments_thread_id, avatar_id, banner_id, bio)
                    VALUES
                ($1, $2, $3, $4, $5)"#,
            )
            .bind(profile.user_id)
            .bind(profile.comments_thread_id)
            .bind(profile.avatar_id)
            .bind(profile.banner_id)
            .bind(profile.bio)
        )?;

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repository::ThreadRepository, snowflake, testutil::test_db};
    use serial_test::serial;
    use sqlx::types::BitVec;

    #[serial]
    #[tokio::test]
    async fn queries() {
        let repo = UserRepository::new(test_db().await);

        for i in 1..=9 {
            repo.get_user_by_id(i + 1000).await.unwrap();
            repo.get_user_by_username(&format!("user0{i}"))
                .await
                .unwrap();
            repo.get_profile_by_user_id(i + 1000).await.unwrap();
            repo.get_full_profile_by_user_id(i + 1000).await.unwrap();
        }

        assert!(repo.get_user_by_id(999).await.is_none());
    }

    #[serial]
    #[tokio::test]
    async fn account_creation() {
        let db = test_db().await;
        let repo = UserRepository::new(db.clone());
        let thread_repo = ThreadRepository::new(db.clone());

        let mut tx = db.pool().begin().await.unwrap();

        let user_id = snowflake();
        let thread_id = snowflake();
        let username = format!("{}", snowflake());
        let password_hash = format!("{}", snowflake());
        let bio = format!("{}", snowflake());

        repo.create_user(
            &mut tx,
            entity::User {
                id: user_id,
                username: username.clone(),
                flags: BitVec::from_elem(2, false),
            },
            password_hash.clone(),
        )
        .await
        .unwrap();

        thread_repo
            .create_thread(
                &mut tx,
                entity::Thread {
                    id: thread_id,
                    user_id,
                },
            )
            .await
            .unwrap();

        repo.create_profile(
            &mut tx,
            entity::Profile {
                user_id,
                comments_thread_id: thread_id,
                avatar_id: None,
                banner_id: None,
                bio: bio.clone(),
            },
        )
        .await
        .unwrap();

        tx.commit().await.unwrap();

        assert_eq!(
            repo.get_user_password_hash_by_user_id(user_id)
                .await
                .unwrap(),
            password_hash.clone()
        );

        assert_eq!(
            repo.get_user_password_hash_by_username(&username)
                .await
                .unwrap(),
            password_hash.clone()
        );
    }
}
