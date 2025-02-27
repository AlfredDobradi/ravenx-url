use crate::api::error::ApiError;
use axum::extract::{Path, State};
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use redis::Commands;
use tracing::{debug, info};
use crate::redict::Connection;

pub async fn handle_redirect(
    State(redis): State<redis::Client>,
    Path(url_key): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut con = Connection::from((redis.get_connection()?, "v1.0.0".to_string()));

    if let Ok(u) = con.get_item(&url_key) {
        info!("{}", format!("redirecting path /{} to {}", url_key, u.url));
        Ok((StatusCode::TEMPORARY_REDIRECT, [(LOCATION, u.url)], "").into_response())
    } else {
        info!("{}", format!("path /{} not found", url_key));
        Err(ApiError::StatusCode(StatusCode::NOT_FOUND))
    }
}
