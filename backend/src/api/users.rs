use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    Json,
};

use crate::auth::middleware::CurrentUser;
use crate::database::{connection::Database, models::{UpdateUserRequest, UserSummary}, queries::UserQueries};
use crate::utils::errors::AppError;

pub async fn get_current_user(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserQueries::get_user_by_id(app_state.database.pool(), current_user.id()).await?;
    let user_summary: UserSummary = user.into();
    Ok(Json(user_summary))
}

pub async fn update_current_user(
    State(app_state): State<crate::AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate display name if provided
    if let Some(ref display_name) = request.display_name {
        crate::utils::validation::validate_display_name(display_name)?;
    }

    let updated_user = UserQueries::update_user(app_state.database.pool(), current_user.id(), &request).await?;
    let user_summary: UserSummary = updated_user.into();
    Ok(Json(user_summary))
}