# Phase 4: Real-time Features - Testing Plan

This document provides a comprehensive testing plan for Phase 4 real-time features. This plan covers WebSocket functionality, live collaboration, and real-time updates.

## Prerequisites

1. **Phase 3 Completion**: Ensure Phase 3 (Task and Board Management) is working correctly
2. **Database Setup**: PostgreSQL with all migrations applied through Phase 3
3. **Server Running**: Backend server with WebSocket support (`cargo run`)
4. **Authentication**: Valid JWT tokens for testing users
5. **Test Data**: At least one team, project, and multiple users for collaboration testing
6. **WebSocket Client**: Tool to test WebSocket connections (wscat, browser dev tools, or custom client)

## Testing Overview

### Core Features to Test
- ✅ WebSocket connection and authentication
- ✅ Project subscription and user presence
- ✅ Real-time task operations (create, update, delete, move)
- ✅ Real-time board operations (create, update, delete)
- ✅ Real-time comment operations (create, delete)
- ✅ Live collaboration indicators (typing, presence)
- ✅ Connection management and error handling

## WebSocket Connection Testing

### 1. WebSocket Endpoint Verification

```bash
# Test WebSocket endpoint availability
curl -i -N -H "Connection: Upgrade" \
    -H "Upgrade: websocket" \
    -H "Sec-WebSocket-Version: 13" \
    -H "Sec-WebSocket-Key: test" \
    http://localhost:8000/ws

# Expected: HTTP/1.1 101 Switching Protocols
```

### 2. Authentication Testing

#### 2.1 Valid Token Authentication
```javascript
// Connect with valid JWT token
const ws = new WebSocket('ws://localhost:8000/ws?token=YOUR_JWT_TOKEN');

ws.onopen = function() {
    console.log('Connected to WebSocket');
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Received:', data);
    
    // Expected first message:
    // {
    //   "type": "AuthenticationSuccess",
    //   "data": { "user_id": "uuid-here" }
    // }
};
```

#### 2.2 Invalid Token Authentication
```javascript
// Connect with invalid token
const ws = new WebSocket('ws://localhost:8000/ws?token=invalid-token');

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Error:', data);
    
    // Expected:
    // {
    //   "type": "AuthenticationError",
    //   "data": { "message": "Invalid token" }
    // }
};
```

#### 2.3 No Token Authentication
```javascript
// Connect without token
const ws = new WebSocket('ws://localhost:8000/ws');

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    
    // Expected:
    // {
    //   "type": "AuthenticationError",
    //   "data": { "message": "No token provided" }
    // }
};
```

### 3. Project Subscription Testing

#### 3.1 Valid Project Subscription
```javascript
// Subscribe to project updates
ws.send(JSON.stringify({
    type: "Subscribe",
    data: { project_id: "project-uuid-here" }
}));

// Expected response:
// {
//   "type": "SubscriptionSuccess",
//   "data": { "project_id": "project-uuid-here" }
// }

// Followed by:
// {
//   "type": "UserJoined",
//   "data": {
//     "user": { "id": "uuid", "username": "user", "display_name": "User" },
//     "project_id": "project-uuid-here",
//     "timestamp": "2024-01-01T00:00:00Z"
//   }
// }
```

#### 3.2 Unauthorized Project Subscription
```javascript
// Try to subscribe to project user doesn't have access to
ws.send(JSON.stringify({
    type: "Subscribe",
    data: { project_id: "unauthorized-project-uuid" }
}));

// Expected:
// {
//   "type": "SubscriptionError",
//   "data": { "message": "Not a project member" }
// }
```

## Real-time Task Operations Testing

### 4. Task Creation Broadcasting

#### 4.1 Real-time Task Creation
```bash
# User A: Create a task via API
curl -X POST http://localhost:8000/api/projects/{project_id}/tasks \
  -H "Authorization: Bearer {user_a_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Real-time Test Task",
    "description": "Testing real-time task creation",
    "priority": "high",
    "tags": ["testing", "realtime"]
  }'

# User B (connected via WebSocket): Should receive:
# {
#   "type": "TaskCreated",
#   "data": {
#     "task": { task_object },
#     "project_id": "project-uuid",
#     "user": { user_a_summary }
#   }
# }
```

