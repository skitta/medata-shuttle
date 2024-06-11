use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Other {
    pub id: i32,
    pub date: NaiveDate,
    pub pct: f32,
    pub crp: f32,
    pub esr: f32,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<Other>> {
    sqlx::query_as("SELECT * FROM othertest WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}
