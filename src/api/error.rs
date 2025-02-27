use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(anyhow::Error),
    StatusCode(StatusCode),
    UuidParseError(uuid::Error),
    RedisError(redis::RedisError),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::InternalServerError(err)
    }
}

impl From<StatusCode> for ApiError {
    fn from(code: StatusCode) -> Self {
        ApiError::StatusCode(code)
    }
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        ApiError::UuidParseError(err)
    }
}

impl From<redis::RedisError> for ApiError {
    fn from(err: redis::RedisError) -> Self {
        ApiError::RedisError(err)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InternalServerError(rejection) => {
                write!(f, "{}", rejection)
            }
            ApiError::StatusCode(code) => {
                write!(f, "{}", code)
            }
            ApiError::UuidParseError(rejection) => {
                write!(f, "{}", rejection)
            }
            ApiError::RedisError(rejection) => {
                write!(f, "{}", rejection)
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        let (status, message) = match self {
            ApiError::InternalServerError(rejection) => {
                (StatusCode::INTERNAL_SERVER_ERROR, rejection.to_string())
            }
            ApiError::StatusCode(code) => (
                code,
                code.canonical_reason()
                    .unwrap_or("Internal Server Error")
                    .to_string(),
            ),
            ApiError::UuidParseError(rejection) => {
                (StatusCode::INTERNAL_SERVER_ERROR, rejection.to_string())
            },
            ApiError::RedisError(rejection) => {
                (StatusCode::INTERNAL_SERVER_ERROR, rejection.to_string())
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
