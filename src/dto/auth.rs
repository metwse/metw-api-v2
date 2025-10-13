use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

api_errors!(
    AuthError,
    responses(
        CannotCreateAccount = (
            status = FORBIDDEN,
            description = "Server rejected to create the account.",
            variants = (RegistrationRejected = "Could not create account.")
        ),
        InappropriatePasswordOrUsername = (
            status = BAD_REQUEST,
            description = "Could not create an account with provided password and username.",
            variants = (
                UsernameTooShort = "Username is too short.",
                UsernameTaken((String)) = "Username {0} has been taken." ((String::from("example"))),
                UsernameTooLong = "Username is too long.",
                PasswordRejected = "Invalid password.",
            )
        ),
        InvalidCredentials = (
            status = UNAUTHORIZED,
            description = "The username or password you entered is incorrect.",
            variants = (
                UserNotFound((String)) = "User {0} has not found." ((String::from("example"))),
                InvalidPassword = "Invalid password.",
            )
        ),
    )
);

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
