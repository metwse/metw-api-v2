use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unix timestamp as u64.
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Runs argon2 password hashing function in blocking thread.
pub async fn argon2_hash(password: String) -> Option<String> {
    tokio::task::spawn_blocking(move || -> Option<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        Some(
            argon2
                .hash_password(password.as_bytes(), &salt)
                .ok()?
                .to_string(),
        )
    })
    .await
    .unwrap()
}

/// Verifies argon2-hashed passwords.
pub async fn argon2_verify(password: String, password_hash: String) -> bool {
    tokio::task::spawn_blocking(move || {
        if let Ok(parsed_hash) = PasswordHash::new(&password_hash) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    })
    .await
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util::{argon2_hash, argon2_verify, timestamp};

    #[tokio::test]
    async fn argon2() {
        let password = format!("{}", timestamp());

        let password_hash = argon2_hash(password.clone()).await.unwrap();

        assert!(argon2_verify(password, password_hash).await);
    }
}
