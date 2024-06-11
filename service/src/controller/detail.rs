use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::AppState;
use models::documents::physical::{self, Physical};
use models::documents::samples::{self, Samples};
use models::documents::tests::{
    bloodtest::{self, BloodTest},
    echocardiography::{self, Echocardiography},
    liverfunction::{self, LiverFunction},
    othertest::{self, Other},
};

#[derive(Serialize)]
pub struct DocumentDetail {
    physical: Vec<Physical>,
    bloodtests: Vec<BloodTest>,
    liverfunctions: Vec<LiverFunction>,
    othertests: Vec<Other>,
    echocardiographies: Vec<Echocardiography>,
    samples: Vec<Samples>,
}

pub async fn document_detail(
    State(state): State<AppState>,
    Path(document_id): Path<i32>,
) -> Result<Json<DocumentDetail>, impl IntoResponse> {
    let kd = match physical::list_by(&state.postgres, document_id).await {
        Ok(k_list) => k_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    let blood_rt = match bloodtest::list_by(&state.postgres, document_id).await {
        Ok(b_list) => b_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    let lf = match liverfunction::list_by(&state.postgres, document_id).await {
        Ok(lf_list) => lf_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    let ot = match othertest::list_by(&state.postgres, document_id).await {
        Ok(ot_list) => ot_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    let ecg = match echocardiography::list_by(&state.postgres, document_id).await {
        Ok(ecg_list) => ecg_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    let sps = match samples::list_by(&state.postgres, document_id).await {
        Ok(sps_list) => sps_list,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };

    Ok(Json(DocumentDetail {
        physical: kd,
        bloodtests: blood_rt,
        liverfunctions: lf,
        othertests: ot,
        echocardiographies: ecg,
        samples: sps,
    }))
}
