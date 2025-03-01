use axum::{
    body::Body, extract::{Request, State}, http::{header::AUTHORIZATION, HeaderMap, HeaderValue, Response, StatusCode}, middleware::Next,
};
use crate::config::Config;

#[tracing::instrument]
pub async fn auth_middleware(
    State(config): State<Config>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let bearer = format!("Bearer {}", config.auth_token);

    let header = match HeaderValue::from_str(bearer.as_str()) {
        Ok(h) => h,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if headers.get(AUTHORIZATION) == Some(&header) {
        return Ok(next.run(request).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}