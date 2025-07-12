use axum::{
    routing::{get, post},
    middleware,
    Router,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber;

mod api;
mod auth;
mod database;
mod utils;
mod websocket;

use auth::jwt::JwtService;
use database::connection::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub jwt_service: JwtService,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "SimpleCards API v0.1.0".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .compact()
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Initialize database
    let database = Database::new().await?;
    info!("Database connected and migrations applied");

    // Initialize JWT service
    let jwt_service = JwtService::new()?;
    info!("JWT service initialized");

    // Create app state
    let app_state = AppState {
        database,
        jwt_service: jwt_service.clone(),
    };

    // Build protected routes (require authentication)
    let protected_routes = Router::new()
        .route("/users/me", get(api::users::get_current_user))
        .route("/users/me", post(api::users::update_current_user))
        .layer(middleware::from_fn_with_state(
            jwt_service,
            auth::middleware::auth_middleware,
        ));

    // Build public routes
    let public_routes = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ready", get(health))
        .route("/auth/register", post(api::auth::register))
        .route("/auth/login", post(api::auth::login))
        .route("/auth/refresh", post(api::auth::refresh_token))
        .route("/auth/logout", post(api::auth::logout));

    // Combine routes
    let app = Router::new()
        .nest("/api", protected_routes)
        .merge(public_routes.clone())
        .nest("/api", public_routes)
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    info!("SimpleCards backend starting on http://{}:{}", host, port);
    info!("Health check available at http://{}:{}/health", host, port);
    info!("API documentation: http://{}:{}/api", host, port);

    // Run the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
