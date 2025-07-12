use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::auth::jwt::{JwtService, Claims, TokenType};
use crate::utils::errors::AppError;

pub async fn auth_middleware(
    State(jwt_service): State<JwtService>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization format".to_string()));
    }

    // Extract token
    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Verify token
    let claims = jwt_service
        .verify_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    // Only allow access tokens for API endpoints
    if !matches!(claims.token_type, TokenType::Access) {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    // Parse user ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Add user info to request extensions
    req.extensions_mut().insert(CurrentUser {
        id: user_id,
        username: claims.username,
    });

    Ok(next.run(req).await)
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub username: String,
}

impl CurrentUser {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}