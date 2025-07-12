# SimpleCards

A modern, self-hosted task management tool built with Next.js and Rust.

## Overview

SimpleCards is a flexible task management system designed as an alternative to Trello and Jira. It provides customizable boards, granular permissions, real-time collaboration, and advanced features like roadmaps and epics.

## Features

- **Flexible Board System**: Configure boards with custom groupings (status, assignee, labels, epics)
- **Real-time Collaboration**: Live updates via WebSocket connections
- **Granular Permissions**: Role-based access control at team and project levels
- **Epic Management**: High-level planning with roadmap visualizations
- **Rich Task Management**: Comments, attachments, activity tracking
- **Modern Architecture**: Built with Rust backend and Next.js frontend

## Technology Stack

- **Frontend**: Next.js 14+ with TypeScript and Tailwind CSS
- **Backend**: Rust with axum web framework and tokio async runtime
- **Database**: PostgreSQL with Redis for caching and real-time features
- **Deployment**: Docker containers with Kubernetes support

## Project Status

🚧 **Under Development** - Currently in initial implementation phase.

## Documentation

- [Technical Architecture](./technical-architecture.md) - System design and database schema
- [API Specification](./api-specification.md) - REST and WebSocket API documentation
- [Data Models](./data-models.md) - Entity definitions and business rules
- [Development Setup](./development-setup.md) - Local development environment
- [Deployment Guide](./deployment.md) - Production deployment instructions
- [Implementation Roadmap](./implementation-roadmap.md) - Development timeline and milestones

## Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 18+
- PostgreSQL 14+
- Redis 6+
- Docker (optional, recommended for development)

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/SimpleCards.git
   cd SimpleCards
   ```

2. **Start development services**
   ```bash
   docker-compose up -d postgres redis
   ```

3. **Set up backend**
   ```bash
   cd backend
   cp .env.example .env
   cargo install sqlx-cli --features postgres
   sqlx migrate run
   cargo run
   ```

4. **Set up frontend**
   ```bash
   cd frontend
   cp .env.local.example .env.local
   npm install
   npm run dev
   ```

5. **Access the application**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8000

## Contributing

Please read our [Implementation Roadmap](./implementation-roadmap.md) for details on our development process and coding standards.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Concept Documents

This project started with comprehensive planning. The original concept documents are preserved in the repository:

- [Concept.md](./Concept.md) - Detailed UI/UX specifications (German)
- [Projektplan.md](./Projektplan.md) - Project overview and goals (German)
- [CLAUDE.md](./CLAUDE.md) - Development guidance for AI assistants