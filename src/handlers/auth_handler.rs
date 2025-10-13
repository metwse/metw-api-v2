use crate::{
    dto::auth::{AuthUserDto, AuthError, TokenDto, error_examples},
    response::AppResult,
};

/// Register a new account.
///
/// Registering an user account, initializes an empty profile and thread for
/// the profile.
#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthUserDto,
    responses(
        (status = CREATED, description = "Register a new account", body = TokenDto),
        error_examples::CannotCreateAccountDto,
        error_examples::InappropriatePasswordOrUsernameDto,
    ),
)]
pub async fn register() -> AppResult<TokenDto> {
    Err(AuthError::RegistrationRejected.into())
}

/// Logs into user account.
///
/// Returns a authentication JWT.
#[utoipa::path(
    post,
    path = "/login",
    request_body = AuthUserDto,
    responses(
        (status = OK, description = "Log into existing account", body = TokenDto),
        error_examples::InvalidCredentialsDto,
    ),
)]
pub async fn login() -> AppResult<TokenDto> {
    todo!()
}
