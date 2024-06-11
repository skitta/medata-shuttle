use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::AppState;
use models::documents::participant::{self, UpdatedPatient};

pub async fn update_patient(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatedPatient>,
) -> Result<StatusCode, impl IntoResponse> {
    match participant::update(&state.postgres, id, req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    }
}
