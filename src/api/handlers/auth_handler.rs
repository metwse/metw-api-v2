use crate::{
    AppState,
    dto::auth::{AuthError, AuthUserDto, TokenDto, error_examples},
    response::AppResult,
};
use axum::{Json, extract::State};

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
pub async fn register(
    state: State<AppState>,
    Json(credentials): Json<AuthUserDto>,
) -> AppResult<TokenDto> {
    if !state.config.allow_account_creation {
        return Err(AuthError::RegistrationRejected.into());
    }

    match credentials.username.len() {
        3..=20 => (),
        v if v < 3 => return Err(AuthError::UsernameTooShort.into()),
        _ => return Err(AuthError::UsernameTooLong.into()),
    };

    todo!()
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
