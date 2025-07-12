# Technical Architecture

## System Overview

The task management system follows a modern three-tier architecture with real-time capabilities:

- **Frontend**: Next.js (React) with TypeScript
- **Backend**: Rust with axum web framework and tokio async runtime
- **Database**: PostgreSQL with Redis for caching and WebSocket scaling
- **Real-time**: WebSocket connections with Redis PubSub for horizontal scaling

## Architecture Diagram

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Next.js       │    │   Rust Backend  │    │   PostgreSQL    │
│   Frontend      │◄──►│   (axum)        │◄──►│   Database      │
│                 │    │                 │    │                 │
│ - React UI      │    │ - REST API      │    │ - Primary Data  │
│ - WebSocket     │    │ - WebSocket     │    │ - ACID          │
│ - State Mgmt    │    │ - Auth          │    │ - Complex Queries│
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │     Redis       │
                       │                 │
                       │ - Session Cache │
                       │ - WebSocket     │
                       │   Scaling       │
                       │ - Rate Limiting │
                       └─────────────────┘
```

## Database Schema

### Core Tables

```sql
-- Users and Authentication
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Teams
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Team Memberships
CREATE TABLE team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role team_role NOT NULL DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(team_id, user_id)
);

-- Projects
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    color VARCHAR(7), -- hex color code
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Project Memberships and Roles
CREATE TABLE project_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role project_role NOT NULL DEFAULT 'editor',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, user_id)
);

-- Project Status Configuration
CREATE TABLE project_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    color VARCHAR(7),
    position INTEGER NOT NULL,
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, name),
    UNIQUE(project_id, position)
);

-- Epics
CREATE TABLE epics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    assigned_to UUID REFERENCES users(id),
    start_date DATE,
    end_date DATE,
    status VARCHAR(50) DEFAULT 'planned',
    color VARCHAR(7),
    is_archived BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tasks
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    epic_id UUID REFERENCES epics(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id),
    assigned_to UUID REFERENCES users(id),
    status_id UUID REFERENCES project_statuses(id),
    due_date TIMESTAMPTZ,
    position DOUBLE PRECISION, -- for manual ordering
    is_archived BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Task Labels
CREATE TABLE labels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    color VARCHAR(7) NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(project_id, name)
);

CREATE TABLE task_labels (
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    label_id UUID NOT NULL REFERENCES labels(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, label_id)
);

-- Comments
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Activity Log
CREATE TABLE activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    activity_type activity_type NOT NULL,
    old_value JSONB,
    new_value JSONB,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Board Configurations
CREATE TABLE boards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    board_type board_type NOT NULL DEFAULT 'team',
    group_by VARCHAR(50) NOT NULL DEFAULT 'status',
    filters JSONB DEFAULT '{}',
    sort_by VARCHAR(50) DEFAULT 'position',
    column_order TEXT[], -- array of column identifiers
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- File Attachments
CREATE TABLE attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    filename VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    storage_path TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Custom Types

```sql
-- Enum types
CREATE TYPE team_role AS ENUM ('admin', 'member');
CREATE TYPE project_role AS ENUM ('admin', 'member', 'editor', 'guest');
CREATE TYPE board_type AS ENUM ('team', 'personal');
CREATE TYPE activity_type AS ENUM (
    'task_created', 'task_updated', 'task_deleted', 'task_moved',
    'comment_added', 'comment_updated', 'comment_deleted',
    'assignment_changed', 'status_changed', 'due_date_changed',
    'label_added', 'label_removed', 'epic_assigned', 'epic_removed'
);
```

### Indexes

```sql
-- Performance indexes
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX idx_tasks_status_id ON tasks(status_id);
CREATE INDEX idx_tasks_epic_id ON tasks(epic_id);
CREATE INDEX idx_tasks_position ON tasks(project_id, position);
CREATE INDEX idx_comments_task_id ON comments(task_id);
CREATE INDEX idx_activities_task_id ON activities(task_id);
CREATE INDEX idx_activities_project_id ON activities(project_id);
CREATE INDEX idx_activities_created_at ON activities(created_at);
CREATE INDEX idx_project_members_user_id ON project_members(user_id);
CREATE INDEX idx_team_members_user_id ON team_members(user_id);
```

## Security Architecture

### Authentication Flow

1. **User Registration/Login**: JWT tokens with refresh token rotation
2. **Session Management**: Redis-backed session storage with configurable TTL
3. **Password Security**: Argon2id hashing with secure random salts
4. **API Authentication**: Bearer token validation middleware

### Authorization Model

