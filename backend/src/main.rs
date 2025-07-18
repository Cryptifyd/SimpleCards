use axum::{
    routing::{get, post, put, delete},
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
use websocket::handler::{WebSocketState, websocket_handler};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub jwt_service: JwtService,
    pub websocket: WebSocketState,
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

    // Initialize WebSocket state
    let ws_state = WebSocketState::new(jwt_service.clone(), database.clone());
    info!("WebSocket service initialized");

    // Create app state
    let app_state = AppState {
        database,
        jwt_service: jwt_service.clone(),
        websocket: ws_state.clone(),
    };

    // Build protected routes (require authentication)
    let protected_routes = Router::new()
        // User routes
        .route("/users/me", get(api::users::get_current_user))
        .route("/users/me", post(api::users::update_current_user))
        
        // Team routes
        .route("/teams", post(api::teams::create_team))
        .route("/teams", get(api::teams::get_user_teams))
        .route("/teams/:team_id", get(api::teams::get_team_details))
        .route("/teams/:team_id", put(api::teams::update_team))
        .route("/teams/:team_id", delete(api::teams::delete_team))
        .route("/teams/:team_id/members", post(api::teams::add_team_member))
        .route("/teams/:team_id/members/:user_id", delete(api::teams::remove_team_member))
        .route("/teams/:team_id/members/:user_id", put(api::teams::update_team_member_role))
        
        // Project routes
        .route("/teams/:team_id/projects", post(api::projects::create_project))
        .route("/teams/:team_id/projects", get(api::projects::get_team_projects))
        .route("/projects", get(api::projects::get_user_projects))
        .route("/projects/:project_id", get(api::projects::get_project_details))
        .route("/projects/:project_id", put(api::projects::update_project))
        .route("/projects/:project_id", delete(api::projects::delete_project))
        .route("/projects/:project_id/archive", post(api::projects::archive_project))
        .route("/projects/:project_id/activate", post(api::projects::activate_project))
        .route("/projects/:project_id/members", post(api::projects::add_project_member))
        .route("/projects/:project_id/members/:user_id", delete(api::projects::remove_project_member))
        .route("/projects/:project_id/members/:user_id", put(api::projects::update_project_member_role))
        
        // Task routes
        .route("/projects/:project_id/tasks", post(api::tasks::create_task))
        .route("/projects/:project_id/tasks", get(api::tasks::get_project_tasks))
        .route("/tasks", get(api::tasks::get_user_assigned_tasks))
        .route("/tasks/:task_id", get(api::tasks::get_task_details))
        .route("/tasks/:task_id", put(api::tasks::update_task))
        .route("/tasks/:task_id", delete(api::tasks::delete_task))
        .route("/tasks/:task_id/move", post(api::tasks::move_task))
        
        // Board routes
        .route("/projects/:project_id/boards", post(api::boards::create_board))
        .route("/projects/:project_id/boards", get(api::boards::get_project_boards))
        .route("/boards/:board_id", get(api::boards::get_board_details))
        .route("/boards/:board_id", put(api::boards::update_board))
        .route("/boards/:board_id", delete(api::boards::delete_board))
        
        // Task comment routes
        .route("/tasks/:task_id/comments", post(api::comments::create_task_comment))
        .route("/tasks/:task_id/comments", get(api::comments::get_task_comments))
        .route("/comments/:comment_id", delete(api::comments::delete_task_comment))
        
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

    // WebSocket routes
    let ws_routes = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(ws_state);

    // Combine routes
    let app = Router::new()
        .nest("/api", protected_routes)
        .merge(public_routes.clone())
        .nest("/api", public_routes)
        .merge(ws_routes)
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    info!("SimpleCards backend starting on http://{}:{}", host, port);
    info!("Health check available at http://{}:{}/health", host, port);
    info!("API documentation: http://{}:{}/api", host, port);
    info!("WebSocket endpoint available at ws://{}:{}/ws", host, port);

    // Run the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
