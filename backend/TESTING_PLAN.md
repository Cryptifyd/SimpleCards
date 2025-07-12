# Phase 3: Task and Board Management - Testing Plan

This document provides a comprehensive testing plan for Phase 3 features. Please use this guide to thoroughly test the new task and board management functionality.

## Prerequisites

1. **Database Setup**: Ensure you have PostgreSQL running and the Phase 3 migration has been applied
2. **Server Running**: Start the backend server with `cargo run`
3. **Authentication**: You'll need valid JWT tokens for testing (register/login first)
4. **Test Data**: Create at least one team and project for testing

## Testing Overview

### Core Features to Test
- ✅ Task CRUD operations
- ✅ Task filtering and searching
- ✅ Task assignment and status management
- ✅ Board management and customization
- ✅ Task comments and collaboration
- ✅ Permission-based access control
- ✅ Drag-and-drop task movement

## Detailed Test Cases

### 1. Task Management Tests

#### 1.1 Create Task
```bash
# Test creating a basic task
curl -X POST http://localhost:8000/api/projects/{project_id}/tasks \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Implement user authentication",
    "description": "Add JWT-based authentication system",
    "priority": "high",
    "tags": ["backend", "security"]
  }'

# Expected: 201 Created with task details
# Test validation: Try with empty title (should fail)
```

#### 1.2 Get Project Tasks
```bash
# Get all tasks for a project
curl -X GET http://localhost:8000/api/projects/{project_id}/tasks \
  -H "Authorization: Bearer {your_token}"

# Test with filters
curl -X GET "http://localhost:8000/api/projects/{project_id}/tasks?status=todo&priority=high" \
  -H "Authorization: Bearer {your_token}"

# Expected: 200 OK with filtered task list
```

#### 1.3 Update Task
```bash
# Update task details
curl -X PUT http://localhost:8000/api/tasks/{task_id} \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated task title",
    "status": "inprogress",
    "assigned_to": "{user_id}"
  }'

# Expected: 200 OK with updated task
# Test permissions: Try as non-editor (should fail with 403)
```

#### 1.4 Move Task (Drag & Drop)
```bash
# Move task between columns
curl -X POST http://localhost:8000/api/tasks/{task_id}/move \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "review",
    "position": 2
  }'

# Expected: 200 OK with updated task position
```

#### 1.5 Delete Task
```bash
# Delete task (only admin or creator)
curl -X DELETE http://localhost:8000/api/tasks/{task_id} \
  -H "Authorization: Bearer {your_token}"

# Expected: 204 No Content
# Test permissions: Try as non-admin/non-creator (should fail)
```

### 2. Board Management Tests

#### 2.1 Create Board
```bash
# Create custom board
curl -X POST http://localhost:8000/api/projects/{project_id}/boards \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Development Board",
    "description": "Board for development tasks",
    "columns": ["Backlog", "In Progress", "Testing", "Done"]
  }'

# Expected: 201 Created with board details
# Note: Default board should already exist from project creation
```

#### 2.2 Get Board with Tasks
```bash
# Get board details with all tasks
curl -X GET http://localhost:8000/api/boards/{board_id} \
  -H "Authorization: Bearer {your_token}"

# Expected: 200 OK with board details and all project tasks
```

#### 2.3 Update Board
```bash
# Update board configuration
curl -X PUT http://localhost:8000/api/boards/{board_id} \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Board Name",
    "columns": ["Todo", "Doing", "Review", "Complete"]
  }'

# Expected: 200 OK with updated board
```

### 3. Task Comments Tests

#### 3.1 Create Comment
```bash
# Add comment to task
curl -X POST http://localhost:8000/api/tasks/{task_id}/comments \
  -H "Authorization: Bearer {your_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "This task needs to be prioritized for the next sprint."
  }'

# Expected: 201 Created with comment details
```

