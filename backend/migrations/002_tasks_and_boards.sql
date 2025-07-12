-- Phase 3: Tasks and Boards migration
-- Adds task management and Kanban board functionality

-- Task status enum
DO $$ BEGIN
    CREATE TYPE task_status AS ENUM ('todo', 'inprogress', 'review', 'done');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Task priority enum
DO $$ BEGIN
    CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high', 'critical');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Tasks table
CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    assigned_to UUID REFERENCES users(id),
    status task_status NOT NULL DEFAULT 'todo',
    priority task_priority NOT NULL DEFAULT 'medium',
    due_date TIMESTAMPTZ,
    tags JSONB DEFAULT '[]', -- JSON array of tags
    position INTEGER NOT NULL DEFAULT 0, -- for drag-and-drop ordering
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Boards table
CREATE TABLE IF NOT EXISTS boards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    columns JSONB NOT NULL DEFAULT '["Todo", "In Progress", "Review", "Done"]', -- JSON array of column names
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Task comments table
CREATE TABLE IF NOT EXISTS task_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_created_by ON tasks(created_by);
CREATE INDEX IF NOT EXISTS idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
CREATE INDEX IF NOT EXISTS idx_tasks_position ON tasks(position);
CREATE INDEX IF NOT EXISTS idx_tasks_tags ON tasks USING gin(tags);

CREATE INDEX IF NOT EXISTS idx_boards_project_id ON boards(project_id);
CREATE INDEX IF NOT EXISTS idx_boards_created_by ON boards(created_by);
CREATE INDEX IF NOT EXISTS idx_boards_is_default ON boards(is_default);

CREATE INDEX IF NOT EXISTS idx_task_comments_task_id ON task_comments(task_id);
CREATE INDEX IF NOT EXISTS idx_task_comments_user_id ON task_comments(user_id);

-- Apply updated_at triggers to new tables
DO $$ BEGIN
    CREATE TRIGGER update_tasks_updated_at BEFORE UPDATE ON tasks
        FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TRIGGER update_boards_updated_at BEFORE UPDATE ON boards
        FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TRIGGER update_task_comments_updated_at BEFORE UPDATE ON task_comments
        FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Function to automatically create a default board for new projects
CREATE OR REPLACE FUNCTION create_default_board_for_project()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO boards (name, description, project_id, created_by, is_default)
    VALUES ('Default Board', 'Default Kanban board for project', NEW.id, NEW.created_by, true);
    
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to create default board when project is created
DO $$ BEGIN
    CREATE TRIGGER create_default_board AFTER INSERT ON projects
        FOR EACH ROW EXECUTE FUNCTION create_default_board_for_project();
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;