use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::database::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::utils::errors::AppError;

pub struct UserQueries;

impl UserQueries {
    pub async fn create_user(
        pool: &PgPool,
        request: &CreateUserRequest,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (email, username, display_name, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at
            "#
        )
        .bind(&request.email)
        .bind(&request.username)
        .bind(&request.display_name)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE email = $1 AND is_active = true"
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: Uuid,
        request: &UpdateUserRequest,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE users 
            SET 
                display_name = COALESCE($2, display_name),
                avatar_url = COALESCE($3, avatar_url),
                updated_at = NOW()
            WHERE id = $1 AND is_active = true
            RETURNING id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at
            "#
        )
        .bind(user_id)
        .bind(&request.display_name)
        .bind(&request.avatar_url)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn check_email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }

    pub async fn check_username_exists(pool: &PgPool, username: &str) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }

    pub async fn deactivate_user(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}