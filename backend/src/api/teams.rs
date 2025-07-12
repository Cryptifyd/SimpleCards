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
    connection::Database,
    models::{CreateTeamRequest, TeamRole, TeamMember, UserSummary},
    queries::{TeamQueries, UserQueries}
};
use crate::utils::errors::AppError;
use crate::utils::validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTeamMemberRequest {
    pub user_id: Uuid,
    pub role: TeamRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamMemberRequest {
    pub role: TeamRole,
}

#[derive(Debug, Serialize)]
pub struct TeamDetailsResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub members: Vec<TeamMemberResponse>,
}

#[derive(Debug, Serialize)]
pub struct TeamMemberResponse {
    pub id: Uuid,
    pub user: UserSummary,
    pub role: TeamRole,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_team(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    validation::validate_team_name(&request.name)?;
    if let Some(ref description) = request.description {
        validation::validate_team_description(description)?;
    }

    let team = TeamQueries::create_team(
        app_state.database.pool(),
        &request,
        current_user.id(),
    ).await?;

    Ok((StatusCode::CREATED, Json(team)))
}

pub async fn get_user_teams(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<impl IntoResponse, AppError> {
    let teams = TeamQueries::get_user_teams(
        app_state.database.pool(),
        current_user.id(),
    ).await?;

    Ok(Json(teams))
}

pub async fn get_team_details(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team member
    if !TeamQueries::is_team_member(app_state.database.pool(), team_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Not a team member".to_string()));
    }

    let team = TeamQueries::get_team_by_id(app_state.database.pool(), team_id).await?;
    let members_data = TeamQueries::get_team_members(app_state.database.pool(), team_id).await?;

    let members = members_data.into_iter().map(|(member, user)| TeamMemberResponse {
        id: member.id,
        user,
        role: member.role,
        joined_at: member.joined_at,
    }).collect();

    let response = TeamDetailsResponse {
        id: team.id,
        name: team.name,
        description: team.description,
        created_by: team.created_by,
        created_at: team.created_at,
        updated_at: team.updated_at,
        members,
    };

    Ok(Json(response))
}

pub async fn update_team(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
    Json(request): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team admin
    let user_role = TeamQueries::get_user_team_role(
        app_state.database.pool(),
        team_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(TeamRole::Admin)) {
        return Err(AppError::Forbidden("Only team admins can update teams".to_string()));
    }

    // Validate input
    validation::validate_team_name(&request.name)?;
    if let Some(ref description) = request.description {
        validation::validate_team_description(description)?;
    }

    let team = TeamQueries::update_team(app_state.database.pool(), team_id, &request).await?;

    Ok(Json(team))
}

pub async fn delete_team(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team admin
    let user_role = TeamQueries::get_user_team_role(
        app_state.database.pool(),
        team_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(TeamRole::Admin)) {
        return Err(AppError::Forbidden("Only team admins can delete teams".to_string()));
    }

    TeamQueries::delete_team(app_state.database.pool(), team_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_team_member(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(team_id): Path<Uuid>,
    Json(request): Json<AddTeamMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team admin
    let user_role = TeamQueries::get_user_team_role(
        app_state.database.pool(),
        team_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(TeamRole::Admin)) {
        return Err(AppError::Forbidden("Only team admins can add members".to_string()));
    }

    // Check if target user exists
    UserQueries::get_user_by_id(app_state.database.pool(), request.user_id).await?;

    // Check if user is already a member
    if TeamQueries::is_team_member(app_state.database.pool(), team_id, request.user_id).await? {
        return Err(AppError::Conflict("User is already a team member".to_string()));
    }

    let member = TeamQueries::add_team_member(
        app_state.database.pool(),
        team_id,
        request.user_id,
        request.role,
    ).await?;

    Ok((StatusCode::CREATED, Json(member)))
}

pub async fn remove_team_member(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team admin or removing themselves
    let user_role = TeamQueries::get_user_team_role(
        app_state.database.pool(),
        team_id,
        current_user.id(),
    ).await?;

    let is_admin = matches!(user_role, Some(TeamRole::Admin));
    let is_self = current_user.id() == user_id;

    if !is_admin && !is_self {
        return Err(AppError::Forbidden("Can only remove yourself or be team admin".to_string()));
    }

    // Prevent removing the last admin
    if user_id == current_user.id() && is_admin {
        let members = TeamQueries::get_team_members(app_state.database.pool(), team_id).await?;
        let admin_count = members.iter().filter(|(member, _)| matches!(member.role, TeamRole::Admin)).count();
        
        if admin_count <= 1 {
            return Err(AppError::Validation("Cannot remove the last admin from team".to_string()));
        }
    }

    TeamQueries::remove_team_member(app_state.database.pool(), team_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_team_member_role(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateTeamMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is team admin
    let user_role = TeamQueries::get_user_team_role(
        app_state.database.pool(),
        team_id,
        current_user.id(),
    ).await?;

    if !matches!(user_role, Some(TeamRole::Admin)) {
        return Err(AppError::Forbidden("Only team admins can update member roles".to_string()));
    }

    // Prevent demoting the last admin
    if matches!(request.role, TeamRole::Member) {
        let members = TeamQueries::get_team_members(app_state.database.pool(), team_id).await?;
        let admin_count = members.iter().filter(|(member, _)| matches!(member.role, TeamRole::Admin)).count();
        
        // Check if this user is currently an admin and would be the last one
        let target_member = members.iter().find(|(member, _)| member.user_id == user_id);
        if let Some((member, _)) = target_member {
            if matches!(member.role, TeamRole::Admin) && admin_count <= 1 {
                return Err(AppError::Validation("Cannot demote the last admin".to_string()));
            }
        }
    }

    let member = TeamQueries::update_team_member_role(
        app_state.database.pool(),
        team_id,
        user_id,
        request.role,
    ).await?;

    Ok(Json(member))
}