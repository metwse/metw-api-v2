use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

/// Result of a API handler
pub type AppResult<T> = Result<AppOk<T>, AppError>;

/// Indicates successful API response
pub struct AppOk<T>(T);

/// Application level error reporting
#[derive(Debug, Error)]
pub enum AppError {
    /// Unknow error
    #[error("Internal server error")]
    InternalServerError,
}

impl<T: Serialize> IntoResponse for AppOk<T> {
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Json(json!({
            "message": self.to_string(),
            "status": status.as_u16()
        }))
        .into_response()
    }
}
