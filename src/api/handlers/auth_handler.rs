use crate::{
    AppState,
    dto::auth::{AuthError, AuthUserDto, TokenDto, error_examples},
    response::{AppOk, AppResult},
    service::token_service::AuthToken,
};
use axum::{Json, extract::State};
use validator::Validate;

/// Register a new account.
///
/// Registering an user account, initializes an empty profile and thread for
/// the profile.
#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthUserDto,
    responses(
        (status = CREATED, description = "Account info for registered account", body = TokenDto),
        error_examples::CannotCreateAccountDto,
        error_examples::InappropriatePasswordOrUsernameDto,
    ),
)]
pub async fn register(
    state: State<AppState>,
    Json(mut credentials): Json<AuthUserDto>,
) -> AppResult<TokenDto> {
    if !state.config.allow_account_creation {
        return Err(AuthError::RegistrationRejected.into());
    }

    match credentials.username.len() {
        3..=20 => (),
        v if v < 3 => return Err(AuthError::UsernameTooShort.into()),
        _ => return Err(AuthError::UsernameTooLong.into()),
    };

    credentials.username = credentials.username.to_lowercase();

    if credentials.validate().is_err() {
        return Err(AuthError::UsernameRejected.into());
    }

    if let Some(user) = state
        .user_service
        .create_user(credentials.username.clone(), credentials.password)
        .await
    {
        let dto = TokenDto {
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
pub async fn login(
    state: State<AppState>,
    Json(mut credentials): Json<AuthUserDto>,
) -> AppResult<TokenDto> {
    credentials.username = credentials.username.to_lowercase();

    if credentials.validate().is_err() {
        return Err(AuthError::UsernameRejected.into());
    }

    let is_credentials_valid = state
        .user_service
        .validate_password_of_username(&credentials.username, credentials.password)
        .await;

    if is_credentials_valid {
        let user = state
            .user_service
            .get_profile_by_username(&credentials.username)
            .await;

        if let Some(user) = user {
            let dto = TokenDto {
                token: state
                    .auth_token_service
                    .sign(AuthToken::new(user.id, credentials.username))
                    .await,
                user,
            };

            return AppOk(dto).into();
        }
    }

    Err(AuthError::InvalidCredentials.into())
}