```rust
// Permission checking at multiple levels
pub enum Permission {
    // Project-level permissions
    ProjectView,
    ProjectEdit,
    ProjectDelete,
    ProjectManageMembers,
    
    // Task-level permissions
    TaskCreate,
    TaskEdit,
    TaskDelete,
    TaskComment,
    
    // Board-level permissions
    BoardCreate,
    BoardEdit,
    BoardDelete,
    BoardConfigure,
}

// Role-based permission matrix
impl Permission {
    pub fn check(user_role: ProjectRole, permission: Permission) -> bool {
        match (user_role, permission) {
            (ProjectRole::Admin, _) => true,
            (ProjectRole::Member, Permission::ProjectManageMembers) => false,
            (ProjectRole::Member, Permission::TaskDelete) => false,
            (ProjectRole::Editor, Permission::TaskCreate | Permission::TaskEdit) => true,
            (ProjectRole::Guest, Permission::ProjectView | Permission::TaskView) => true,
            _ => false,
        }
    }
}
```

## Real-time Architecture

### WebSocket Event System

```rust
#[derive(Serialize, Deserialize)]
pub enum WebSocketEvent {
    // Board-level events
    TaskCreated { task: TaskSummary, board_id: Uuid },
    TaskUpdated { task: TaskSummary, board_id: Uuid },
    TaskMoved { task_id: Uuid, from_status: String, to_status: String },
    TaskDeleted { task_id: Uuid, board_id: Uuid },
    
    // Task detail events
    TaskDetailUpdated { task: TaskDetail },
    CommentAdded { comment: Comment, task_id: Uuid },
    CommentUpdated { comment: Comment, task_id: Uuid },
    CommentDeleted { comment_id: Uuid, task_id: Uuid },
    
    // Presence events
    UserJoinedBoard { user: UserSummary, board_id: Uuid },
    UserLeftBoard { user_id: Uuid, board_id: Uuid },
    UserTyping { user_id: Uuid, task_id: Uuid },
}
```

### Redis PubSub Channels

- `board:{board_id}`: Board-specific events (task movements, new tasks)
- `task:{task_id}`: Task-specific events (comments, detail changes)
- `project:{project_id}`: Project-wide events (member changes, status updates)

## Caching Strategy

### Redis Cache Layers

1. **Session Cache**: User sessions and JWT refresh tokens (TTL: 7 days)
2. **Query Cache**: Frequently accessed data (project members, status lists)
3. **Rate Limiting**: API rate limiting counters (TTL: configurable)
4. **WebSocket State**: Active connections and user presence

### Cache Invalidation

- **Write-through**: Critical data immediately updated in cache
- **Event-driven**: Cache invalidation triggered by domain events
- **TTL-based**: Non-critical data with appropriate expiration

## File Storage

### Attachment Handling

```rust
pub struct FileUploadConfig {
    max_file_size: u64,      // 10MB default
    allowed_types: Vec<String>, // MIME type whitelist
    storage_backend: StorageBackend,
    virus_scanning: bool,
}

pub enum StorageBackend {
    Local { path: PathBuf },
    S3 { bucket: String, region: String },
    MinIO { endpoint: String, bucket: String },
}
```

### Security Measures

- File type validation by MIME type and magic bytes
- Virus scanning integration (ClamAV)
- Sanitized filename generation
- Access control via signed URLs

## Performance Considerations

### Database Optimization

- **Connection Pooling**: PostgreSQL connection pool with configurable limits
- **Query Optimization**: Prepared statements and query analysis
- **Pagination**: Cursor-based pagination for large datasets
- **Read Replicas**: Optional read-only replicas for analytics

### Application Performance

- **Async Processing**: Background job queue for heavy operations
- **Compression**: Response compression for API and WebSocket
- **CDN Integration**: Static asset delivery optimization
- **Metrics Collection**: Prometheus metrics for monitoring

## Monitoring and Observability

### Logging Strategy

```rust
// Structured logging with tracing
#[tracing::instrument(skip(db))]
pub async fn create_task(
    db: &Database,
    project_id: Uuid,
    task_data: CreateTaskRequest,
) -> Result<Task, Error> {
    tracing::info!("Creating task in project {}", project_id);
    // Implementation
}
```

### Metrics Collection

- **Request Metrics**: Response times, error rates, throughput
- **Database Metrics**: Query performance, connection pool usage
- **WebSocket Metrics**: Active connections, message rates
- **Business Metrics**: User activity, feature usage patterns

### Health Checks

- Database connectivity
- Redis connectivity
- External service dependencies
- Disk space and memory usage