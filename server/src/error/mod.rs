use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Wrong username or password")]
    UsernameOrPasswordError,

    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

pub type Result<T> = std::result::Result<T, ServerError>;

pub type ApiError = (StatusCode, Json<Value>);

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<ServerError> for ApiError {
    fn from(err: ServerError) -> Self {
        let status = match err {
            ServerError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServerError::UsernameOrPasswordError => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}