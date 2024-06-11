use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::AppState;
use models::documents::participant::{self, Patient};
use models::documents::tests::bloodtest::{self, BloodTest};

pub async fn add_patient(
    State(state): State<AppState>,
    Json(patient): Json<Patient>,
) -> Result<StatusCode, impl IntoResponse> {
    match participant::create(&state.postgres, patient).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    }
}

pub async fn add_bloodtest(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(bloodtest): Json<BloodTest>,
) -> Result<StatusCode, impl IntoResponse> {
    match bloodtest::create(&state.postgres, id, bloodtest).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    }
}
