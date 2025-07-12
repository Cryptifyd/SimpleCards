# Data Models

This document defines all entity models, their relationships, and business rules for the task management system.

## Core Entities

### User

Represents system users with authentication and profile information.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,          // Valid email format
    pub username: String,       // 3-50 chars, alphanumeric + underscore
    pub display_name: String,   // 1-255 chars
    pub password: String,       // Min 8 chars, complexity rules
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
```

**Business Rules:**
- Email must be unique and valid format
- Username must be unique, 3-50 characters, alphanumeric plus underscore
- Display name is required, 1-255 characters
- Password minimum 8 characters with complexity requirements
- Users can be deactivated but not deleted (data integrity)

### Team

Groups users for collaboration and project organization.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,               // 1-255 chars, unique per organization
    pub description: Option<String>, // Optional, max 1000 chars
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: TeamRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamRole {
    Admin,  // Can manage team, add/remove members, create projects
    Member, // Can create projects, participate in team activities
}
```

**Business Rules:**
- Team names must be unique within the system
- Team creator automatically becomes admin
- Teams must have at least one admin
- Only admins can add/remove members and delete teams
- Members can leave teams (except last admin)

### Project

Contains tasks and defines project-specific configurations.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_id: Uuid,
    pub created_by: Uuid,
    pub color: Option<String>,      // Hex color code
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,               // 1-255 chars
    pub description: Option<String>, // Optional, max 2000 chars
    pub team_id: Uuid,
    pub color: Option<String>,       // Hex color code validation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMember {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub role: ProjectRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectRole {
    Admin,   // Full project control, manage members, configure boards
    Member,  // Create/edit tasks, manage team boards, edit project settings
    Editor,  // Create/edit tasks, create personal boards only
    Guest,   // Read-only access to project content
}
```

**Business Rules:**
- Project names must be unique within a team
- Users must be team members to be added to projects
- Project creator gets admin role automatically
- Projects must have at least one admin
- Color must be valid hex code if provided
- Archived projects are read-only but not deleted

### ProjectStatus

Defines available task statuses for a project.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatus {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub color: Option<String>,      // Hex color code
    pub position: i32,              // Display order
    pub is_default: bool,           // Default status for new tasks
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStatusRequest {
    pub name: String,               // 1-100 chars, unique per project
    pub color: Option<String>,       // Hex color code
    pub position: Option<i32>,       // Auto-assigned if not provided
    pub is_default: Option<bool>,
}
```

**Business Rules:**
- Status names must be unique within a project
- Each project must have at least one status
- Only one status can be marked as default per project
- Position determines display order (auto-assigned if not specified)
- Cannot delete status if tasks are using it

### Epic

