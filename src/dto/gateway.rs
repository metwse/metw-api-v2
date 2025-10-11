use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Username and password for account creation or login
#[derive(Deserialize, ToSchema)]
pub struct AuthUserDto {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Login credentials of an account
#[derive(Serialize, ToSchema)]
pub struct TokenDto {
    /// JSON Web Token
    pub token: String,
}
