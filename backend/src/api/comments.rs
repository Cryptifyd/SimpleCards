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
    models::{CreateTaskCommentRequest, TaskComment, UserSummary},
    queries::{TaskCommentQueries, TaskQueries, ProjectQueries, UserQueries}
};
use crate::utils::errors::AppError;
use crate::utils::validation;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCommentResponse {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user: UserSummary,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_task_comment(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
    Json(request): Json<CreateTaskCommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), task.project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a project member to comment on tasks".to_string()));
    }

    // Validate input
    validation::validate_task_comment(&request.content)?;

    let comment = TaskCommentQueries::create_comment(
        app_state.database.pool(),
        task_id,
        current_user.id(),
        &request,
    ).await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn get_task_comments(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = TaskQueries::get_task_by_id(app_state.database.pool(), task_id).await?;

    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), task.project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a project member to view task comments".to_string()));
    }

    let comments = TaskCommentQueries::get_task_comments(app_state.database.pool(), task_id).await?;

    // Fetch user details for each comment
    let mut comment_responses = Vec::new();
    for comment in comments {
        let user = UserQueries::get_user_by_id(app_state.database.pool(), comment.user_id).await?;
        comment_responses.push(TaskCommentResponse {
            id: comment.id,
            task_id: comment.task_id,
            user: user.into(),
            content: comment.content,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        });
    }

    Ok(Json(comment_responses))
}

pub async fn delete_task_comment(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(comment_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // The delete_comment function already checks if the user owns the comment
    TaskCommentQueries::delete_comment(
        app_state.database.pool(),
        comment_id,
        current_user.id(),
    ).await?;

    Ok(StatusCode::NO_CONTENT)
}