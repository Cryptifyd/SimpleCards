# API Specification

## Overview

The API follows RESTful principles with additional WebSocket endpoints for real-time functionality. All endpoints require authentication except for registration and login.

## Authentication

### Authentication Flow

```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "username",
  "display_name": "Display Name",
  "password": "securepassword"
}

Response 201:
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "username",
    "display_name": "Display Name",
    "avatar_url": null,
    "created_at": "2024-01-01T00:00:00Z"
  },
  "access_token": "jwt_token",
  "refresh_token": "refresh_token",
  "expires_in": 3600
}
```

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}

Response 200: Same as register
```

```http
POST /api/auth/refresh
Content-Type: application/json

{
  "refresh_token": "refresh_token"
}

Response 200:
{
  "access_token": "new_jwt_token",
  "expires_in": 3600
}
```

```http
POST /api/auth/logout
Authorization: Bearer jwt_token

Response 204: No Content
```

### Authorization Header

All authenticated endpoints require:
```http
Authorization: Bearer {jwt_token}
```

## Users API

### Get Current User

```http
GET /api/users/me
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "username",
  "display_name": "Display Name",
  "avatar_url": "https://example.com/avatar.jpg",
  "is_active": true,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Update Current User

```http
PATCH /api/users/me
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "display_name": "New Display Name",
  "avatar_url": "https://example.com/new-avatar.jpg"
}

Response 200: Updated user object
```

### Search Users

```http
GET /api/users/search?q=john&limit=10
Authorization: Bearer jwt_token

Response 200:
{
  "users": [
    {
      "id": "uuid",
      "username": "john_doe",
      "display_name": "John Doe",
      "avatar_url": "https://example.com/avatar.jpg"
    }
  ],
  "total": 1
}
```

## Teams API

### List Teams

```http
GET /api/teams
Authorization: Bearer jwt_token

Response 200:
{
  "teams": [
    {
      "id": "uuid",
      "name": "Development Team",
      "description": "Main development team",
      "created_by": "uuid",
      "created_at": "2024-01-01T00:00:00Z",
      "role": "admin",
      "member_count": 5
    }
  ]
}
```

### Create Team

```http
POST /api/teams
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "New Team",
  "description": "Team description"
}

Response 201:
{
  "id": "uuid",
  "name": "New Team",
  "description": "Team description",
  "created_by": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Get Team Details

```http
GET /api/teams/{team_id}
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "name": "Development Team",
  "description": "Main development team",
  "created_by": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "members": [
    {
      "id": "uuid",
      "user": {
        "id": "uuid",
        "username": "john_doe",
        "display_name": "John Doe",
        "avatar_url": "https://example.com/avatar.jpg"
      },
      "role": "admin",
      "joined_at": "2024-01-01T00:00:00Z"
    }
  ],
  "projects": [
    {
      "id": "uuid",
      "name": "Task Manager",
      "description": "Project description",
      "color": "#3B82F6"
    }
  ]
}
```

### Add Team Member

```http
POST /api/teams/{team_id}/members
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "user_id": "uuid",
  "role": "member"
}

Response 201:
{
  "id": "uuid",
  "user": { /* user object */ },
  "role": "member",
  "joined_at": "2024-01-01T00:00:00Z"
}
```

### Update Team Member Role

```http
PATCH /api/teams/{team_id}/members/{member_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "role": "admin"
}

Response 200: Updated member object
```

### Remove Team Member

```http
DELETE /api/teams/{team_id}/members/{member_id}
Authorization: Bearer jwt_token

Response 204: No Content
```

## Projects API

### List Projects

```http
GET /api/projects?team_id=uuid&limit=20&offset=0
Authorization: Bearer jwt_token

Response 200:
{
  "projects": [
    {
      "id": "uuid",
      "name": "Task Manager",
      "description": "Main project",
      "team_id": "uuid",
      "created_by": "uuid",
      "color": "#3B82F6",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z",
      "role": "admin",
      "task_count": 15,
      "last_activity": "2024-01-02T10:30:00Z"
    }
  ],
  "total": 1
}
```

### Create Project

```http
POST /api/projects
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "New Project",
  "description": "Project description",
  "team_id": "uuid",
  "color": "#10B981"
}

