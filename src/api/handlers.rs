use crate::api::error::ApiError;
use crate::redict::Connection;
use axum::extract::{Path, Query, State};
use axum::http::header::{self, LOCATION};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::string::ToString;
use tracing::debug;

static KEY_VERSION: &str = "2";

#[tracing::instrument]
pub async fn handle_index(
    State(redis): State<redis::Client>,
    headers: HeaderMap
) -> Result<impl IntoResponse, ApiError> {
    let accept: &str = match headers.get(header::ACCEPT) {
        Some(a) => {
            match a.to_str().unwrap_or("text/plain") {
                "application/json" => "application/json",
                _ => "text/plain"
            }
        },
        None => {
            "text/plain"
        }
    };

    let mut con = Connection::from((redis.get_connection()?, KEY_VERSION.to_string()));

    let list = con.get_items()?;

    match accept {
        "application/json" => {
            Ok((
                StatusCode::OK,
                [("Content-Type", "application/json")],
                serde_json::to_string(&list).unwrap(),
            ))
        },
        _ => {
            Ok((
                StatusCode::OK,
                [("Content-Type", "text/plain")],
                list.iter().fold("".to_string(), |str, (_key, url)| format!("{}\n{} -> {}", str, url.key, url.url)),
            ))
        }
    }
}

#[tracing::instrument]
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

#[tracing::instrument]
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

#[tracing::instrument]
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
