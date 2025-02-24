use axum::{debug_handler, Json};
use axum::extract::Path;
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::debug;
use crate::api::error::ApiError;
use crate::CONFIG;
use crate::config::Url;

pub async fn handle_redirect(Path(url_key): Path<String>) -> Result<impl IntoResponse, ApiError> {
    if let Some(u) = CONFIG.urls.get(&url_key) {
        debug!("{}", format!("redirecting path /{} to {}", url_key, &u.url));
        Ok((StatusCode::TEMPORARY_REDIRECT, [(LOCATION, u.url.clone())], "").into_response())
    } else {
        debug!("{}", format!("path /{} not found", url_key));
        Err(ApiError::StatusCode(StatusCode::NOT_FOUND))
    }
}