Response 201: Project object
```

### Get Project Details

```http
GET /api/projects/{project_id}
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "name": "Task Manager",
  "description": "Main project",
  "team": {
    "id": "uuid",
    "name": "Development Team"
  },
  "created_by": "uuid",
  "color": "#3B82F6",
  "is_active": true,
  "created_at": "2024-01-01T00:00:00Z",
  "members": [
    {
      "id": "uuid",
      "user": { /* user object */ },
      "role": "admin",
      "joined_at": "2024-01-01T00:00:00Z"
    }
  ],
  "statuses": [
    {
      "id": "uuid",
      "name": "To Do",
      "color": "#6B7280",
      "position": 0,
      "is_default": true
    }
  ]
}
```

### Update Project

```http
PATCH /api/projects/{project_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "Updated Project Name",
  "description": "Updated description",
  "color": "#EF4444"
}

Response 200: Updated project object
```

### Project Status Management

```http
POST /api/projects/{project_id}/statuses
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "In Review",
  "color": "#F59E0B",
  "position": 2
}

Response 201: Status object
```

```http
PATCH /api/projects/{project_id}/statuses/{status_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "Code Review",
  "color": "#8B5CF6"
}

Response 200: Updated status object
```

```http
DELETE /api/projects/{project_id}/statuses/{status_id}
Authorization: Bearer jwt_token

Response 204: No Content
Error 409: Status is in use by tasks
```

## Tasks API

### List Tasks

```http
GET /api/projects/{project_id}/tasks?status_id=uuid&assigned_to=uuid&epic_id=uuid&limit=50&offset=0
Authorization: Bearer jwt_token

Response 200:
{
  "tasks": [
    {
      "id": "uuid",
      "title": "Implement user authentication",
      "description": "Add JWT-based authentication",
      "project_id": "uuid",
      "epic_id": "uuid",
      "created_by": "uuid",
      "assigned_to": "uuid",
      "status": {
        "id": "uuid",
        "name": "In Progress",
        "color": "#F59E0B"
      },
      "due_date": "2024-01-15T00:00:00Z",
      "position": 1024.5,
      "is_archived": false,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-02T10:30:00Z",
      "labels": [
        {
          "id": "uuid",
          "name": "Backend",
          "color": "#3B82F6"
        }
      ],
      "comment_count": 3,
      "attachment_count": 1
    }
  ],
  "total": 1
}
```

### Create Task

```http
POST /api/projects/{project_id}/tasks
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "title": "New task",
  "description": "Task description",
  "epic_id": "uuid",
  "assigned_to": "uuid",
  "status_id": "uuid",
  "due_date": "2024-01-15T00:00:00Z",
  "position": 2048.0
}

Response 201: Task object
```

### Get Task Details

```http
GET /api/tasks/{task_id}
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "title": "Implement user authentication",
  "description": "Add JWT-based authentication system with...",
  "project": {
    "id": "uuid",
    "name": "Task Manager"
  },
  "epic": {
    "id": "uuid",
    "title": "Authentication System"
  },
  "created_by": {
    "id": "uuid",
    "username": "john_doe",
    "display_name": "John Doe"
  },
  "assigned_to": {
    "id": "uuid",
    "username": "jane_doe",
    "display_name": "Jane Doe"
  },
  "status": {
    "id": "uuid",
    "name": "In Progress",
    "color": "#F59E0B"
  },
  "due_date": "2024-01-15T00:00:00Z",
  "position": 1024.5,
  "is_archived": false,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-02T10:30:00Z",
  "labels": [ /* label objects */ ],
  "comments": [ /* comment objects */ ],
  "attachments": [ /* attachment objects */ ],
  "activities": [ /* activity objects */ ]
}
```

### Update Task

```http
PATCH /api/tasks/{task_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "title": "Updated task title",
  "description": "Updated description",
  "assigned_to": "uuid",
  "status_id": "uuid",
  "due_date": "2024-01-20T00:00:00Z",
  "position": 512.25
}

