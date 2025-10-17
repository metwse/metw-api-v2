use crate::{
    dto::user::{FullProfileDto, UserDto, UserStatsDto},
    entity,
    repository::{ThreadRepository, UserRepository},
    snowflake,
    state::Database,
    util::{argon2_hash, argon2_verify},
};
use sqlx::types::BitVec;

/// Service struct for handling user-related operations such as creating,
/// updating, deleting, and fetching users.
///
/// It uses a repository pattern to abstract the data access layer.
pub struct UserService {
    db: Database,
    thread_repo: ThreadRepository,
    repo: UserRepository,
}

impl UserService {
    /// Creates a new repository instance.
    pub fn new(db: Database) -> Self {
        Self {
            repo: UserRepository::new(db.clone()),
            thread_repo: ThreadRepository::new(db.clone()),
            db,
        }
    }

    /// Finds an user from its ID.
    pub async fn get_user_by_id(&self, id: i64) -> Option<UserDto> {
        self.repo.get_user_by_id(id).await
    }

    /// Finds an user from its username.
    pub async fn get_user_by_username(&self, username: &str) -> Option<UserDto> {
        self.repo.get_user_by_username(username).await
    }

    /// Fetches user's profile from its ID.
    pub async fn get_profile_by_id(&self, user_id: i64) -> Option<FullProfileDto> {
        self.repo.get_profile_by_id(user_id).await
    }

    /// Fetches user's profile from its ID.
    pub async fn get_profile_by_username(&self, username: &str) -> Option<FullProfileDto> {
        self.repo.get_profile_by_username(username).await
    }

    /// Fetches user's stats from its ID.
    pub async fn get_user_stats_by_id(&self, id: i64) -> Option<UserStatsDto> {
        self.repo.get_user_stats_by_id(id).await
    }

    /// Fetches user's stats from its ID.
    pub async fn get_follows(
        &self,
        id: i64,
        limit: Option<u64>,
        before: Option<i64>,
    ) -> Vec<UserDto> {
        self.repo.get_follows(id, limit, before).await
    }

    /// Fetches user's stats from its ID.
    pub async fn get_followers(
        &self,
        id: i64,
        limit: Option<u64>,
        before: Option<i64>,
    ) -> Vec<UserDto> {
        self.repo.get_followers(id, limit, before).await
    }

    /// Checks user's password.
    pub async fn validate_password_of_user_id(&self, user_id: i64, password: String) -> bool {
        if let Some(password_hash) = self.repo.get_user_password_hash_by_id(user_id).await {
            argon2_verify(password, password_hash).await
        } else {
            false
        }
    }

    /// Checks user's password, with username.
    pub async fn validate_password_of_username(&self, username: &str, password: String) -> bool {
        if let Some(password_hash) = self.repo.get_user_password_hash_by_username(username).await {
            argon2_verify(password, password_hash).await
        } else {
            false
        }
    }

    /// Creates an account with comment thread and profile.
    ///
    /// Returns full profile of the created user if succeedded.
    pub async fn create_user(&self, username: String, password: String) -> Option<FullProfileDto> {
        let password_hash = argon2_hash(password).await?;

        let user_id = snowflake();
        let thread_id = snowflake();

        let mut tx = self.db.pool().begin().await.ok()?;

        self.repo
            .create_user(
                &mut tx,
                entity::User {
                    id: user_id,
                    username: username.clone(),
                    flags: BitVec::from_elem(2, false),
                },
                password_hash,
            )
            .await?;

        self.thread_repo
            .create_thread(
                &mut tx,
                entity::Thread {
                    id: thread_id,
                    user_id,
                },
            )
            .await?;

        self.repo
            .create_profile(
                &mut tx,
                entity::Profile {
                    user_id,
                    comments_thread_id: thread_id,
                    avatar_id: None,
                    banner_id: None,
                    bio: String::from(""),
                },
            )
            .await?;

        tx.commit().await.ok()?;

        Some(FullProfileDto {
            id: user_id,
            username,
            avatar_id: None,
            banner_id: None,
            bio: String::from(""),
            flags: BitVec::from_elem(2, false),
            comments_thread_id: thread_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::test_db;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn account_creation() {
        let db = test_db().await;
        let service = UserService::new(db);

        let username = format!("{}", snowflake());
        let password = format!("{}", snowflake());
        let user = service
            .create_user(username.clone(), password.clone())
            .await
            .unwrap();

        service.get_user_by_id(user.id).await.unwrap();
        service.get_user_by_username(&username).await.unwrap();
        service.get_profile_by_id(user.id).await.unwrap();

        assert!(
            service
                .validate_password_of_user_id(user.id, password.clone())
                .await
        );
        assert!(
            service
                .validate_password_of_username(&username, password)
                .await
        );
    }
}
