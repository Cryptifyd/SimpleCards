# Development Setup

This guide covers setting up the local development environment for the task management system.

## Prerequisites

### Required Software

- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **Node.js** (18+): [Install Node.js](https://nodejs.org/)
- **PostgreSQL** (14+): [Install PostgreSQL](https://www.postgresql.org/download/)
- **Redis** (6+): [Install Redis](https://redis.io/download)
- **Docker** (optional): [Install Docker](https://docs.docker.com/get-docker/)

### Development Tools

```bash
# Install Rust development tools
cargo install cargo-watch
cargo install sqlx-cli --features postgres

# Install Node.js development tools
npm install -g pnpm
npm install -g @next/eslint-plugin-next
```

## Project Structure

```
cards/
├── backend/                    # Rust API server
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/               # API route handlers
│   │   ├── auth/              # Authentication & authorization
│   │   ├── database/          # Database models & queries
│   │   ├── websocket/         # WebSocket handling
│   │   └── utils/             # Utility functions
│   ├── migrations/            # Database migrations
│   ├── Cargo.toml
│   └── .env.example
├── frontend/                   # Next.js React app
│   ├── src/
│   │   ├── app/               # App router pages
│   │   ├── components/        # Reusable components
│   │   ├── hooks/             # Custom React hooks
│   │   ├── lib/               # Utility libraries
│   │   └── types/             # TypeScript type definitions
│   ├── public/                # Static assets
│   ├── package.json
│   └── .env.local.example
├── docker-compose.yml         # Local development services
├── docker-compose.prod.yml    # Production configuration
└── README.md
```

## Database Setup

### Option 1: Local PostgreSQL Installation

1. **Install PostgreSQL** (if not using Docker)
2. **Create development database:**

```bash
# Connect to PostgreSQL
psql -U postgres

# Create user and database
CREATE USER taskmanager WITH PASSWORD 'dev_password';
CREATE DATABASE taskmanager_dev OWNER taskmanager;
GRANT ALL PRIVILEGES ON DATABASE taskmanager_dev TO taskmanager;

# Create test database
CREATE DATABASE taskmanager_test OWNER taskmanager;
GRANT ALL PRIVILEGES ON DATABASE taskmanager_test TO taskmanager;

\q
```

3. **Install Redis** (if not using Docker)

### Option 2: Docker Development Environment

```bash
# Start PostgreSQL and Redis using Docker
docker-compose up -d postgres redis

# Check services are running
docker-compose ps
```

**docker-compose.yml:**
```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: taskmanager_dev
      POSTGRES_USER: taskmanager
      POSTGRES_PASSWORD: dev_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backend/migrations:/docker-entrypoint-initdb.d

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data

  postgres_test:
    image: postgres:15
    environment:
      POSTGRES_DB: taskmanager_test
      POSTGRES_USER: taskmanager
      POSTGRES_PASSWORD: dev_password
    ports:
      - "5433:5432"
    volumes:
      - postgres_test_data:/var/lib/postgresql/data

volumes:
  postgres_data:
  postgres_test_data:
  redis_data:
```

## Backend Setup

### 1. Environment Configuration

Create `backend/.env` from `backend/.env.example`:

```bash
cd backend
cp .env.example .env
```

**backend/.env:**
```bash
# Database
DATABASE_URL=postgres://taskmanager:dev_password@localhost:5432/taskmanager_dev
DATABASE_TEST_URL=postgres://taskmanager:dev_password@localhost:5433/taskmanager_test

# Redis
REDIS_URL=redis://localhost:6379

# Server
HOST=127.0.0.1
PORT=8000
RUST_LOG=debug

# Authentication
JWT_SECRET=your-super-secret-jwt-key-for-development-only
JWT_EXPIRATION=3600
REFRESH_TOKEN_EXPIRATION=604800

# File Upload
UPLOAD_DIR=./uploads
MAX_FILE_SIZE=10485760  # 10MB

# CORS
CORS_ORIGIN=http://localhost:3000

# Rate Limiting
RATE_LIMIT_REQUESTS=1000
RATE_LIMIT_WINDOW=3600
```

### 2. Database Migration

```bash
cd backend

# Install sqlx CLI if not already installed
cargo install sqlx-cli --features postgres

# Run migrations
sqlx migrate run

# Verify migration status
sqlx migrate info
```

### 3. Install Dependencies and Run

```bash
cd backend

# Install dependencies
cargo build

# Run in development mode with auto-reload
cargo watch -x run

# Or run normally
cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### 4. API Documentation

The backend serves API documentation at:
- **Development**: http://localhost:8000/docs
- **OpenAPI Spec**: http://localhost:8000/api-docs/openapi.json

## Frontend Setup

### 1. Environment Configuration

Create `frontend/.env.local` from `frontend/.env.local.example`:

```bash
cd frontend
cp .env.local.example .env.local
```

**frontend/.env.local:**
```bash
# API Configuration
NEXT_PUBLIC_API_URL=http://localhost:8000
NEXT_PUBLIC_WS_URL=ws://localhost:8000/ws

# App Configuration
NEXT_PUBLIC_APP_NAME="Task Manager"
NEXT_PUBLIC_APP_VERSION="0.1.0"

# Feature Flags
NEXT_PUBLIC_ENABLE_ANALYTICS=false
NEXT_PUBLIC_ENABLE_ERROR_REPORTING=false

# Development
NODE_ENV=development
```

### 2. Install Dependencies and Run

```bash
cd frontend

# Install dependencies using pnpm
pnpm install

# Run development server
pnpm dev

# Or using npm
npm install
npm run dev

# Run type checking
pnpm type-check

# Run linting
pnpm lint

# Run tests
pnpm test
```

### 3. Access the Application

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8000

## Development Workflow

### 1. Starting Development Environment

```bash
# Terminal 1: Start databases (if using Docker)
docker-compose up postgres redis

# Terminal 2: Start backend
cd backend
cargo watch -x run

# Terminal 3: Start frontend
cd frontend
pnpm dev
```

### 2. Database Operations

```bash
# Create new migration
cd backend
sqlx migrate add create_tasks_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Reset database (DROP ALL DATA)
sqlx database drop
sqlx database create
sqlx migrate run
```

### 3. Code Quality

**Backend (Rust):**
```bash
cd backend

# Format code
cargo fmt

# Run clippy (linter)
cargo clippy

# Run tests with coverage
cargo test
```

**Frontend (Next.js):**
```bash
cd frontend

# Format code
pnpm format

# Run linter
pnpm lint

# Fix linting issues
pnpm lint:fix

# Type checking
pnpm type-check

# Run tests
pnpm test

# Run tests in watch mode
pnpm test:watch
```

## Testing

### Backend Testing

```bash
cd backend

# Run all tests
cargo test

# Run specific test
cargo test auth::tests::test_login

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration

# Run with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Frontend Testing

```bash
cd frontend

# Run unit tests
pnpm test

# Run E2E tests (requires backend running)
pnpm test:e2e

# Run component tests
pnpm test:components

# Update snapshots
pnpm test -- --updateSnapshot
```

### Test Database

For backend tests, use a separate test database:

```bash
# Set test database URL
export DATABASE_URL=postgres://taskmanager:dev_password@localhost:5433/taskmanager_test

# Run migrations on test database
sqlx migrate run

# Run tests
cargo test
```

## Debugging

### Backend Debugging

1. **Logging Configuration:**
```bash
# Detailed logging
RUST_LOG=debug cargo run

# SQL query logging
RUST_LOG=sqlx=debug cargo run

# Specific module logging
RUST_LOG=taskmanager::auth=trace cargo run
```

2. **Using VS Code with rust-analyzer:**
   - Install `rust-analyzer` extension
   - Configure launch.json for debugging
   - Set breakpoints and debug normally

### Frontend Debugging

1. **Next.js Debug Mode:**
```bash
# Enable debug mode
DEBUG=* pnpm dev

# Debug specific modules
DEBUG=next:* pnpm dev
```

2. **Browser DevTools:**
   - React DevTools extension
   - Redux DevTools (if using Redux)
   - Performance profiling

## Common Issues and Solutions

### Database Connection Issues

```bash
# Check PostgreSQL is running
pg_isready -h localhost -p 5432

# Check Redis is running
redis-cli ping

# Reset database connection
sudo service postgresql restart
```

### Cargo Build Issues

```bash
# Clean build cache
cargo clean
cargo build

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Node.js Issues

```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Clear Next.js cache
rm -rf .next

# Check for outdated packages
npm outdated
```

### Port Conflicts

```bash
# Find process using port
lsof -i :8000
lsof -i :3000

# Kill process
kill -9 <PID>

# Use different ports
PORT=8001 cargo run
NEXT_PUBLIC_PORT=3001 pnpm dev
```

## Development Tools and Extensions

### VS Code Extensions

- **Rust:** rust-analyzer
- **TypeScript:** ES7+ React/Redux/React-Native snippets
- **Next.js:** Next.js snippets
- **Database:** PostgreSQL, SQLTools
- **Git:** GitLens
- **Formatting:** Prettier, EditorConfig

### Chrome Extensions

- React Developer Tools
- Redux DevTools
- Apollo Client DevTools (if using GraphQL)

### Terminal Tools

```bash
# Install useful CLI tools
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-outdated
npm install -g npm-check-updates
```

## Performance Monitoring

### Development Metrics

```bash
# Backend performance
cargo build --release
time ./target/release/taskmanager

# Frontend bundle analysis
cd frontend
pnpm build
pnpm analyze
```

### Memory and CPU Monitoring

```bash
# Monitor Rust application
ps aux | grep taskmanager
top -p $(pgrep taskmanager)

# Monitor Node.js application
htop
node --inspect pnpm dev
```

This development setup provides a comprehensive environment for building and testing the task management system with hot reloading, debugging capabilities, and proper testing infrastructure.