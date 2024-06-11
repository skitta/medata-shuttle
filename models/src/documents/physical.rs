use crate::{DbPool, QueryResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Physical {
    pub id: i32,
    pub fever_days: i32,
    pub red_eyes: bool,
    pub lips_tongue: bool,
    pub lymph_nodes: bool,
    pub rash: bool,
    pub hands_feet: bool,
    pub relase: bool,
    pub resistance: bool,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<Physical>> {
    sqlx::query_as("SELECT * FROM physical WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}
