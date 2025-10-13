use crate::dto::auth::AuthError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

/// Result of a API handler
pub type AppResult<T> = Result<AppOk<T>, AppError>;

/// Indicates successful API response
pub struct AppOk<T>(pub T);

/// Application level error reporting
#[derive(Debug, Error, Serialize, ToSchema)]
pub enum AppError {
    /// Unknown error
    #[error("Internal server error.")]
    InternalServerError,
    /// /auth error types.
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
}

/// Error sent back to clients
#[derive(Serialize, ToSchema)]
pub struct AppErrorDto {
    /// Message indicating error as string
    pub message: String,
    /// HTTP status code
    pub status: u16,
    /// Full error enum
    pub r#type: AppError,
}

impl<T> From<AppOk<T>> for AppResult<T> {
    fn from(value: AppOk<T>) -> Self {
        Ok(value)
    }
}

impl<T: Serialize> IntoResponse for AppOk<T> {
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

impl<T> From<AppError> for AppResult<T> {
    fn from(value: AppError) -> Self {
        Err(value)
    }
}

impl AppError {
    /// HTTP status code of the error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthError(err) => err.into(),
        }
    }

    /// Consuming `self`, creates DTO
    pub fn into_dto(self) -> AppErrorDto {
        AppErrorDto {
            status: self.status_code().as_u16(),
            message: self.to_string(),
            r#type: self,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), self.into_dto()).into_response()
    }
}

impl IntoResponse for AppErrorDto {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
