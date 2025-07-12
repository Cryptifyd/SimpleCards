use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade, Query},
    response::Response,
    http::StatusCode,
};
use axum::extract::ws::Message;
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock, Mutex};
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use chrono::Utc;

use crate::auth::jwt::JwtService;
use crate::database::{
    models::UserSummary,
    queries::{ProjectQueries, UserQueries}
};
use crate::utils::errors::AppError;
use super::events::{WebSocketEvent, ConnectionInfo, HeartbeatEvent};

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    token: Option<String>,
}

// Global connection manager
pub type ConnectionManager = Arc<RwLock<HashMap<Uuid, broadcast::Sender<WebSocketEvent>>>>;
pub type UserConnectionsManager = Arc<RwLock<HashMap<Uuid, ConnectionInfo>>>;

#[derive(Clone)]
pub struct WebSocketState {
    pub connections: ConnectionManager,
    pub user_connections: UserConnectionsManager,
    pub jwt_service: JwtService,
    pub database: crate::database::connection::Database,
}

impl WebSocketState {
    pub fn new(jwt_service: JwtService, database: crate::database::connection::Database) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            user_connections: Arc::new(RwLock::new(HashMap::new())),
            jwt_service,
            database,
        }
    }

    // Broadcast event to all users subscribed to a project
    pub async fn broadcast_to_project(&self, project_id: Uuid, event: WebSocketEvent, exclude_user: Option<Uuid>) {
        let user_connections = self.user_connections.read().await;
        let connections = self.connections.read().await;

        for (user_id, conn_info) in user_connections.iter() {
            if let Some(exclude) = exclude_user {
                if *user_id == exclude {
                    continue;
                }
            }

            if conn_info.is_subscribed_to(project_id) {
                if let Some(sender) = connections.get(user_id) {
                    if let Err(e) = sender.send(event.clone()) {
                        warn!("Failed to send message to user {}: {}", user_id, e);
                    }
                }
            }
        }
    }

    // Send event to specific user
    pub async fn send_to_user(&self, user_id: Uuid, event: WebSocketEvent) {
        let connections = self.connections.read().await;
        if let Some(sender) = connections.get(&user_id) {
            if let Err(e) = sender.send(event) {
                warn!("Failed to send message to user {}: {}", user_id, e);
            }
        }
    }

    // Register new connection
    pub async fn register_connection(&self, user_id: Uuid) -> broadcast::Receiver<WebSocketEvent> {
        let (tx, rx) = broadcast::channel(1000);
        
        {
            let mut connections = self.connections.write().await;
            connections.insert(user_id, tx);
        }

        {
            let mut user_connections = self.user_connections.write().await;
            user_connections.insert(user_id, ConnectionInfo::new(user_id));
        }

        info!("User {} connected to WebSocket", user_id);
        rx
    }

    // Unregister connection
    pub async fn unregister_connection(&self, user_id: Uuid) {
        {
            let mut connections = self.connections.write().await;
            connections.remove(&user_id);
        }

        // Notify other users that this user left
        if let Some(conn_info) = {
            let mut user_connections = self.user_connections.write().await;
            user_connections.remove(&user_id)
        } {
            // Get user info for presence notifications
            if let Ok(user) = UserQueries::get_user_by_id(self.database.pool(), user_id).await {
                let user_summary: UserSummary = user.into();
                
                // Notify all subscribed projects that user left
                for project_id in conn_info.subscribed_projects {
                    let presence_event = WebSocketEvent::UserLeft(super::events::UserPresenceData {
                        user: user_summary.clone(),
                        project_id,
                        timestamp: Utc::now(),
                    });
                    
                    self.broadcast_to_project(project_id, presence_event, Some(user_id)).await;
                }
            }
        }

        info!("User {} disconnected from WebSocket", user_id);
    }

    // Subscribe user to project updates
    pub async fn subscribe_to_project(&self, user_id: Uuid, project_id: Uuid) -> Result<(), AppError> {
        // Check if user has access to this project
        if !ProjectQueries::is_project_member(self.database.pool(), project_id, user_id).await? {
            return Err(AppError::Forbidden("Not a project member".to_string()));
        }

        {
            let mut user_connections = self.user_connections.write().await;
            if let Some(conn_info) = user_connections.get_mut(&user_id) {
                conn_info.subscribe_to_project(project_id);
            }
        }

        // Notify other users that this user joined
        if let Ok(user) = UserQueries::get_user_by_id(self.database.pool(), user_id).await {
            let user_summary: UserSummary = user.into();
            let presence_event = WebSocketEvent::UserJoined(super::events::UserPresenceData {
                user: user_summary,
                project_id,
                timestamp: Utc::now(),
            });
            
            self.broadcast_to_project(project_id, presence_event, Some(user_id)).await;
        }

        // Send subscription success
        self.send_to_user(user_id, WebSocketEvent::SubscriptionSuccess { project_id }).await;
        
        debug!("User {} subscribed to project {}", user_id, project_id);
        Ok(())
    }

    // Unsubscribe user from project updates
    pub async fn unsubscribe_from_project(&self, user_id: Uuid, project_id: Uuid) {
        {
            let mut user_connections = self.user_connections.write().await;
            if let Some(conn_info) = user_connections.get_mut(&user_id) {
                conn_info.unsubscribe_from_project(project_id);
            }
        }

        // Notify other users that this user left the project
        if let Ok(user) = UserQueries::get_user_by_id(self.database.pool(), user_id).await {
            let user_summary: UserSummary = user.into();
            let presence_event = WebSocketEvent::UserLeft(super::events::UserPresenceData {
                user: user_summary,
                project_id,
                timestamp: Utc::now(),
            });
            
            self.broadcast_to_project(project_id, presence_event, Some(user_id)).await;
        }

        debug!("User {} unsubscribed from project {}", user_id, project_id);
    }
}

