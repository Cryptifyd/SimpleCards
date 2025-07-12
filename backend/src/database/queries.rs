use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::database::models::{
    User, CreateUserRequest, UpdateUserRequest,
    Team, CreateTeamRequest, TeamMember, TeamRole,
    Project, CreateProjectRequest, ProjectMember, ProjectRole, UserSummary,
    Task, CreateTaskRequest, UpdateTaskRequest, TaskStatus, TaskPriority,
    Board, CreateBoardRequest, UpdateBoardRequest, MoveTaskRequest,
    TaskComment, CreateTaskCommentRequest
};
use crate::utils::errors::AppError;

pub struct UserQueries;

impl UserQueries {
    pub async fn create_user(
        pool: &PgPool,
        request: &CreateUserRequest,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (email, username, display_name, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at
            "#
        )
        .bind(&request.email)
        .bind(&request.username)
        .bind(&request.display_name)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE email = $1 AND is_active = true"
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, AppError> {
        let row = sqlx::query(
            "SELECT id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: Uuid,
        request: &UpdateUserRequest,
    ) -> Result<User, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE users 
            SET 
                display_name = COALESCE($2, display_name),
                avatar_url = COALESCE($3, avatar_url),
                updated_at = NOW()
            WHERE id = $1 AND is_active = true
            RETURNING id, email, username, password_hash, display_name, avatar_url, is_active, created_at, updated_at
            "#
        )
        .bind(user_id)
        .bind(&request.display_name)
        .bind(&request.avatar_url)
        .fetch_one(pool)
        .await?;

        let user = User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    pub async fn check_email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }

    pub async fn check_username_exists(pool: &PgPool, username: &str) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }

    pub async fn deactivate_user(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct TeamQueries;

impl TeamQueries {
    pub async fn create_team(
        pool: &PgPool,
        request: &CreateTeamRequest,
        created_by: Uuid,
    ) -> Result<Team, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO teams (name, description, created_by)
            VALUES ($1, $2, $3)
            RETURNING id, name, description, created_by, created_at, updated_at
            "#
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(created_by)
        .fetch_one(pool)
        .await?;

        let team = Team {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        // Add creator as admin
        TeamQueries::add_team_member(pool, team.id, created_by, TeamRole::Admin).await?;

        Ok(team)
    }

    pub async fn get_team_by_id(pool: &PgPool, team_id: Uuid) -> Result<Team, AppError> {
        let row = sqlx::query(
            "SELECT id, name, description, created_by, created_at, updated_at FROM teams WHERE id = $1"
        )
        .bind(team_id)
        .fetch_one(pool)
        .await?;

        let team = Team {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(team)
    }

    pub async fn get_user_teams(pool: &PgPool, user_id: Uuid) -> Result<Vec<Team>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT t.id, t.name, t.description, t.created_by, t.created_at, t.updated_at
            FROM teams t
            INNER JOIN team_members tm ON t.id = tm.team_id
            WHERE tm.user_id = $1
            ORDER BY t.name
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let teams = rows.into_iter().map(|row| Team {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(teams)
    }

    pub async fn update_team(
        pool: &PgPool,
        team_id: Uuid,
        request: &CreateTeamRequest,
    ) -> Result<Team, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE teams 
            SET name = $2, description = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, created_by, created_at, updated_at
            "#
        )
        .bind(team_id)
        .bind(&request.name)
        .bind(&request.description)
        .fetch_one(pool)
        .await?;

        let team = Team {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(team)
    }

    pub async fn delete_team(pool: &PgPool, team_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM teams WHERE id = $1")
            .bind(team_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn add_team_member(
        pool: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
        role: TeamRole,
    ) -> Result<TeamMember, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO team_members (team_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING id, team_id, user_id, role, joined_at
            "#
        )
        .bind(team_id)
        .bind(user_id)
        .bind(&role)
        .fetch_one(pool)
        .await?;

        let member = TeamMember {
            id: row.get("id"),
            team_id: row.get("team_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
            joined_at: row.get("joined_at"),
        };

        Ok(member)
    }

    pub async fn remove_team_member(
        pool: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_team_member_role(
        pool: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
        role: TeamRole,
    ) -> Result<TeamMember, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE team_members 
            SET role = $3
            WHERE team_id = $1 AND user_id = $2
            RETURNING id, team_id, user_id, role, joined_at
            "#
        )
        .bind(team_id)
        .bind(user_id)
        .bind(&role)
        .fetch_one(pool)
        .await?;

        let member = TeamMember {
            id: row.get("id"),
            team_id: row.get("team_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
            joined_at: row.get("joined_at"),
        };

        Ok(member)
    }

    pub async fn get_team_members(pool: &PgPool, team_id: Uuid) -> Result<Vec<(TeamMember, UserSummary)>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                tm.id, tm.team_id, tm.user_id, tm.role, tm.joined_at,
                u.username, u.display_name, u.avatar_url
            FROM team_members tm
            INNER JOIN users u ON tm.user_id = u.id
            WHERE tm.team_id = $1 AND u.is_active = true
            ORDER BY tm.role, u.display_name
            "#
        )
        .bind(team_id)
        .fetch_all(pool)
        .await?;

        let members = rows.into_iter().map(|row| {
            let member = TeamMember {
                id: row.get("id"),
                team_id: row.get("team_id"),
                user_id: row.get("user_id"),
                role: row.get("role"),
                joined_at: row.get("joined_at"),
            };

            let user = UserSummary {
                id: member.user_id,
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
            };

            (member, user)
        }).collect();

        Ok(members)
    }

    pub async fn get_user_team_role(
        pool: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TeamRole>, AppError> {
        let row = sqlx::query(
            "SELECT role FROM team_members WHERE team_id = $1 AND user_id = $2"
        )
        .bind(team_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.get("role")))
    }

    pub async fn is_team_member(
        pool: &PgPool,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM team_members WHERE team_id = $1 AND user_id = $2)"
        )
        .bind(team_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }
}

pub struct ProjectQueries;

impl ProjectQueries {
    pub async fn create_project(
        pool: &PgPool,
        request: &CreateProjectRequest,
        created_by: Uuid,
    ) -> Result<Project, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO projects (name, description, team_id, created_by, color)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, team_id, created_by, color, is_active, created_at, updated_at
            "#
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.team_id)
        .bind(created_by)
        .bind(&request.color)
        .fetch_one(pool)
        .await?;

        let project = Project {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            team_id: row.get("team_id"),
            created_by: row.get("created_by"),
            color: row.get("color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        // Add creator as admin
        ProjectQueries::add_project_member(pool, project.id, created_by, ProjectRole::Admin).await?;

        Ok(project)
    }

    pub async fn get_project_by_id(pool: &PgPool, project_id: Uuid) -> Result<Project, AppError> {
        let row = sqlx::query(
            "SELECT id, name, description, team_id, created_by, color, is_active, created_at, updated_at FROM projects WHERE id = $1"
        )
        .bind(project_id)
        .fetch_one(pool)
        .await?;

        let project = Project {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            team_id: row.get("team_id"),
            created_by: row.get("created_by"),
            color: row.get("color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(project)
    }

    pub async fn get_team_projects(pool: &PgPool, team_id: Uuid) -> Result<Vec<Project>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, team_id, created_by, color, is_active, created_at, updated_at
            FROM projects 
            WHERE team_id = $1 AND is_active = true
            ORDER BY name
            "#
        )
        .bind(team_id)
        .fetch_all(pool)
        .await?;

        let projects = rows.into_iter().map(|row| Project {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            team_id: row.get("team_id"),
            created_by: row.get("created_by"),
            color: row.get("color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(projects)
    }

    pub async fn get_user_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT p.id, p.name, p.description, p.team_id, p.created_by, p.color, p.is_active, p.created_at, p.updated_at
            FROM projects p
            INNER JOIN project_members pm ON p.id = pm.project_id
            WHERE pm.user_id = $1 AND p.is_active = true
            ORDER BY p.name
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let projects = rows.into_iter().map(|row| Project {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            team_id: row.get("team_id"),
            created_by: row.get("created_by"),
            color: row.get("color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(projects)
    }

    pub async fn update_project(
        pool: &PgPool,
        project_id: Uuid,
        request: &CreateProjectRequest,
    ) -> Result<Project, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE projects 
            SET name = $2, description = $3, color = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, team_id, created_by, color, is_active, created_at, updated_at
            "#
        )
        .bind(project_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.color)
        .fetch_one(pool)
        .await?;

        let project = Project {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            team_id: row.get("team_id"),
            created_by: row.get("created_by"),
            color: row.get("color"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(project)
    }

    pub async fn archive_project(pool: &PgPool, project_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE projects SET is_active = false, updated_at = NOW() WHERE id = $1"
        )
        .bind(project_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn activate_project(pool: &PgPool, project_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE projects SET is_active = true, updated_at = NOW() WHERE id = $1"
        )
        .bind(project_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_project(pool: &PgPool, project_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(project_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn add_project_member(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: ProjectRole,
    ) -> Result<ProjectMember, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO project_members (project_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING id, project_id, user_id, role, joined_at
            "#
        )
        .bind(project_id)
        .bind(user_id)
        .bind(&role)
        .fetch_one(pool)
        .await?;

        let member = ProjectMember {
            id: row.get("id"),
            project_id: row.get("project_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
            joined_at: row.get("joined_at"),
        };

        Ok(member)
    }

    pub async fn remove_project_member(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM project_members WHERE project_id = $1 AND user_id = $2")
            .bind(project_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_project_member_role(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: ProjectRole,
    ) -> Result<ProjectMember, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE project_members 
            SET role = $3
            WHERE project_id = $1 AND user_id = $2
            RETURNING id, project_id, user_id, role, joined_at
            "#
        )
        .bind(project_id)
        .bind(user_id)
        .bind(&role)
        .fetch_one(pool)
        .await?;

        let member = ProjectMember {
            id: row.get("id"),
            project_id: row.get("project_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
            joined_at: row.get("joined_at"),
        };

        Ok(member)
    }

    pub async fn get_project_members(pool: &PgPool, project_id: Uuid) -> Result<Vec<(ProjectMember, UserSummary)>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                pm.id, pm.project_id, pm.user_id, pm.role, pm.joined_at,
                u.username, u.display_name, u.avatar_url
            FROM project_members pm
            INNER JOIN users u ON pm.user_id = u.id
            WHERE pm.project_id = $1 AND u.is_active = true
            ORDER BY pm.role, u.display_name
            "#
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let members = rows.into_iter().map(|row| {
            let member = ProjectMember {
                id: row.get("id"),
                project_id: row.get("project_id"),
                user_id: row.get("user_id"),
                role: row.get("role"),
                joined_at: row.get("joined_at"),
            };

            let user = UserSummary {
                id: member.user_id,
                username: row.get("username"),
                display_name: row.get("display_name"),
                avatar_url: row.get("avatar_url"),
            };

            (member, user)
        }).collect();

        Ok(members)
    }

    pub async fn get_user_project_role(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ProjectRole>, AppError> {
        let row = sqlx::query(
            "SELECT role FROM project_members WHERE project_id = $1 AND user_id = $2"
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.get("role")))
    }

    pub async fn is_project_member(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM project_members WHERE project_id = $1 AND user_id = $2)"
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(row.get::<bool, _>("exists"))
    }
}

pub struct TaskQueries;

impl TaskQueries {
    pub async fn create_task(
        pool: &PgPool,
        project_id: Uuid,
        request: &CreateTaskRequest,
        created_by: Uuid,
    ) -> Result<Task, AppError> {
        // Get the next position for this project
        let position_row = sqlx::query(
            "SELECT COALESCE(MAX(position), 0) + 1 as next_position FROM tasks WHERE project_id = $1"
        )
        .bind(project_id)
        .fetch_one(pool)
        .await?;
        
        let position: i32 = position_row.get("next_position");
        let priority = request.priority.clone().unwrap_or(TaskPriority::Medium);

        let row = sqlx::query(
            r#"
            INSERT INTO tasks (title, description, project_id, created_by, assigned_to, priority, due_date, tags, position)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            "#
        )
        .bind(&request.title)
        .bind(&request.description)
        .bind(project_id)
        .bind(created_by)
        .bind(&request.assigned_to)
        .bind(&priority)
        .bind(&request.due_date)
        .bind(serde_json::to_value(&request.tags).unwrap_or(serde_json::Value::Array(vec![])))
        .bind(position)
        .fetch_one(pool)
        .await?;

        Ok(Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            project_id: row.get("project_id"),
            created_by: row.get("created_by"),
            assigned_to: row.get("assigned_to"),
            status: row.get("status"),
            priority: row.get("priority"),
            due_date: row.get("due_date"),
            tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_project_tasks(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            FROM tasks 
            WHERE project_id = $1 
            ORDER BY position ASC, created_at ASC
            "#
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let tasks = rows.into_iter().map(|row| Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            project_id: row.get("project_id"),
            created_by: row.get("created_by"),
            assigned_to: row.get("assigned_to"),
            status: row.get("status"),
            priority: row.get("priority"),
            due_date: row.get("due_date"),
            tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(tasks)
    }

    pub async fn get_task_by_id(
        pool: &PgPool,
        task_id: Uuid,
    ) -> Result<Task, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            FROM tasks 
            WHERE id = $1
            "#
        )
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                project_id: row.get("project_id"),
                created_by: row.get("created_by"),
                assigned_to: row.get("assigned_to"),
                status: row.get("status"),
                priority: row.get("priority"),
                due_date: row.get("due_date"),
                tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
                position: row.get("position"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::NotFound("Task not found".to_string())),
        }
    }

    pub async fn update_task(
        pool: &PgPool,
        task_id: Uuid,
        request: &UpdateTaskRequest,
    ) -> Result<Task, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE tasks 
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                assigned_to = COALESCE($4, assigned_to),
                status = COALESCE($5, status),
                priority = COALESCE($6, priority),
                due_date = COALESCE($7, due_date),
                tags = COALESCE($8, tags)
            WHERE id = $1
            RETURNING id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            "#
        )
        .bind(task_id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(&request.assigned_to)
        .bind(&request.status)
        .bind(&request.priority)
        .bind(&request.due_date)
        .bind(request.tags.as_ref().map(|tags| serde_json::to_value(tags).unwrap_or(serde_json::Value::Null)))
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                project_id: row.get("project_id"),
                created_by: row.get("created_by"),
                assigned_to: row.get("assigned_to"),
                status: row.get("status"),
                priority: row.get("priority"),
                due_date: row.get("due_date"),
                tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
                position: row.get("position"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::NotFound("Task not found".to_string())),
        }
    }

    pub async fn delete_task(
        pool: &PgPool,
        task_id: Uuid,
    ) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(task_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Task not found".to_string()));
        }

        Ok(())
    }

    pub async fn move_task(
        pool: &PgPool,
        task_id: Uuid,
        new_status: TaskStatus,
        new_position: i32,
    ) -> Result<Task, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE tasks 
            SET status = $2, position = $3
            WHERE id = $1
            RETURNING id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            "#
        )
        .bind(task_id)
        .bind(&new_status)
        .bind(new_position)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                project_id: row.get("project_id"),
                created_by: row.get("created_by"),
                assigned_to: row.get("assigned_to"),
                status: row.get("status"),
                priority: row.get("priority"),
                due_date: row.get("due_date"),
                tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
                position: row.get("position"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::NotFound("Task not found".to_string())),
        }
    }

    pub async fn get_user_assigned_tasks(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, project_id, created_by, assigned_to, status, priority, due_date, tags, position, created_at, updated_at
            FROM tasks 
            WHERE assigned_to = $1 
            ORDER BY due_date ASC NULLS LAST, priority DESC, created_at ASC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let tasks = rows.into_iter().map(|row| Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            project_id: row.get("project_id"),
            created_by: row.get("created_by"),
            assigned_to: row.get("assigned_to"),
            status: row.get("status"),
            priority: row.get("priority"),
            due_date: row.get("due_date"),
            tags: serde_json::from_value(row.get("tags")).unwrap_or(None),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(tasks)
    }
}

pub struct BoardQueries;

impl BoardQueries {
    pub async fn create_board(
        pool: &PgPool,
        project_id: Uuid,
        request: &CreateBoardRequest,
        created_by: Uuid,
    ) -> Result<Board, AppError> {
        let columns = request.columns.clone()
            .unwrap_or_else(|| vec!["Todo".to_string(), "In Progress".to_string(), "Review".to_string(), "Done".to_string()]);

        let row = sqlx::query(
            r#"
            INSERT INTO boards (name, description, project_id, created_by, columns)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, project_id, created_by, columns, is_default, created_at, updated_at
            "#
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(project_id)
        .bind(created_by)
        .bind(serde_json::to_value(&columns).unwrap())
        .fetch_one(pool)
        .await?;

        Ok(Board {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            project_id: row.get("project_id"),
            created_by: row.get("created_by"),
            columns: serde_json::from_value(row.get("columns")).unwrap_or(vec![]),
            is_default: row.get("is_default"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_project_boards(
        pool: &PgPool,
        project_id: Uuid,
    ) -> Result<Vec<Board>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, project_id, created_by, columns, is_default, created_at, updated_at
            FROM boards 
            WHERE project_id = $1 
            ORDER BY is_default DESC, created_at ASC
            "#
        )
        .bind(project_id)
        .fetch_all(pool)
        .await?;

        let boards = rows.into_iter().map(|row| Board {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            project_id: row.get("project_id"),
            created_by: row.get("created_by"),
            columns: serde_json::from_value(row.get("columns")).unwrap_or(vec![]),
            is_default: row.get("is_default"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(boards)
    }

    pub async fn get_board_by_id(
        pool: &PgPool,
        board_id: Uuid,
    ) -> Result<Board, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, project_id, created_by, columns, is_default, created_at, updated_at
            FROM boards 
            WHERE id = $1
            "#
        )
        .bind(board_id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Board {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                project_id: row.get("project_id"),
                created_by: row.get("created_by"),
                columns: serde_json::from_value(row.get("columns")).unwrap_or(vec![]),
                is_default: row.get("is_default"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::NotFound("Board not found".to_string())),
        }
    }

    pub async fn update_board(
        pool: &PgPool,
        board_id: Uuid,
        request: &UpdateBoardRequest,
    ) -> Result<Board, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE boards 
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                columns = COALESCE($4, columns)
            WHERE id = $1
            RETURNING id, name, description, project_id, created_by, columns, is_default, created_at, updated_at
            "#
        )
        .bind(board_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.columns.as_ref().map(|cols| serde_json::to_value(cols).unwrap()))
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Board {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                project_id: row.get("project_id"),
                created_by: row.get("created_by"),
                columns: serde_json::from_value(row.get("columns")).unwrap_or(vec![]),
                is_default: row.get("is_default"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::NotFound("Board not found".to_string())),
        }
    }

    pub async fn delete_board(
        pool: &PgPool,
        board_id: Uuid,
    ) -> Result<(), AppError> {
        // Check if this is the default board
        let is_default_row = sqlx::query(
            "SELECT is_default FROM boards WHERE id = $1"
        )
        .bind(board_id)
        .fetch_optional(pool)
        .await?;

        match is_default_row {
            Some(row) => {
                if row.get::<bool, _>("is_default") {
                    return Err(AppError::Validation("Cannot delete the default board".to_string()));
                }
            }
            None => return Err(AppError::NotFound("Board not found".to_string())),
        }

        let result = sqlx::query("DELETE FROM boards WHERE id = $1")
            .bind(board_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Board not found".to_string()));
        }

        Ok(())
    }
}

pub struct TaskCommentQueries;

impl TaskCommentQueries {
    pub async fn create_comment(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
        request: &CreateTaskCommentRequest,
    ) -> Result<TaskComment, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO task_comments (task_id, user_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, task_id, user_id, content, created_at, updated_at
            "#
        )
        .bind(task_id)
        .bind(user_id)
        .bind(&request.content)
        .fetch_one(pool)
        .await?;

        Ok(TaskComment {
            id: row.get("id"),
            task_id: row.get("task_id"),
            user_id: row.get("user_id"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_task_comments(
        pool: &PgPool,
        task_id: Uuid,
    ) -> Result<Vec<TaskComment>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, task_id, user_id, content, created_at, updated_at
            FROM task_comments 
            WHERE task_id = $1 
            ORDER BY created_at ASC
            "#
        )
        .bind(task_id)
        .fetch_all(pool)
        .await?;

        let comments = rows.into_iter().map(|row| TaskComment {
            id: row.get("id"),
            task_id: row.get("task_id"),
            user_id: row.get("user_id"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(comments)
    }

    pub async fn delete_comment(
        pool: &PgPool,
        comment_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM task_comments WHERE id = $1 AND user_id = $2"
        )
        .bind(comment_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Comment not found or not authorized".to_string()));
        }

        Ok(())
    }
}