use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
use chrono::{Duration, Utc};
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user id
    pub username: String,
    pub exp: i64,         // expiration timestamp
    pub iat: i64,         // issued at timestamp
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiry: Duration,
    refresh_token_expiry: Duration,
}

impl JwtService {
    pub fn new() -> Result<Self> {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-for-development-only".to_string());
        
        let access_expiry = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<i64>()
            .unwrap_or(3600);
            
        let refresh_expiry = env::var("REFRESH_TOKEN_EXPIRATION")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<i64>()
            .unwrap_or(604800);

        Ok(JwtService {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            access_token_expiry: Duration::seconds(access_expiry),
            refresh_token_expiry: Duration::seconds(refresh_expiry),
        })
    }

    pub fn generate_access_token(&self, user_id: Uuid, username: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + self.access_token_expiry;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Access,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to generate access token: {}", e))
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, username: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + self.refresh_token_expiry;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Refresh,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to generate refresh token: {}", e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        ).map_err(|e| anyhow!("Failed to verify token: {}", e))?;

        Ok(token_data.claims)
    }

    pub fn get_access_token_expiry(&self) -> i64 {
        self.access_token_expiry.num_seconds()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let jwt_service = JwtService::new().unwrap();
        let user_id = Uuid::new_v4();
        let username = "testuser";

        let token = jwt_service.generate_access_token(user_id, username).unwrap();
        let claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert!(matches!(claims.token_type, TokenType::Access));
    }
}