// WebSocket upgrade handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(ws_state): State<WebSocketState>,
    Query(params): Query<WebSocketQuery>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, ws_state, params.token))
}

// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, ws_state: WebSocketState, token: Option<String>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Authentication
    let user_id = match authenticate_connection(&token, &ws_state.jwt_service).await {
        Ok(user_id) => {
            let auth_success = WebSocketEvent::AuthenticationSuccess { user_id };
            if let Err(e) = sender.send(Message::Text(serde_json::to_string(&auth_success).unwrap())).await {
                error!("Failed to send auth success: {}", e);
                return;
            }
            user_id
        }
        Err(e) => {
            let auth_error = WebSocketEvent::AuthenticationError { 
                message: e.to_string() 
            };
            if let Err(send_err) = sender.send(Message::Text(serde_json::to_string(&auth_error).unwrap())).await {
                error!("Failed to send auth error: {}", send_err);
            }
            return;
        }
    };

    // Register connection
    let mut event_rx = ws_state.register_connection(user_id).await;
    
    // Spawn task to handle outgoing messages
    let ws_state_clone = ws_state.clone();
    let sender_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let message = match serde_json::to_string(&event) {
                Ok(msg) => Message::Text(msg),
                Err(e) => {
                    error!("Failed to serialize event: {}", e);
                    continue;
                }
            };
            
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let ws_state_clone2 = ws_state.clone();
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if handle_message(msg, user_id, &ws_state_clone2).await.is_err() {
                break;
            }
        } else {
            break;
        }
    }

    // Cleanup
    sender_task.abort();
    ws_state.unregister_connection(user_id).await;
}

// Authenticate WebSocket connection
async fn authenticate_connection(token: &Option<String>, jwt_service: &JwtService) -> Result<Uuid, AppError> {
    let token = token.as_ref().ok_or_else(|| AppError::Unauthorized("No token provided".to_string()))?;
    
    let claims = jwt_service.verify_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;
    
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;
    
    Ok(user_id)
}

// Handle incoming WebSocket messages
async fn handle_message(msg: Message, user_id: Uuid, ws_state: &WebSocketState) -> Result<(), AppError> {
    match msg {
        Message::Text(text) => {
            let event: WebSocketEvent = serde_json::from_str(&text)
                .map_err(|e| AppError::BadRequest(format!("Invalid message format: {}", e)))?;
            
            handle_event(event, user_id, ws_state).await
        }
        Message::Close(_) => {
            debug!("WebSocket connection closed by user {}", user_id);
            Err(AppError::BadRequest("Connection closed".to_string()))
        }
        Message::Ping(data) => {
            // Echo back as pong - axum handles this automatically
            Ok(())
        }
        Message::Pong(_) => {
            // Update last seen
            let mut user_connections = ws_state.user_connections.write().await;
            if let Some(conn_info) = user_connections.get_mut(&user_id) {
                conn_info.update_last_seen();
            }
            Ok(())
        }
        _ => {
            warn!("Unsupported message type from user {}", user_id);
            Ok(())
        }
    }
}

// Handle specific WebSocket events
async fn handle_event(event: WebSocketEvent, user_id: Uuid, ws_state: &WebSocketState) -> Result<(), AppError> {
    match event {
        WebSocketEvent::Subscribe { project_id } => {
            ws_state.subscribe_to_project(user_id, project_id).await?;
        }
        WebSocketEvent::Unsubscribe { project_id } => {
            ws_state.unsubscribe_from_project(user_id, project_id).await;
        }
        WebSocketEvent::UserTyping(typing_data) => {
            // Broadcast typing indicator to other users in the project
            ws_state.broadcast_to_project(
                typing_data.project_id, 
                WebSocketEvent::UserTyping(typing_data),
                Some(user_id)
            ).await;
        }
        WebSocketEvent::UserStoppedTyping(typing_data) => {
            // Broadcast stop typing indicator to other users in the project
            ws_state.broadcast_to_project(
                typing_data.project_id, 
                WebSocketEvent::UserStoppedTyping(typing_data),
                Some(user_id)
            ).await;
        }
        WebSocketEvent::Pong => {
            // Handle pong response to keep connection alive
            let mut user_connections = ws_state.user_connections.write().await;
            if let Some(conn_info) = user_connections.get_mut(&user_id) {
                conn_info.update_last_seen();
            }
        }
        _ => {
            // Other events are only sent from server to client
            warn!("Unexpected event from client: {:?}", event);
        }
    }
    
    Ok(())
}