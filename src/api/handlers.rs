use axum::extract::{Path, State};
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use redis::Commands;
use tracing::debug;
use crate::api::error::ApiError;

pub async fn handle_redirect(
    State(redis): State<redis::Client>,
    Path(url_key): Path<String>
) -> Result<impl IntoResponse, ApiError> {
    let mut con = redis.get_connection()?;

    if let Ok(u) = con.get::<&String, String>(&url_key) {
        debug!("{}", format!("redirecting path /{} to {}", url_key, u));
        Ok((StatusCode::TEMPORARY_REDIRECT, [(LOCATION, u)], "").into_response())
    } else {
        debug!("{}", format!("path /{} not found", url_key));
        Err(ApiError::StatusCode(StatusCode::NOT_FOUND))
    }
}
