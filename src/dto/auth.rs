use super::user::FullProfileDto;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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
                UsernameTooShort = "Username cannot contain less than 3 characters.",
                UsernameTaken((String)) = "Username {0} has been taken."((String::from("example"))),
                UsernameTooLong = "Username cannot contain more than 20 characters.",
                UsernameRejected = "Username contains inappropriate characters.",
                PasswordRejected = "Invalid password.",
            )
        ),
        InvalidCredentials = (
            status = UNAUTHORIZED,
            description = "The username or password you entered is incorrect.",
            variants = (InvalidCredentials = "Provided username or password is incorrect",)
        ),
    )
);

lazy_static!(
    /// Regex for character validation in usernames
    pub static ref USERNAME_REGEX: Regex =
        Regex::new("^[a-z0-9]+(?:[_-][a-z0-9]+)*$")
            .unwrap();
);

/// Username and password for account creation or login
#[derive(Deserialize, ToSchema, Validate)]
pub struct AuthUserDto {
    /// Username
    ///
    /// ATTENTION: [`Validate`] implementation does not checks username length.
    #[validate(regex(path = *USERNAME_REGEX), length(min = 2, max = 20))]
    pub username: String,
    /// Password
    pub password: String,
}

/// Account token with full profile
#[derive(Serialize, ToSchema)]
pub struct TokenDto {
    /// JSON Web Token
    pub token: String,
    /// User's profile
    pub user: FullProfileDto,
}

#[cfg(test)]
#[test]
fn username_validation() {
    let valid_usernames = ["aa", "12345678901234567890", "a-a", "a_a", "a-b-c"];
    let invalid_usernames = ["_a", "a", "123456789012345678901", "!!", "a_"];

    for username in valid_usernames {
        AuthUserDto {
            username: String::from(username),
            password: String::from(""),
        }
        .validate()
        .unwrap();
    }

    for username in invalid_usernames {
        assert!(
            AuthUserDto {
                username: String::from(username),
                password: String::from(""),
            }
            .validate()
            .is_err()
        );
    }
}
