use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct LiverFunction {
    pub id: i32,
    pub date: NaiveDate,
    pub ast: f32,
    pub alt: f32,
    pub alb: f32,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<LiverFunction>> {
    sqlx::query_as("SELECT * FROM liverfunction WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}
