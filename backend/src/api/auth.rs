use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::auth::{jwt::JwtService, password};
use crate::database::{connection::Database, models::{CreateUserRequest, LoginRequest, LoginResponse, UserSummary}, queries::UserQueries};
use crate::utils::{errors::AppError, validation};

pub async fn register(
    State(app_state): State<crate::AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db = &app_state.database;
    let jwt_service = &app_state.jwt_service;
    // Validate input
    validation::validate_email(&request.email)?;
    validation::validate_username(&request.username)?;
    validation::validate_password(&request.password)?;
    validation::validate_display_name(&request.display_name)?;

    // Check if email or username already exists
    if UserQueries::check_email_exists(db.pool(), &request.email).await? {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    if UserQueries::check_username_exists(db.pool(), &request.username).await? {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Hash password
    let password_hash = password::hash_password(&request.password)
        .map_err(|e| AppError::InternalServer(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = UserQueries::create_user(db.pool(), &request, &password_hash).await?;

    // Generate tokens
    let access_token = jwt_service
        .generate_access_token(user.id, &user.username)
        .map_err(|e| AppError::InternalServer(format!("Failed to generate access token: {}", e)))?;

    let refresh_token = jwt_service
        .generate_refresh_token(user.id, &user.username)
        .map_err(|e| AppError::InternalServer(format!("Failed to generate refresh token: {}", e)))?;

    // Create response
    let response = LoginResponse {
        user,
        access_token,
        refresh_token,
        expires_in: jwt_service.get_access_token_expiry(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(app_state): State<crate::AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db = &app_state.database;
    let jwt_service = &app_state.jwt_service;
    // Validate input
    validation::validate_email(&request.email)?;

    // Get user by email
    let user = UserQueries::get_user_by_email(db.pool(), &request.email)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // Verify password
    let is_valid = password::verify_password(&request.password, &user.password_hash)
        .map_err(|e| AppError::InternalServer(format!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    // Generate tokens
    let access_token = jwt_service
        .generate_access_token(user.id, &user.username)
        .map_err(|e| AppError::InternalServer(format!("Failed to generate access token: {}", e)))?;

    let refresh_token = jwt_service
        .generate_refresh_token(user.id, &user.username)
        .map_err(|e| AppError::InternalServer(format!("Failed to generate refresh token: {}", e)))?;

    // Create response
    let response = LoginResponse {
        user,
        access_token,
        refresh_token,
        expires_in: jwt_service.get_access_token_expiry(),
    };

    Ok(Json(response))
}

pub async fn refresh_token(
    State(app_state): State<crate::AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let db = &app_state.database;
    let jwt_service = &app_state.jwt_service;
    // Extract refresh token from request
    let refresh_token = payload
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Refresh token is required".to_string()))?;

    // Verify refresh token
    let claims = jwt_service
        .verify_token(refresh_token)
        .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

    // Verify it's a refresh token
    if !matches!(claims.token_type, crate::auth::jwt::TokenType::Refresh) {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    // Parse user ID and get user
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    let user = UserQueries::get_user_by_id(db.pool(), user_id)
        .await
        .map_err(|_| AppError::Unauthorized("User not found".to_string()))?;

    // Generate new access token
    let access_token = jwt_service
        .generate_access_token(user.id, &user.username)
        .map_err(|e| AppError::InternalServer(format!("Failed to generate access token: {}", e)))?;

    let response = json!({
        "access_token": access_token,
        "expires_in": jwt_service.get_access_token_expiry()
    });

    Ok(Json(response))
}

pub async fn logout() -> Result<impl IntoResponse, AppError> {
    // For JWT-based auth, logout is typically handled client-side
    // by removing the tokens from storage
    Ok((StatusCode::NO_CONTENT, Json(json!({}))))
}