use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

api_errors!(
    AuthError,
    {
        CannotCreateAccount => (
            kinds(AccountCreationDisabled),
            description("Account creation is disabled."),
            status_code(FORBIDDEN),
        ),
        InappropriatePasswordOrUsername => (
            kinds(UsernameRejected | PasswordRejected),
            description("Could not create an accout with provided password and username."),
            status_code(BAD_REQUEST),
        ),
        InvalidCredentials => (
            kinds(InvalidCredentials),
            description("The username or password you entered is incorrect."),
            status_code(UNAUTHORIZED),
        )
    }
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
