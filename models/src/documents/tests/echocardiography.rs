use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Echocardiography {
    pub id: i32,
    pub date: NaiveDate,
    pub rca: f32,
    pub lmca: f32,
    pub lad: f32,
    pub lcx: f32,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<Echocardiography>> {
    sqlx::query_as("SELECT * FROM echocardiography WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}