#### 3.2 Get Task Comments
```bash
# Get all comments for a task
curl -X GET http://localhost:8000/api/tasks/{task_id}/comments \
  -H "Authorization: Bearer {your_token}"

# Expected: 200 OK with comments and user details
```

#### 3.3 Delete Comment
```bash
# Delete own comment
curl -X DELETE http://localhost:8000/api/comments/{comment_id} \
  -H "Authorization: Bearer {your_token}"

# Expected: 204 No Content
# Test: Try deleting another user's comment (should fail)
```

### 4. Permission Testing

#### 4.1 Project Member Access
```bash
# Test as project member with different roles:
# - Guest: Can view tasks but not edit
# - Editor: Can create/edit tasks and boards
# - Admin: Full access including deletion

# Create second user and test access levels
# Add user to project with Guest role
# Try creating task (should fail for Guest)
```

#### 4.2 Non-Member Access
```bash
# Test with user not in project
# All endpoints should return 403 Forbidden
```

### 5. Integration Testing Scenarios

#### 5.1 Complete Task Workflow
1. Create a new task
2. Assign it to a team member
3. Move it through different statuses
4. Add comments during progress
5. Complete the task
6. Verify all changes are reflected

#### 5.2 Board Management Workflow
1. Create a custom board
2. Configure custom columns
3. Create tasks and verify they appear on board
4. Move tasks between columns
5. Update board configuration
6. Verify task positions are maintained

#### 5.3 Collaboration Workflow
1. User A creates a task
2. User B gets assigned to the task
3. User B updates task status and adds comment
4. User A responds with comment
5. Task moves through workflow to completion
6. Verify all activity is tracked

## Expected Behaviors

### Task Status Workflow
- `todo` → `inprogress` → `review` → `done`
- Tasks can move backward in the workflow
- Position updates automatically when status changes

### Permission Matrix
| Role | View Tasks | Create Tasks | Edit Tasks | Delete Tasks | Manage Boards |
|------|------------|--------------|------------|--------------|---------------|
| Guest | ✅ | ❌ | ❌ | ❌ | ❌ |
| Editor | ✅ | ✅ | ✅ | Own only | ✅ |
| Admin | ✅ | ✅ | ✅ | ✅ | ✅ |

### Validation Rules
- Task title: 2-255 characters, required
- Task description: Max 2000 characters, optional
- Board name: 2-100 characters, required
- Comment content: 1-1000 characters, required
- Tags: Array of strings, optional

## Error Scenarios to Test

1. **Validation Errors**: Empty titles, too long descriptions
2. **Permission Errors**: Non-members trying to access tasks
3. **Not Found Errors**: Invalid task/board/project IDs
4. **Conflict Errors**: Assigning non-project members to tasks

## Performance Testing

1. **Large Task Lists**: Create 100+ tasks and test filtering
2. **Board Loading**: Test board with many tasks
3. **Comment Threads**: Test tasks with many comments

## Browser Testing (If Frontend Available)

1. **Drag and Drop**: Verify smooth task movement
2. **Real-time Updates**: Test collaborative editing
3. **Responsive Design**: Test on different screen sizes
4. **Keyboard Navigation**: Test accessibility features

## Automated Testing

Run the test suite if available:
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests
```

## Reporting Issues

When reporting issues, please include:
1. **Steps to reproduce**
2. **Expected behavior**
3. **Actual behavior**
4. **Request/response details**
5. **User roles and permissions**
6. **Browser/environment details**

## Success Criteria

✅ All API endpoints respond correctly  
✅ Permission system works as designed  
✅ Task workflow functions smoothly  
✅ Board management is intuitive  
✅ Comments system works properly  
✅ Error handling is appropriate  
✅ Performance is acceptable  

## Notes

- Default boards are automatically created for new projects
- Task positions are automatically managed
- All timestamps are in UTC
- JWT tokens expire after 1 hour
- Database constraints prevent orphaned records

This testing plan ensures comprehensive coverage of Phase 3 functionality. Please test thoroughly and report any issues found.