### 4.2 Task Update Broadcasting
```bash
# User A: Update a task
curl -X PUT http://localhost:8000/api/tasks/{task_id} \
  -H "Authorization: Bearer {user_a_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Task Title",
    "status": "inprogress",
    "assigned_to": "{user_b_id}"
  }'

# All connected users except User A should receive:
# {
#   "type": "TaskUpdated",
#   "data": {
#     "task": { updated_task_object },
#     "project_id": "project-uuid",
#     "user": { user_a_summary }
#   }
# }
```

### 4.3 Task Move Broadcasting
```bash
# User A: Move a task (drag & drop simulation)
curl -X POST http://localhost:8000/api/tasks/{task_id}/move \
  -H "Authorization: Bearer {user_a_token}" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "review",
    "position": 3
  }'

# All connected users except User A should receive:
# {
#   "type": "TaskMoved",
#   "data": {
#     "task_id": "task-uuid",
#     "from_status": "inprogress",
#     "to_status": "review",
#     "position": 3,
#     "project_id": "project-uuid",
#     "user": { user_a_summary }
#   }
# }
```

### 4.4 Task Deletion Broadcasting
```bash
# User A: Delete a task
curl -X DELETE http://localhost:8000/api/tasks/{task_id} \
  -H "Authorization: Bearer {user_a_token}"

# All connected users except User A should receive:
# {
#   "type": "TaskDeleted",
#   "data": {
#     "task_id": "task-uuid",
#     "project_id": "project-uuid"
#   }
# }
```

## Real-time Board Operations Testing

### 5. Board Management Broadcasting

#### 5.1 Board Creation
```bash
# Create a new board
curl -X POST http://localhost:8000/api/projects/{project_id}/boards \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sprint Planning Board",
    "description": "Board for sprint planning",
    "columns": ["Backlog", "Sprint", "In Progress", "Review", "Done"]
  }'

# Expected WebSocket broadcast:
# {
#   "type": "BoardCreated",
#   "data": {
#     "board": { board_object },
#     "project_id": "project-uuid",
#     "user": { user_summary }
#   }
# }
```

#### 5.2 Board Update
```bash
# Update board configuration
curl -X PUT http://localhost:8000/api/boards/{board_id} \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Board Name",
    "columns": ["Todo", "Doing", "Review", "Complete"]
  }'

# Expected WebSocket broadcast:
# {
#   "type": "BoardUpdated",
#   "data": {
#     "board": { updated_board_object },
#     "project_id": "project-uuid",
#     "user": { user_summary }
#   }
# }
```

#### 5.3 Board Deletion
```bash
# Delete a board (admin only)
curl -X DELETE http://localhost:8000/api/boards/{board_id} \
  -H "Authorization: Bearer {admin_token}"

# Expected WebSocket broadcast:
# {
#   "type": "BoardDeleted",
#   "data": {
#     "board_id": "board-uuid",
#     "project_id": "project-uuid"
#   }
# }
```

## Real-time Comment Operations Testing

### 6. Comment System Broadcasting

#### 6.1 Comment Creation
```bash
# Add a comment to a task
curl -X POST http://localhost:8000/api/tasks/{task_id}/comments \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "This is a real-time comment test!"
  }'

# Expected WebSocket broadcast:
# {
#   "type": "CommentCreated",
#   "data": {
#     "comment": { comment_object },
#     "task_id": "task-uuid",
#     "project_id": "project-uuid",
#     "user": { user_summary }
#   }
# }
```

#### 6.2 Comment Deletion
```bash
# Delete a comment
curl -X DELETE http://localhost:8000/api/comments/{comment_id} \
  -H "Authorization: Bearer {token}"

# Expected WebSocket broadcast:
# {
#   "type": "CommentDeleted",
#   "data": {
#     "comment_id": "comment-uuid",
#     "task_id": "task-uuid",
#     "project_id": "project-uuid"
#   }
# }
```

## Live Collaboration Features Testing

### 7. User Presence Testing