Response 200: Updated task object
```

### Archive/Unarchive Task

```http
PATCH /api/tasks/{task_id}/archive
Authorization: Bearer jwt_token

Response 200: Updated task object
```

```http
PATCH /api/tasks/{task_id}/unarchive
Authorization: Bearer jwt_token

Response 200: Updated task object
```

### Delete Task

```http
DELETE /api/tasks/{task_id}
Authorization: Bearer jwt_token

Response 204: No Content
```

## Labels API

### List Project Labels

```http
GET /api/projects/{project_id}/labels
Authorization: Bearer jwt_token

Response 200:
{
  "labels": [
    {
      "id": "uuid",
      "name": "Backend",
      "color": "#3B82F6",
      "project_id": "uuid",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### Create Label

```http
POST /api/projects/{project_id}/labels
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "Frontend",
  "color": "#10B981"
}

Response 201: Label object
```

### Add Label to Task

```http
POST /api/tasks/{task_id}/labels
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "label_id": "uuid"
}

Response 201: No Content
```

### Remove Label from Task

```http
DELETE /api/tasks/{task_id}/labels/{label_id}
Authorization: Bearer jwt_token

Response 204: No Content
```

## Comments API

### List Task Comments

```http
GET /api/tasks/{task_id}/comments?limit=20&offset=0
Authorization: Bearer jwt_token

Response 200:
{
  "comments": [
    {
      "id": "uuid",
      "task_id": "uuid",
      "user": {
        "id": "uuid",
        "username": "john_doe",
        "display_name": "John Doe",
        "avatar_url": "https://example.com/avatar.jpg"
      },
      "content": "This looks good to me!",
      "is_edited": false,
      "created_at": "2024-01-02T10:30:00Z",
      "updated_at": "2024-01-02T10:30:00Z"
    }
  ],
  "total": 1
}
```

### Create Comment

```http
POST /api/tasks/{task_id}/comments
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "content": "Great work on this task!"
}

Response 201: Comment object
```

### Update Comment

```http
PATCH /api/comments/{comment_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "content": "Updated comment content"
}

Response 200: Updated comment object
```

### Delete Comment

```http
DELETE /api/comments/{comment_id}
Authorization: Bearer jwt_token

Response 204: No Content
```

## Epics API

### List Project Epics

```http
GET /api/projects/{project_id}/epics?status=in_progress&limit=20&offset=0
Authorization: Bearer jwt_token

Response 200:
{
  "epics": [
    {
      "id": "uuid",
      "title": "Authentication System",
      "description": "Implement complete authentication",
      "project_id": "uuid",
      "created_by": "uuid",
      "assigned_to": "uuid",
      "start_date": "2024-01-01",
      "end_date": "2024-01-31",
      "status": "in_progress",
      "color": "#8B5CF6",
      "is_archived": false,
      "created_at": "2024-01-01T00:00:00Z",
      "task_count": 8,
      "completed_tasks": 3
    }
  ],
  "total": 1
}
```

### Create Epic

```http
POST /api/projects/{project_id}/epics
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "title": "User Dashboard",
  "description": "Create comprehensive user dashboard",
  "assigned_to": "uuid",
  "start_date": "2024-02-01",
  "end_date": "2024-02-28",
  "color": "#EF4444"
}

Response 201: Epic object
```

### Get Epic Details

```http
GET /api/epics/{epic_id}
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "title": "Authentication System",
  "description": "Implement complete authentication system...",
  "project": {
    "id": "uuid",
    "name": "Task Manager"
  },
  "created_by": { /* user object */ },
  "assigned_to": { /* user object */ },
  "start_date": "2024-01-01",
  "end_date": "2024-01-31",
  "status": "in_progress",
  "color": "#8B5CF6",
  "is_archived": false,
  "created_at": "2024-01-01T00:00:00Z",
  "tasks": [ /* task objects */ ]
}
```

### Update Epic

```http
PATCH /api/epics/{epic_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "title": "Updated Epic Title",
  "status": "completed",
  "end_date": "2024-01-25"
}

Response 200: Updated epic object
```

## Boards API

### List Project Boards

```http
GET /api/projects/{project_id}/boards
Authorization: Bearer jwt_token

Response 200:
{
  "boards": [
    {
      "id": "uuid",
      "name": "Main Kanban",
      "project_id": "uuid",
      "created_by": "uuid",
      "board_type": "team",
      "group_by": "status",
      "sort_by": "position",
      "is_default": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "uuid",
      "name": "My Tasks",
      "project_id": "uuid",
      "created_by": "uuid",
      "board_type": "personal",
      "group_by": "assignee",
      "sort_by": "due_date",
      "is_default": false,
      "created_at": "2024-01-02T00:00:00Z"
    }
  ]
}
```

### Create Board

```http
POST /api/projects/{project_id}/boards
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "Sprint Board",
  "board_type": "team",
  "group_by": "status",
  "sort_by": "position",
  "filters": {
    "labels": ["uuid1", "uuid2"],
    "assigned_to": ["uuid3"]
  },
  "column_order": ["todo", "in_progress", "done"]
}

Response 201: Board object
```

### Get Board Details

```http
GET /api/boards/{board_id}
Authorization: Bearer jwt_token

Response 200:
{
  "id": "uuid",
  "name": "Main Kanban",
  "project_id": "uuid",
  "created_by": "uuid",
  "board_type": "team",
  "group_by": "status",
  "filters": {
    "assigned_to": ["uuid1"],
    "labels": ["uuid2"],
    "due_date_range": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-31T23:59:59Z"
    }
  },
  "sort_by": "position",
  "column_order": ["todo", "in_progress", "review", "done"],
  "is_default": true,
  "created_at": "2024-01-01T00:00:00Z",
  "columns": [
    {
      "id": "todo",
      "name": "To Do",
      "color": "#6B7280",
      "tasks": [ /* filtered task objects */ ]
    },
    {
      "id": "in_progress",
      "name": "In Progress",
      "color": "#F59E0B",
      "tasks": [ /* filtered task objects */ ]
    }
  ]
}
```

### Update Board

```http
PATCH /api/boards/{board_id}
Authorization: Bearer jwt_token
Content-Type: application/json

{
  "name": "Updated Board Name",
  "filters": {
    "assigned_to": ["uuid1", "uuid2"]
  },
  "column_order": ["done", "in_progress", "todo"]
}

Response 200: Updated board object
```

## File Attachments API

### Upload Attachment

```http
POST /api/tasks/{task_id}/attachments
Authorization: Bearer jwt_token
Content-Type: multipart/form-data

file: [binary file data]
filename: document.pdf

Response 201:
{
  "id": "uuid",
  "task_id": "uuid",
  "uploaded_by": "uuid",
  "filename": "document_abc123.pdf",
  "original_name": "document.pdf",
  "file_size": 1048576,
  "mime_type": "application/pdf",
  "created_at": "2024-01-02T10:30:00Z",
  "download_url": "/api/attachments/uuid/download"
}
```

### Download Attachment

```http
GET /api/attachments/{attachment_id}/download
Authorization: Bearer jwt_token

Response 200:
Content-Type: [original mime type]
Content-Disposition: attachment; filename="document.pdf"
[binary file data]
```

### Delete Attachment

```http
DELETE /api/attachments/{attachment_id}
Authorization: Bearer jwt_token

Response 204: No Content
```

## Activities API

### List Task Activities

```http
GET /api/tasks/{task_id}/activities?limit=20&offset=0
Authorization: Bearer jwt_token

Response 200:
{
  "activities": [
    {
      "id": "uuid",
      "task_id": "uuid",
      "user": { /* user object */ },
      "activity_type": "status_changed",
      "old_value": {
        "status_id": "uuid1",
        "status_name": "To Do"
      },
      "new_value": {
        "status_id": "uuid2",
        "status_name": "In Progress"
      },
      "created_at": "2024-01-02T10:30:00Z"
    }
  ],
  "total": 1
}
```

## WebSocket API

### Connection

Connect to WebSocket at `/ws` with authentication:

```javascript
const ws = new WebSocket('wss://api.example.com/ws', [], {
  headers: {
    'Authorization': 'Bearer jwt_token'
  }
});
```

### Message Format

All WebSocket messages follow this format:

```typescript
interface WebSocketMessage {
  type: string;
  payload: any;
  timestamp: string;
  user_id?: string;
}
```

### Subscribe to Channels

```json
{
  "type": "subscribe",
  "payload": {
    "channels": [
      "board:uuid",
      "task:uuid",
      "project:uuid"
    ]
  }
}
```

### Unsubscribe from Channels

```json
{
  "type": "unsubscribe",
  "payload": {
    "channels": ["board:uuid"]
  }
}
```

### Event Types

#### Board Events

```json
{
  "type": "task_created",
  "payload": {
    "task": { /* task object */ },
    "board_id": "uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

```json
{
  "type": "task_moved",
  "payload": {
    "task_id": "uuid",
    "from_status": "todo",
    "to_status": "in_progress",
    "position": 1024.5
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

```json
{
  "type": "task_updated",
  "payload": {
    "task": { /* updated task object */ },
    "board_id": "uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

```json
{
  "type": "task_deleted",
  "payload": {
    "task_id": "uuid",
    "board_id": "uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

#### Task Detail Events

```json
{
  "type": "comment_added",
  "payload": {
    "comment": { /* comment object */ },
    "task_id": "uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

```json
{
  "type": "task_detail_updated",
  "payload": {
    "task": { /* full task object */ }
  },
  "timestamp": "2024-01-02T10:30:00Z",
  "user_id": "uuid"
}
```

#### Presence Events

```json
{
  "type": "user_joined_board",
  "payload": {
    "user": { /* user object */ },
    "board_id": "uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z"
}
```

```json
{
  "type": "user_typing",
  "payload": {
    "user_id": "uuid",
    "task_id": "uuid",
    "is_typing": true
  },
  "timestamp": "2024-01-02T10:30:00Z"
}
```

### Client Heartbeat

```json
{
  "type": "ping",
  "payload": {},
  "timestamp": "2024-01-02T10:30:00Z"
}
```

Server response:
```json
{
  "type": "pong",
  "payload": {},
  "timestamp": "2024-01-02T10:30:00Z"
}
```

## Error Handling

### HTTP Error Format

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validation failed",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ]
  },
  "request_id": "uuid"
}
```

### Common Error Codes

- `VALIDATION_ERROR` (400): Request validation failed
- `UNAUTHORIZED` (401): Authentication required
- `FORBIDDEN` (403): Insufficient permissions
- `NOT_FOUND` (404): Resource not found
- `CONFLICT` (409): Resource conflict (e.g., duplicate name)
- `PAYLOAD_TOO_LARGE` (413): File upload too large
- `RATE_LIMITED` (429): Too many requests
- `INTERNAL_ERROR` (500): Server error

### WebSocket Error Format

```json
{
  "type": "error",
  "payload": {
    "code": "SUBSCRIPTION_FAILED",
    "message": "Failed to subscribe to channel",
    "channel": "board:uuid"
  },
  "timestamp": "2024-01-02T10:30:00Z"
}
```

## Rate Limiting

- **Authentication endpoints**: 5 requests per minute per IP
- **API endpoints**: 1000 requests per hour per user
- **File uploads**: 10 files per minute per user
- **WebSocket connections**: 10 connections per user

Rate limit headers:
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1641024000
```

## Pagination

List endpoints support cursor-based pagination:

```http
GET /api/projects/{project_id}/tasks?limit=20&cursor=eyJpZCI6InV1aWQifQ

Response 200:
{
  "tasks": [ /* task objects */ ],
  "pagination": {
    "has_more": true,
    "next_cursor": "eyJpZCI6InV1aWQyIn0",
    "total": 150
  }
}
```