High-level containers for grouping related tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epic {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub assigned_to: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: EpicStatus,
    pub color: Option<String>,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpicStatus {
    Planned,
    InProgress,
    Completed,
    OnHold,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEpicRequest {
    pub title: String,              // 1-255 chars
    pub description: Option<String>, // Optional, max 5000 chars
    pub assigned_to: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub color: Option<String>,       // Hex color code
}
```

**Business Rules:**
- Epic titles must be unique within a project
- End date must be after start date if both provided
- Assigned user must be project member
- Archiving an epic doesn't affect associated tasks
- Color must be valid hex code if provided

### Task

Primary work items in the system.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub epic_id: Option<Uuid>,
    pub created_by: Uuid,
    pub assigned_to: Option<Uuid>,
    pub status_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub position: f64,              // For manual ordering
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,              // 1-255 chars
    pub description: Option<String>, // Optional, max 10000 chars
    pub epic_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub status_id: Option<Uuid>,    // Uses project default if not provided
    pub due_date: Option<DateTime<Utc>>,
    pub position: Option<f64>,       // Auto-assigned if not provided
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub epic_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub status_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub position: Option<f64>,
}
```

**Business Rules:**
- Task titles are required and can be duplicated
- Assigned user must be project member
- Epic must belong to same project if specified
- Status must belong to same project if specified
- Position used for manual ordering within status columns
- Due date can be in the past (overdue indication)
- Archived tasks are read-only but visible in history

### Label

Categorization tags for tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: Uuid,
    pub name: String,
    pub color: String,              // Hex color code, required
    pub project_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLabelRequest {
    pub name: String,               // 1-100 chars, unique per project
    pub color: String,              // Required hex color code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLabel {
    pub task_id: Uuid,
    pub label_id: Uuid,
}
```

**Business Rules:**
- Label names must be unique within a project
- Color is required and must be valid hex code
- Tasks can have multiple labels
- Deleting a label removes it from all tasks

### Comment

User comments on tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub is_edited: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,            // 1-5000 chars, supports markdown
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,            // 1-5000 chars
}
```

**Business Rules:**
- Comments cannot be empty
- Users can only edit/delete their own comments (except project admins)
- Supports basic Markdown formatting
- Comments are soft-deleted (marked as deleted but preserved)
- Edit history is tracked via `is_edited` flag and `updated_at`

### Board

Configurable views for displaying tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub board_type: BoardType,
    pub group_by: GroupBy,
    pub filters: BoardFilters,
    pub sort_by: SortBy,
    pub column_order: Vec<String>,   // Order of columns/groups
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardType {
    Team,       // Visible to all project members
    Personal,   // Visible only to creator
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupBy {
    Status,
    Assignee,
    Label,
    Epic,
    DueDate,
    None,       // List view
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Position,   // Manual ordering
    CreatedAt,
    UpdatedAt,
    DueDate,
    Title,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardFilters {
    pub assigned_to: Option<Vec<Uuid>>,
    pub labels: Option<Vec<Uuid>>,
    pub epics: Option<Vec<Uuid>>,
    pub statuses: Option<Vec<Uuid>>,
    pub due_date_range: Option<DateRange>,
    pub created_date_range: Option<DateRange>,
    pub search_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
```

**Business Rules:**
- Board names must be unique per user per project
- Personal boards are only visible to creator
- Team boards require member+ permissions to create
- Only one board can be marked as default per project
- Filters are applied client-side for performance
- Column order preserves user preferences

### Activity

Audit trail for task and project changes.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    TaskCreated,
    TaskUpdated,
    TaskDeleted,
    TaskMoved,
    CommentAdded,
    CommentUpdated,
    CommentDeleted,
    AssignmentChanged,
    StatusChanged,
    DueDateChanged,
    LabelAdded,
    LabelRemoved,
    EpicAssigned,
    EpicRemoved,
}
```

**Business Rules:**
- Activities are immutable once created
- System automatically generates activities for tracked changes
- Old/new values stored as JSON for flexibility
- Activities are used for audit trails and user notifications
- Retention policy may apply to old activities

### Attachment

File attachments for tasks.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub uploaded_by: Uuid,
    pub filename: String,           // Generated safe filename
    pub original_name: String,      // User's original filename
    pub file_size: i64,            // Bytes
    pub mime_type: String,
    pub storage_path: String,       // Internal storage reference
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAttachmentRequest {
    pub original_name: String,
    pub file_size: i64,
    pub mime_type: String,
}
```

**Business Rules:**
- Maximum file size: 10MB per file
- Allowed MIME types: configurable whitelist
- Filenames are sanitized and made unique
- Virus scanning required before storage
- Files are logically deleted when tasks are deleted
- Access controlled via task permissions

## Relationships and Constraints

### Entity Relationships

```
Users ──┐
        ├── Teams (created_by)
        ├── Projects (created_by)
        ├── Tasks (created_by, assigned_to)
        ├── Epics (created_by, assigned_to)
        ├── Comments (user_id)
        ├── Boards (created_by)
        ├── Activities (user_id)
        └── Attachments (uploaded_by)

Teams ──┐
        ├── TeamMembers (team_id)
        └── Projects (team_id)

Projects ──┐
           ├── ProjectMembers (project_id)
           ├── ProjectStatuses (project_id)
           ├── Tasks (project_id)
           ├── Epics (project_id)
           ├── Labels (project_id)
           └── Boards (project_id)

Tasks ──┐
        ├── TaskLabels (task_id)
        ├── Comments (task_id)
        ├── Activities (task_id)
        └── Attachments (task_id)
```

### Cascade Rules

- **Team deletion**: Cascades to projects, tasks, and all related data
- **Project deletion**: Cascades to tasks, epics, boards, and project-specific data
- **User deletion**: Prevented if user has created content; use deactivation instead
- **Epic deletion**: Sets task.epic_id to NULL (doesn't delete tasks)
- **Status deletion**: Prevented if tasks are using the status

## Validation Rules

### Common Patterns

```rust
// Email validation
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.len() > 255 || !email.contains('@') {
        return Err(ValidationError::InvalidEmail);
    }
    Ok(())
}

// Hex color validation
pub fn validate_hex_color(color: &str) -> Result<(), ValidationError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ValidationError::InvalidColor);
    }
    Ok(())
}

// Text length validation
pub fn validate_text_length(text: &str, min: usize, max: usize) -> Result<(), ValidationError> {
    if text.len() < min || text.len() > max {
        return Err(ValidationError::InvalidLength { min, max });
    }
    Ok(())
}
```

### Business Logic Validation

- Permission checks before any data modification
- Cross-entity validation (e.g., assignee is project member)
- Date consistency checks (start_date < end_date)
- Uniqueness constraints within appropriate scopes
- File upload safety checks (MIME type, size, virus scanning)