#### 7.1 User Join/Leave Events
```javascript
// User A connects and subscribes to project
// Other users should receive:
// {
//   "type": "UserJoined",
//   "data": {
//     "user": { user_a_summary },
//     "project_id": "project-uuid",
//     "timestamp": "2024-01-01T00:00:00Z"
//   }
// }

// User A disconnects
// Other users should receive:
// {
//   "type": "UserLeft",
//   "data": {
//     "user": { user_a_summary },
//     "project_id": "project-uuid",
//     "timestamp": "2024-01-01T00:00:00Z"
//   }
// }
```

#### 7.2 Typing Indicators
```javascript
// User A starts typing in a task comment
ws.send(JSON.stringify({
    type: "UserTyping",
    data: {
        user: { user_summary },
        task_id: "task-uuid",
        project_id: "project-uuid",
        timestamp: new Date().toISOString()
    }
}));

// Other users should receive the same event

// User A stops typing
ws.send(JSON.stringify({
    type: "UserStoppedTyping",
    data: {
        user: { user_summary },
        task_id: "task-uuid", 
        project_id: "project-uuid",
        timestamp: new Date().toISOString()
    }
}));
```

### 8. Connection Management Testing

#### 8.1 Heartbeat/Ping-Pong
```javascript
// Server should send periodic ping frames
// Client should respond with pong frames automatically
// Test by monitoring WebSocket frames in browser dev tools

// Manual pong test
ws.send(JSON.stringify({
    type: "Pong"
}));
```

#### 8.2 Connection Recovery
```javascript
// Simulate network interruption
// Close connection and reconnect
ws.close();

// Reconnect after delay
setTimeout(() => {
    const newWs = new WebSocket('ws://localhost:8000/ws?token=YOUR_JWT_TOKEN');
    // Should be able to re-authenticate and re-subscribe
}, 2000);
```

#### 8.3 Multiple Connections per User
```javascript
// Open multiple WebSocket connections for same user
const ws1 = new WebSocket('ws://localhost:8000/ws?token=YOUR_JWT_TOKEN');
const ws2 = new WebSocket('ws://localhost:8000/ws?token=YOUR_JWT_TOKEN');

// Both should receive broadcasts
// Test that user presence is handled correctly
```

## Error Handling Testing

### 9. Error Scenarios

#### 9.1 Invalid Message Format
```javascript
// Send malformed JSON
ws.send("invalid json");

// Expected:
// {
//   "type": "Error",
//   "data": { "message": "Invalid message format" }
// }
```

#### 9.2 Unknown Event Type
```javascript
// Send unknown event type
ws.send(JSON.stringify({
    type: "UnknownEvent",
    data: {}
}));

// Should be gracefully ignored or return error
```

#### 9.3 Permission Errors
```javascript
// Try to perform actions without proper permissions
// Should receive appropriate error responses
```

## Performance Testing

### 10. Load and Performance Tests

#### 10.1 Multiple Concurrent Connections
```bash
# Use a tool like wscat to create multiple connections
for i in {1..50}; do
    wscat -c "ws://localhost:8000/ws?token=$TOKEN" &
done

# Monitor server performance and memory usage
```

#### 10.2 High-Frequency Updates
```bash
# Create many rapid task updates
for i in {1..100}; do
    curl -X PUT http://localhost:8000/api/tasks/{task_id} \
      -H "Authorization: Bearer {token}" \
      -H "Content-Type: application/json" \
      -d "{\"title\": \"Rapid Update $i\"}"
    sleep 0.1
done

# Verify all updates are broadcast correctly
```

#### 10.3 Large Project Subscriptions
```bash
# Test with projects that have many tasks and boards
# Verify performance when many users are subscribed to same project
```

## Integration Testing Scenarios

### 11. End-to-End Workflows

#### 11.1 Collaborative Task Management
1. User A creates a project and invites User B
2. Both users connect to WebSocket and subscribe to project
3. User A creates several tasks
4. User B receives real-time notifications
5. User B moves tasks between columns
6. User A sees updates in real-time
7. Both users add comments to same task
8. Comments appear instantly for both users

#### 11.2 Multi-Board Collaboration
1. Create multiple boards for same project
2. Multiple users subscribe to project
3. Test real-time updates across different boards
4. Verify board configuration changes are broadcast
5. Test task movements between different board statuses

