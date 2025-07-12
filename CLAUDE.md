# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a task management tool (Trello alternative) in early planning/concept phase. The project aims to create a modern, self-hosted task management solution with flexible boards, granular permissions, and advanced features like roadmaps.

## Technology Stack

- **Frontend**: Next.js (React)
- **Backend**: Rust + axum + tokio
- **Database**: PostgreSQL
- **Realtime**: WebSockets (tokio-tungstenite)
- **Optional Scaling**: Redis PubSub
- **Deployment**: Docker + Kubernetes

## Key Architecture Concepts

### Board System
- **Boards are views, not data containers** - they display filtered/grouped tasks
- Two board types: Team boards (shared) and Personal boards (private)
- Configurable grouping by status, assignee, label, or epic
- Status lists are project-wide configurations

### Core Data Model
- **Tasks/Cards**: Primary entities with title, description, status, assignments, labels, etc.
- **Epics**: Higher-level containers that group related tasks
- **Projects**: Contain tasks and define project-wide settings like status lists
- **Teams**: Group users and projects for permission management

### Permission System
- **Admin**: Full project control, can manage teams and boards
- **Member**: Can create/edit tasks, manage team boards, edit project status
- **Editor**: Can create/edit tasks, create personal boards only
- **Guest**: Read-only access

### Realtime Features
- WebSocket-based live updates
- Separate feeds for board-level and card-level changes
- Real-time collaboration on comments and task editing

## Current Development Status

The project is in the concept/planning phase. Key documents include:
- `Concept.md`: Detailed UI/UX specifications for all modules
- `Projektplan.md`: Technical roadmap and data models

No code has been implemented yet - this is purely a planning repository at this stage.

## Development Commands

*Note: No package.json or build system exists yet as this is a planning phase*

## MVP Features Planned

1. Project & team management
2. Flexible board system with configurable views
3. Task creation, editing, and drag-and-drop
4. Real-time updates via WebSockets
5. Comment system
6. Epic management and roadmap views
7. Role-based permission system
8. Authentication system