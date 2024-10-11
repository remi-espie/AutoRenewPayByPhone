use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::sync::Arc;

pub async fn auth_middleware(
    req: Request,
    next: Next,
    bearer_token: Arc<String>,
) -> Result<Response, StatusCode> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", bearer_token.as_str()) {
                return Ok(next.run(req).await);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
