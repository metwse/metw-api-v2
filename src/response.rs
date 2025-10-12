use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::dto::gateway::GatewayError;

/// Result of a API handler
pub type AppResult<T> = Result<AppOk<T>, AppError>;

/// Indicates successful API response
pub struct AppOk<T>(T);

/// Application level error reporting
#[derive(Debug, Error, Serialize, ToSchema)]
pub enum AppError {
    /// Unknow error
    #[error("internal server error")]
    InternalServerError,
    /// /gateway error types.
    #[error("gateway error: {0}")]
    GatewayError(#[from] GatewayError),
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

impl AppError {
    /// HTTP status code of the error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GatewayError(err) => err.into(),
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

impl<T: Serialize> IntoResponse for AppOk<T> {
    fn into_response(self) -> Response {
        Json(self.0).into_response()
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
