use crate::{state::Redis, util::timestamp};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

/// 90 days
static AUTH_TOKEN_TTL: u64 = 3600 * 24 * 90;

/// Basic user account authentication token
#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    /// User ID (sub)
    pub id: i64,
    /// Username
    pub username: String,
    /// Issued at (as Unix timestamp)
    pub iat: u64,
    /// Expires at (as Unix timestamp)
    pub exp: u64,
}

impl WebToken for AuthToken {
    fn sub(&self) -> i64 {
        self.id
    }

    fn exp(&self) -> u64 {
        self.exp
    }

    fn iat(&self) -> u64 {
        self.iat
    }
}

impl AuthToken {
    /// Creates a new authentication token
    pub fn new(id: i64, username: String) -> Self {
        let iat = timestamp();

        AuthToken {
            id,
            username,
            iat,
            exp: iat + AUTH_TOKEN_TTL,
        }
    }
}

/// Creates a new [`AuthToken`] service.
pub fn new_auth_token_service(redis: Redis, secret: &str) -> TokenService<AuthToken> {
    TokenService::new(redis, secret, String::from("auth-token"), AUTH_TOKEN_TTL)
}

/// Service for managing JWT tokens backed by Redis.
pub struct TokenService<T> {
    redis: Redis,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    token_type: PhantomData<T>,
    /// Time to live duration of token in seconds.
    ttl: u64,
    /// Unique key for token type.
    key: String,
}

/// Fields required for a web token verification.
pub trait WebToken {
    /// Returns the subject (i.e., user identifier) this token belongs to.
    fn sub(&self) -> i64;

    /// Returns the expiration as a UTC timestamp.
    fn exp(&self) -> u64;

    /// Returns the issue time as a UTC timestamp.
    fn iat(&self) -> u64;
}

impl<T> TokenService<T>
where
    T: Serialize + DeserializeOwned + WebToken,
{
    /// Creates a new `TokenService` instance for tokens of type `T`.
    pub fn new(redis: Redis, secret: &str, key: String, ttl: u64) -> Self {
        TokenService {
            redis,
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            token_type: PhantomData,
            key,
            ttl,
        }
    }

    /// Signs and encodes the given token into a JWT string.
    pub async fn sign(&self, token: T) -> String {
        jsonwebtoken::encode(&Header::default(), &token, &self.encoding_key).unwrap()
    }

    /// Revokes all tokens associated with the given subject.
    pub async fn revoke_tokens_for(&self, sub: i64) {
        let key = format!("{}:token-revocation:{}", self.key, sub);

        redis::pipe()
            .atomic()
            .set(&key, timestamp())
            .expire(&key, self.ttl as i64 + 1)
            .ignore()
            .query_async::<()>(&mut self.redis.client())
            .await
            .ok();
    }

    /// Validates and decodes the provided JWT string.
    pub async fn validate(&self, jwt: &str) -> Option<T> {
        let mut validation = Validation::default();
        validation.validate_exp = false;

        let token = jsonwebtoken::decode::<T>(jwt, &self.decoding_key, &validation)
            .ok()?
            .claims;

        if token.exp() <= timestamp() {
            return None;
        }

        if let Some(revocation_timestamp) = self
            .redis
            .client()
            .get::<_, Option<u64>>(format!("{}:token-revocation:{}", self.key, token.sub()))
            .await
            .ok()?
        {
            let iat = token.iat();

            if iat <= revocation_timestamp {
                return None;
            }
        }

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::test_redis;
    use serde::Deserialize;
    use serial_test::serial;

    #[derive(Debug, Serialize, Deserialize)]
    struct SampleToken {
        sub: i64,
        iat: u64,
        exp: u64,
    }

    impl WebToken for SampleToken {
        fn exp(&self) -> u64 {
            self.exp
        }

        fn iat(&self) -> u64 {
            self.iat
        }

        fn sub(&self) -> i64 {
            self.sub
        }
    }

    static TOKEN_TTL: u64 = 60;

    #[serial]
    #[tokio::test]
    #[test_log::test]
    async fn revocation() {
        let token_service = TokenService::<SampleToken>::new(
            test_redis().await,
            "jwt-secret",
            String::from("test-1-sample-token"),
            TOKEN_TTL,
        );

        let first_token = token_service
            .sign(SampleToken {
                iat: timestamp() - 1,
                exp: timestamp() + TOKEN_TTL - 1,
                sub: 0,
            })
            .await;

        let token_from_future = token_service
            .sign(SampleToken {
                iat: timestamp() + 16,
                exp: timestamp() + 16 + TOKEN_TTL,
                sub: 0,
            })
            .await;

        assert!(token_service.validate(&first_token).await.is_some());
        assert!(token_service.validate(&token_from_future).await.is_some());

        token_service.revoke_tokens_for(0).await;
        tracing::trace!("Tokens of sub 0 has been revoked.");

        assert!(token_service.validate(&first_token).await.is_none());
        assert!(token_service.validate(&token_from_future).await.is_some());
    }

    #[serial]
    #[tokio::test]
    #[test_log::test]
    async fn invalid() {
        let token_service = TokenService::<SampleToken>::new(
            test_redis().await,
            "jwt-secret",
            String::from("test-2-sample-token"),
            TOKEN_TTL,
        );

        let token = token_service
            .sign(SampleToken {
                iat: timestamp() - TOKEN_TTL - 1,
                exp: timestamp() - 1,
                sub: 1,
            })
            .await;

        assert!(token_service.validate(&token).await.is_none());
        assert!(token_service.validate("invalid-token").await.is_none());
    }
}
