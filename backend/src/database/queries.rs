use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::database::models::{
    User, CreateUserRequest, UpdateUserRequest,
    Team, CreateTeamRequest, TeamMember, TeamRole,
    Project, CreateProjectRequest, ProjectMember, ProjectRole, UserSummary
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