#### 11.3 Permission-Based Broadcasting
1. Create project with different user roles (Guest, Editor, Admin)
2. Verify guests only receive read-only events
3. Test that editors can create/update but not delete
4. Test admin-only operations (board deletion)

## Browser Testing

### 12. Cross-Browser Compatibility

#### 12.1 WebSocket Support
- Test in Chrome, Firefox, Safari, Edge
- Verify WebSocket connection establishment
- Test message sending and receiving
- Verify proper connection closure

#### 12.2 Real-time UI Updates
- Test with actual frontend implementation
- Verify DOM updates happen in real-time
- Test drag-and-drop with real-time sync
- Verify typing indicators display correctly

## Automated Testing

### 13. Test Suite Integration

#### 13.1 Unit Tests
```rust
// Example WebSocket unit test
#[tokio::test]
async fn test_websocket_authentication() {
    // Test WebSocket authentication logic
}

#[tokio::test]
async fn test_project_subscription() {
    // Test project subscription permissions
}
```

#### 13.2 Integration Tests
```rust
// Example integration test
#[tokio::test]
async fn test_task_creation_broadcast() {
    // Create WebSocket connections
    // Create task via API
    // Verify broadcast received
}
```

## Monitoring and Debugging

### 14. Debugging Tools

#### 14.1 Server-Side Logging
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Monitor WebSocket connections and events
tail -f logs/websocket.log
```

#### 14.2 Client-Side Debugging
```javascript
// Enable verbose WebSocket logging
const ws = new WebSocket('ws://localhost:8000/ws?token=TOKEN');
ws.onopen = () => console.log('WS: Connected');
ws.onmessage = (e) => console.log('WS: Received', JSON.parse(e.data));
ws.onerror = (e) => console.error('WS: Error', e);
ws.onclose = (e) => console.log('WS: Closed', e.code, e.reason);
```

## Success Criteria

### 15. Testing Completion Checklist

✅ **WebSocket Infrastructure**
- [ ] Connection establishment works
- [ ] Authentication properly validates JWT tokens
- [ ] Connection cleanup happens on disconnect

✅ **Real-time Broadcasting**
- [ ] Task operations broadcast to all subscribers
- [ ] Board operations broadcast correctly
- [ ] Comment operations broadcast immediately
- [ ] Events excluded from originating user

✅ **User Presence**
- [ ] Join/leave events work correctly
- [ ] Typing indicators function properly
- [ ] Multiple connections per user handled

✅ **Permission System**
- [ ] Only project members receive events
- [ ] Role-based permissions respected
- [ ] Unauthorized access properly blocked

✅ **Error Handling**
- [ ] Invalid tokens rejected gracefully
- [ ] Malformed messages handled properly
- [ ] Connection errors don't crash server

✅ **Performance**
- [ ] Multiple concurrent connections supported
- [ ] High-frequency updates don't cause issues
- [ ] Memory usage remains reasonable

## Troubleshooting Guide

### Common Issues and Solutions

#### Connection Issues
- **Problem**: WebSocket connection fails
- **Check**: JWT token validity, server logs, network connectivity
- **Solution**: Verify token format and expiration

#### Authentication Issues
- **Problem**: Authentication fails with valid token
- **Check**: JWT secret configuration, token claims structure
- **Solution**: Ensure JWT_SECRET environment variable is set

#### Broadcasting Issues
- **Problem**: Events not received by subscribers
- **Check**: Project subscription status, user permissions
- **Solution**: Verify user is project member and properly subscribed

#### Performance Issues
- **Problem**: Slow or delayed updates
- **Check**: Server load, network latency, number of connections
- **Solution**: Monitor server resources and optimize if needed

## Reporting Issues

When reporting issues, include:
1. **Steps to reproduce**
2. **Expected vs actual behavior**
3. **WebSocket connection details**
4. **Server logs (with timestamps)**
5. **Client-side console logs**
6. **Network tab information**
7. **User roles and permissions**

This comprehensive testing plan ensures all real-time features work correctly and provides a solid foundation for collaborative task management.