use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::middleware::CurrentUser;
use crate::database::{
    models::{CreateTaskRequest, UpdateTaskRequest, Task, MoveTaskRequest, TaskStatus, TaskPriority},
    queries::{TaskQueries, ProjectQueries}
};
use crate::utils::errors::AppError;
use crate::utils::validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFilters {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub assigned_to: Option<Uuid>,
    pub tag: Option<String>,
}

pub async fn create_task(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a project member to create tasks".to_string()));
    }

    // Validate input
    validation::validate_task_title(&request.title)?;
    if let Some(ref description) = request.description {
        validation::validate_task_description(description)?;
    }

    // Validate assigned user is a project member if provided
    if let Some(assigned_to) = request.assigned_to {
        if !ProjectQueries::is_project_member(app_state.database.pool(), project_id, assigned_to).await? {
            return Err(AppError::Validation("Assigned user must be a project member".to_string()));
        }
    }

    let task = TaskQueries::create_task(
        app_state.database.pool(),
        project_id,
        &request,
        current_user.id(),
    ).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn get_project_tasks(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
    Query(filters): Query<TaskFilters>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a project member to view tasks".to_string()));
    }

    let mut tasks = TaskQueries::get_project_tasks(app_state.database.pool(), project_id).await?;

    // Apply filters
    if let Some(status) = filters.status {
        tasks.retain(|task| task.status == status);
    }
    if let Some(priority) = filters.priority {
        tasks.retain(|task| task.priority == priority);
    }
    if let Some(assigned_to) = filters.assigned_to {
        tasks.retain(|task| task.assigned_to == Some(assigned_to));
    }
    if let Some(tag) = filters.tag {
        tasks.retain(|task| {
            task.tags.as_ref()
                .map(|tags| tags.contains(&tag))
                .unwrap_or(false)
        });
    }

    Ok(Json(tasks))
}

pub async fn get_task_details(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), task.project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Not a project member".to_string()));
    }

    Ok(Json(task))
}

pub async fn update_task(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
    Json(request): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project member (at least editor role required for updates)
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        task.project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    if !matches!(user_role, Some(ProjectRole::Admin) | Some(ProjectRole::Editor)) {
        return Err(AppError::Forbidden("Need editor or admin role to update tasks".to_string()));
    }

    // Validate input
    if let Some(ref title) = request.title {
        validation::validate_task_title(title)?;
    }
    if let Some(ref description) = request.description {
        validation::validate_task_description(description)?;
    }

    // Validate assigned user is a project member if provided
    if let Some(assigned_to) = request.assigned_to {
        if !ProjectQueries::is_project_member(app_state.database.pool(), task.project_id, assigned_to).await? {
            return Err(AppError::Validation("Assigned user must be a project member".to_string()));
        }
    }

    let updated_task = TaskQueries::update_task(app_state.database.pool(), task_id, &request).await?;

    Ok(Json(updated_task))
}

pub async fn delete_task(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project admin or task creator
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        task.project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    let is_admin = matches!(user_role, Some(ProjectRole::Admin));
    let is_task_creator = task.created_by == current_user.id();

    if !is_admin && !is_task_creator {
        return Err(AppError::Forbidden("Only project admins or task creators can delete tasks".to_string()));
    }

    TaskQueries::delete_task(app_state.database.pool(), task_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn move_task(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
    Json(request): Json<MoveTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project member (at least editor role required)
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        task.project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    if !matches!(user_role, Some(ProjectRole::Admin) | Some(ProjectRole::Editor)) {
        return Err(AppError::Forbidden("Need editor or admin role to move tasks".to_string()));
    }

    let updated_task = TaskQueries::move_task(
        app_state.database.pool(),
        task_id,
        request.status,
        request.position,
    ).await?;

    Ok(Json(updated_task))
}

pub async fn get_user_assigned_tasks(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<impl IntoResponse, AppError> {
    let tasks = TaskQueries::get_user_assigned_tasks(
        app_state.database.pool(),
        current_user.id(),
    ).await?;

    Ok(Json(tasks))
}