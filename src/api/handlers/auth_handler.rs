use crate::{
    AppState,
    dto::auth::{AccountCreationDto, AuthError, AuthUserDto, TokenDto, error_examples},
    response::{AppOk, AppResult},
    service::token_service::AuthToken,
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
        (status = CREATED, description = "Account info for registered account", body = AccountCreationDto),
        error_examples::CannotCreateAccountDto,
        error_examples::InappropriatePasswordOrUsernameDto,
    ),
)]
pub async fn register(
    state: State<AppState>,
    Json(credentials): Json<AuthUserDto>,
) -> AppResult<AccountCreationDto> {
    if !state.config.allow_account_creation {
        return Err(AuthError::RegistrationRejected.into());
    }

    match credentials.username.len() {
        3..=20 => (),
        v if v < 3 => return Err(AuthError::UsernameTooShort.into()),
        _ => return Err(AuthError::UsernameTooLong.into()),
    };

    if let Some(user) = state
        .user_service
        .create_user(credentials.username.clone(), credentials.password)
        .await
    {
        let dto = AccountCreationDto {
            token: state
                .auth_token_service
                .sign(AuthToken::new(user.id, credentials.username))
                .await,
            user,
        };

        AppOk(dto).into()
    } else {
        Err(AuthError::UsernameTaken(credentials.username).into())
    }
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
