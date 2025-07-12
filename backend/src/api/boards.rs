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
    models::{CreateBoardRequest, UpdateBoardRequest, Board, TaskStatus, UserSummary},
    queries::{BoardQueries, ProjectQueries, TaskQueries, UserQueries}
};
use crate::utils::errors::AppError;
use crate::utils::validation;
use crate::websocket::events::{WebSocketEvent, BoardEventData};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardWithTasks {
    #[serde(flatten)]
    pub board: Board,
    pub tasks: Vec<crate::database::models::Task>,
}

pub async fn create_board(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<CreateBoardRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project member (at least editor role required)
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    if !matches!(user_role, Some(ProjectRole::Admin) | Some(ProjectRole::Editor)) {
        return Err(AppError::Forbidden("Need editor or admin role to create boards".to_string()));
    }

    // Validate input
    validation::validate_board_name(&request.name)?;
    if let Some(ref description) = request.description {
        validation::validate_board_description(description)?;
    }

    let board = BoardQueries::create_board(
        app_state.database.pool(),
        project_id,
        &request,
        current_user.id(),
    ).await?;

    // Broadcast board creation to WebSocket subscribers
    let user = UserQueries::get_user_by_id(app_state.database.pool(), current_user.id()).await?;
    let user_summary: UserSummary = user.into();
    
    let event = WebSocketEvent::BoardCreated(BoardEventData {
        board: board.clone(),
        project_id,
        user: user_summary,
    });
    
    app_state.websocket.broadcast_to_project(project_id, event, Some(current_user.id())).await;

    Ok((StatusCode::CREATED, Json(board)))
}

pub async fn get_project_boards(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Must be a project member to view boards".to_string()));
    }

    let boards = BoardQueries::get_project_boards(app_state.database.pool(), project_id).await?;

    Ok(Json(boards))
}

pub async fn get_board_details(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(board_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let board = BoardQueries::get_board_by_id(app_state.database.pool(), board_id).await?;

    // Check if user is project member
    if !ProjectQueries::is_project_member(app_state.database.pool(), board.project_id, current_user.id()).await? {
        return Err(AppError::Forbidden("Not a project member".to_string()));
    }

    // Get all tasks for this project to show on the board
    let tasks = TaskQueries::get_project_tasks(app_state.database.pool(), board.project_id).await?;

    let board_with_tasks = BoardWithTasks {
        board,
        tasks,
    };

    Ok(Json(board_with_tasks))
}

pub async fn update_board(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(board_id): Path<Uuid>,
    Json(request): Json<UpdateBoardRequest>,
) -> Result<impl IntoResponse, AppError> {
    let board = BoardQueries::get_board_by_id(app_state.database.pool(), board_id).await?;

    // Check if user is project member (at least editor role required)
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        board.project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    if !matches!(user_role, Some(ProjectRole::Admin) | Some(ProjectRole::Editor)) {
        return Err(AppError::Forbidden("Need editor or admin role to update boards".to_string()));
    }

    // Validate input
    if let Some(ref name) = request.name {
        validation::validate_board_name(name)?;
    }
    if let Some(ref description) = request.description {
        validation::validate_board_description(description)?;
    }

    let updated_board = BoardQueries::update_board(app_state.database.pool(), board_id, &request).await?;

    // Broadcast board update to WebSocket subscribers
    let user = UserQueries::get_user_by_id(app_state.database.pool(), current_user.id()).await?;
    let user_summary: UserSummary = user.into();
    
    let event = WebSocketEvent::BoardUpdated(BoardEventData {
        board: updated_board.clone(),
        project_id: board.project_id,
        user: user_summary,
    });
    
    app_state.websocket.broadcast_to_project(board.project_id, event, Some(current_user.id())).await;

    Ok(Json(updated_board))
}

pub async fn delete_board(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(board_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let board = BoardQueries::get_board_by_id(app_state.database.pool(), board_id).await?;

    // Check if user is project admin
    let user_role = ProjectQueries::get_user_project_role(
        app_state.database.pool(),
        board.project_id,
        current_user.id(),
    ).await?;

    use crate::database::models::ProjectRole;
    if !matches!(user_role, Some(ProjectRole::Admin)) {
        return Err(AppError::Forbidden("Only project admins can delete boards".to_string()));
    }

    BoardQueries::delete_board(app_state.database.pool(), board_id).await?;

    // Broadcast board deletion to WebSocket subscribers
    let event = WebSocketEvent::BoardDeleted { 
        board_id, 
        project_id: board.project_id 
    };
    
    app_state.websocket.broadcast_to_project(board.project_id, event, Some(current_user.id())).await;

    Ok(StatusCode::NO_CONTENT)
}