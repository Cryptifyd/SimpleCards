use axum::{
    extract::{Extension, State, Path},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::middleware::CurrentUser;
use crate::database::{
    models::{CreateProjectRequest, ProjectRole, ProjectMember, UserSummary},
    queries::{ProjectQueries, TeamQueries, UserQueries}
};
use crate::utils::errors::AppError;
use crate::utils::validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddProjectMemberRequest {
    pub user_id: Uuid,
    pub role: ProjectRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectMemberRequest {
    pub role: ProjectRole,
}

#[derive(Debug, Serialize)]
pub struct ProjectDetailsResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_id: Uuid,
    pub created_by: Uuid,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub members: Vec<ProjectMemberResponse>,
}

#[derive(Debug, Serialize)]
pub struct ProjectMemberResponse {
    pub id: Uuid,
    pub user: UserSummary,
    pub role: ProjectRole,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_project(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
    Json(mut request): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Set team_id from path
    request.team_id = team_id;

    // Check if user is team member
    if !TeamQueries::is_team_member(app_state.database.pool(), team_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a team member to create projects".to_string()));
    }

    // Validate input
    validation::validate_project_name(&request.name)?;
    if let Some(ref description) = request.description {
        validation::validate_project_description(description)?;
    }
    if let Some(ref color) = request.color {
        validation::validate_hex_color(color)?;
    }

    let project = ProjectQueries::create_project(
        app_state.database.pool(),
        &request,
        current_user.id(),
    ).await?;

    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn get_team_projects(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team member
    if !TeamQueries::is_team_member(app_state.database.pool(), team_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a team member to view projects".to_string()));
    }

    let projects = ProjectQueries::get_team_projects(app_state.database.pool(), team_id).await?;

    Ok(Json(projects))
}

pub async fn get_user_projects(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<impl IntoResponse, AppError> {
    let projects = ProjectQueries::get_user_projects(
        app_state.database.pool(),
        current_user.id(),
    ).await?;

    Ok(Json(projects))
}

pub async fn get_project_details(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Not a project member".to_string()));
    }

    let project = ProjectQueries::get_project_by_id(app_state.database.pool(), project_id).await?;
    let members_data = ProjectQueries::get_project_members(app_state.database.pool(), project_id).await?;

    let members = members_data.into_iter().map(|(member, user)| ProjectMemberResponse {
        id: member.id,
        user,
        role: member.role,
        joined_at: member.joined_at,
    }).collect();

    let response = ProjectDetailsResponse {
        id: project.id,
        name: project.name,
        description: project.description,
        team_id: project.team_id,
        created_by: project.created_by,
        color: project.color,
        is_active: project.is_active,
        created_at: project.created_at,
        updated_at: project.updated_at,
        members,
    };

    Ok(Json(response))
}

pub async fn update_project(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can update projects".to_string()));
    }

    // Validate input
    validation::validate_project_name(&request.name)?;
    if let Some(ref description) = request.description {
        validation::validate_project_description(description)?;
    }
    if let Some(ref color) = request.color {
        validation::validate_hex_color(color)?;
    }

    let project = ProjectQueries::update_project(app_state.database.pool(), project_id, &request).await?;

    Ok(Json(project))
}

pub async fn archive_project(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can archive projects".to_string()));
    }

    ProjectQueries::archive_project(app_state.database.pool(), project_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn activate_project(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can activate projects".to_string()));
    }

    ProjectQueries::activate_project(app_state.database.pool(), project_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_project(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can delete projects".to_string()));
    }

    ProjectQueries::delete_project(app_state.database.pool(), project_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_project_member(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<AddProjectMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can add members".to_string()));
    }

    // Get project to check team membership
    let project = ProjectQueries::get_project_by_id(app_state.database.pool(), project_id).await?;
    
    // Check if target user is a team member
    if !TeamQueries::is_team_member(app_state.database.pool(), project.team_id, request.user_id).await? {
        return Err(AppError::Validation("User must be a team member to join project".to_string()));
    }

    // Check if user is already a project member
    if ProjectQueries::is_project_member(app_state.database.pool(), project_id, request.user_id).await? {
        return Err(AppError::Conflict("User is already a project member".to_string()));
    }

    let member = ProjectQueries::add_project_member(
        app_state.database.pool(),
        project_id,
        request.user_id,
        request.role,
    ).await?;

    Ok((StatusCode::CREATED, Json(member)))
}

pub async fn remove_project_member(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path((project_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin or removing themselves
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    let is_admin = matches!(user_role, Some(ProjectRole::Admin));
    let is_self = current_user.id() == user_id;

    if !is_admin && !is_self {
        return Err(AppError::Forbidden("Can only remove yourself or be project admin".to_string()));
    }

    // Prevent removing the last admin
    if user_id == current_user.id() && is_admin {
        let members = ProjectQueries::get_project_members(app_state.database.pool(), project_id).await?;
        let admin_count = members.iter().filter(|(member, _)| matches!(member.role, ProjectRole::Admin)).count();
        
        if admin_count <= 1 {
            return Err(AppError::Validation("Cannot remove the last admin from project".to_string()));
        }
    }

    ProjectQueries::remove_project_member(app_state.database.pool(), project_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_project_member_role(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path((project_id, user_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateProjectMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can update member roles".to_string()));
    }

    // Prevent demoting the last admin
    if !matches!(request.role, ProjectRole::Admin) {
        let members = ProjectQueries::get_project_members(app_state.database.pool(), project_id).await?;
        let admin_count = members.iter().filter(|(member, _)| matches!(member.role, ProjectRole::Admin)).count();
        
        // Check if this user is currently an admin and would be the last one
        let target_member = members.iter().find(|(member, _)| member.user_id == user_id);
        if let Some((member, _)) = target_member {
            if matches!(member.role, ProjectRole::Admin) && admin_count <= 1 {
                return Err(AppError::Validation("Cannot demote the last admin".to_string()));
            }
        }
    }

    let member = ProjectQueries::update_project_member_role(
        app_state.database.pool(),
        project_id,
        user_id,
        request.role,
    ).await?;

    Ok(Json(member))
}