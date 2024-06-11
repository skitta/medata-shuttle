use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::Serialize;

use crate::AppState;
use models::documents::{
    document,
    enrollgroup::{self, EnrollGroup},
};

// 定义响应体
#[derive(Serialize)]
pub struct DocumentView {
    id: i32,
    p_id: i32,
    p_name: String,
    p_age: i64,
    in_data: NaiveDate,
    group: String,
}

pub async fn document_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<DocumentView>>, impl IntoResponse> {
    match document::list(&state.postgres).await {
        Ok(d_list) => {
            let mut resp = Vec::new();
            for d in d_list {
                let age = d
                    .enroll_date
                    .signed_duration_since(d.p_birth_date)
                    .num_days();
                let tmp = DocumentView {
                    id: d.id,
                    p_id: d.p_id,
                    p_name: d.p_name,
                    p_age: age,
                    in_data: d.enroll_date,
                    group: d.group,
                };
                resp.push(tmp);
            }
            Ok(Json(resp))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    }
}

pub async fn group_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<EnrollGroup>>, impl IntoResponse> {
    match enrollgroup::list(&state.postgres).await {
        Ok(g_list) => Ok(Json(g_list)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    }
}
