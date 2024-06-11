use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Samples {
    pub id: i32,
    pub date: NaiveDate,
    #[serde(rename = "type")]
    pub stype: String,
    pub lable: String,
    pub status: String,
    pub note: String,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<Samples>> {
    sqlx::query_as("SELECT * FROM samples WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}
