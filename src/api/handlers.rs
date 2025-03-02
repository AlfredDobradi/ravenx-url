use crate::api::error::ApiError;
use crate::redict::Connection;
use axum::extract::{Path, Query, State};
use axum::http::header::{self, LOCATION};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::string::ToString;
use tracing::debug;

use super::state::AppState;

#[tracing::instrument]
pub async fn handle_index(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let redis = state.redis.clone();
    let cfg = state.config.clone();

    let accept: &str = match headers.get(header::ACCEPT) {
        Some(a) => match a.to_str().unwrap_or("text/plain") {
            "application/json" => "application/json",
            _ => "text/plain",
        },
        None => "text/plain",
    };

    let mut con = Connection::from((redis.get_connection()?, cfg.key_version));

    let list = con.get_items()?;

    match accept {
        "application/json" => Ok((
            StatusCode::OK,
            [("Content-Type", "application/json")],
            serde_json::to_string(&list).unwrap(),
        )),
        _ => Ok((
            StatusCode::OK,
            [("Content-Type", "text/plain")],
            list.iter().fold("".to_string(), |str, url| {
                format!("{}{} -> {}\n", str, url.key, url.url)
            }),
        )),
    }
}

#[tracing::instrument]
pub async fn handle_redirect(
    State(state): State<AppState>,
    Path(url_key): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let redis = state.redis.clone();
    let cfg = state.config.clone();

    let mut con = Connection::from((redis.get_connection()?, cfg.key_version));

    if let Ok(u) = con.get_item(&url_key) {
        con.increase_hits(&url_key)?;

        debug!("{}", format!("redirecting path /{} to {}", url_key, u.url));
        Ok((StatusCode::TEMPORARY_REDIRECT, [(LOCATION, u.url)], "").into_response())
    } else {
        debug!("{}", format!("path /{} not found", url_key));
        Err(ApiError::StatusCode(StatusCode::NOT_FOUND))
    }
}

#[tracing::instrument]
pub async fn handle_post(
    State(state): State<AppState>,
    Path(url_key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let redis = state.redis.clone();
    let cfg = state.config.clone();

    let url = params
        .get("to")
        .ok_or(ApiError::StatusCode(StatusCode::BAD_REQUEST))?;

    let mut con = Connection::from((redis.get_connection()?, cfg.key_version));
    con.add_item(&url_key, url.to_string(), false)?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument]
pub async fn handle_put(
    State(state): State<AppState>,
    Path(url_key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiError> {
    let redis = state.redis.clone();
    let cfg = state.config.clone();

    let url = params
        .get("to")
        .ok_or(ApiError::StatusCode(StatusCode::BAD_REQUEST))?;

    let mut con = Connection::from((redis.get_connection()?, cfg.key_version));
    con.add_item(&url_key, url.to_string(), true)?;

    Ok(StatusCode::NO_CONTENT)
}
