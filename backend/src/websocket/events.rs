use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::database::models::{Task, TaskStatus, TaskPriority, Board, TaskComment, UserSummary};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketEvent {
    // Authentication events
    Authenticate { token: String },
    AuthenticationSuccess { user_id: Uuid },
    AuthenticationError { message: String },

    // Subscription events
    Subscribe { project_id: Uuid },
    Unsubscribe { project_id: Uuid },
    SubscriptionSuccess { project_id: Uuid },
    SubscriptionError { message: String },

    // Task events
    TaskCreated(TaskEventData),
    TaskUpdated(TaskEventData),
    TaskDeleted { task_id: Uuid, project_id: Uuid },
    TaskMoved(TaskMoveEventData),

    // Board events
    BoardCreated(BoardEventData),
    BoardUpdated(BoardEventData),
    BoardDeleted { board_id: Uuid, project_id: Uuid },

    // Comment events
    CommentCreated(CommentEventData),
    CommentDeleted { comment_id: Uuid, task_id: Uuid, project_id: Uuid },

    // User presence events
    UserJoined(UserPresenceData),
    UserLeft(UserPresenceData),
    UserTyping(TypingEventData),
    UserStoppedTyping(TypingEventData),

    // Error events
    Error { message: String },
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEventData {
    pub task: Task,
    pub project_id: Uuid,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMoveEventData {
    pub task_id: Uuid,
    pub from_status: TaskStatus,
    pub to_status: TaskStatus,
    pub position: i32,
    pub project_id: Uuid,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardEventData {
    pub board: Board,
    pub project_id: Uuid,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEventData {
    pub comment: TaskComment,
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub user: UserSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPresenceData {
    pub user: UserSummary,
    pub project_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingEventData {
    pub user: UserSummary,
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

// Heartbeat events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeartbeatEvent {
    Ping,
    Pong,
}

// Connection state
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub user_id: Uuid,
    pub subscribed_projects: std::collections::HashSet<Uuid>,
    pub last_seen: DateTime<Utc>,
}

impl ConnectionInfo {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            subscribed_projects: std::collections::HashSet::new(),
            last_seen: Utc::now(),
        }
    }

    pub fn subscribe_to_project(&mut self, project_id: Uuid) {
        self.subscribed_projects.insert(project_id);
        self.last_seen = Utc::now();
    }

    pub fn unsubscribe_from_project(&mut self, project_id: Uuid) {
        self.subscribed_projects.remove(&project_id);
        self.last_seen = Utc::now();
    }

    pub fn is_subscribed_to(&self, project_id: Uuid) -> bool {
        self.subscribed_projects.contains(&project_id)
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
}