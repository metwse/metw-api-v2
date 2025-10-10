use crate::{
    dto::gateway::{AuthUserDto, TokenDto},
    response::AppResult,
};

/// A router for creating user
#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthUserDto,
    responses(
        (status = CREATED, description = "Register a new account", body = TokenDto)
    )
)]
pub async fn create_user() -> AppResult<TokenDto> {
    todo!()
}

/// A router for logging into an user account
#[utoipa::path(
    post,
    path = "/login",
    request_body = AuthUserDto,
    responses(
        (status = OK, description = "Log into existing account", body = TokenDto)
    )
)]
pub async fn login() -> AppResult<TokenDto> {
    todo!()
}
