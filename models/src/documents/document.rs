use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub participant_id: i32,
    pub group_id: i32,
    pub in_data: NaiveDate,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct DocumentList {
    pub id: i32,
    pub p_id: i32,
    pub p_name: String,
    pub p_birth_date: NaiveDate,
    pub enroll_date: NaiveDate,
    pub group: String,
}

pub async fn create(db: &DbPool, new: Document) -> QueryResult<()> {
    sqlx::query(
        "INSERT INTO document (id, participant_id, group_id, in_data) VALUES ($1, $2, $3, $4)",
    )
    .bind(new.id)
    .bind(new.participant_id)
    .bind(new.group_id)
    .bind(new.in_data)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn list(db: &DbPool) -> QueryResult<Vec<DocumentList>> {
    sqlx::query_as::<_, DocumentList>("SELECT d.id, p.id AS p_id, p.name AS p_name, p.birth_date AS p_birth_date, d.enroll_date, g.name AS group FROM participant p INNER JOIN document d ON p.id = d.participant_id INNER JOIN enrollgroup g ON d.group_id = g.id")
        .fetch_all(db)
        .await
}
