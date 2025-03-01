use crate::api::error::ApiError;
use crate::redict::Connection;
use axum::extract::{Path, Query, State};
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::string::ToString;
use tracing::debug;

static KEY_VERSION: &str = "1";

pub async fn handle_redirect(
    State(redis): State<redis::Client>,
    Path(url_key): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut con = Connection::from((redis.get_connection()?, KEY_VERSION.to_string()));

    if let Ok(u) = con.get_item(&url_key) {
        debug!("{}", format!("redirecting path /{} to {}", url_key, u.url));
        Ok((StatusCode::TEMPORARY_REDIRECT, [(LOCATION, u.url)], "").into_response())
    } else {
        debug!("{}", format!("path /{} not found", url_key));
        Err(ApiError::StatusCode(StatusCode::NOT_FOUND))
    }
}

pub async fn handle_post(
    State(redis): State<redis::Client>,
    Path(url_key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let url = params
        .get("to")
        .ok_or(ApiError::StatusCode(StatusCode::BAD_REQUEST))?;

    let mut con = Connection::from((redis.get_connection()?, KEY_VERSION.to_string()));
    con.add_item(&url_key, url.to_string(), false)?;

    Ok(StatusCode::CREATED)
}

pub async fn handle_put(
    State(redis): State<redis::Client>,
    Path(url_key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let url = params
        .get("to")
        .ok_or(ApiError::StatusCode(StatusCode::BAD_REQUEST))?;

    let mut con = Connection::from((redis.get_connection()?, KEY_VERSION.to_string()));
    con.add_item(&url_key, url.to_string(), true)?;

    Ok(StatusCode::NO_CONTENT)
}
