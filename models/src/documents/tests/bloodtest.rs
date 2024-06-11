use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BloodTest {
    pub id: i32,
    pub date: NaiveDate,
    pub wbc: f32,
    pub ne: f32,
    pub ly: f32,
    pub mo: f32,
    pub rbc: f32,
    pub hgb: f32,
    pub plt: f32,
}

pub async fn list_by(db: &DbPool, document_id: i32) -> QueryResult<Vec<BloodTest>> {
    sqlx::query_as("SELECT * FROM bloodtest WHERE document_id = $1")
        .bind(document_id)
        .fetch_all(db)
        .await
}

pub async fn create(db: &DbPool, document_id: i32, new: BloodTest) -> QueryResult<()> {
    sqlx::query("INSERT INTO bloodtest (id, document_id, date, wbc, ne, ly, mo, rbc, hgb, plt) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(new.id)
        .bind(document_id)
        .bind(new.date)
        .bind(new.wbc)
        .bind(new.ne)
        .bind(new.ly)
        .bind(new.mo)
        .bind(new.rbc)
        .bind(new.hgb)
        .bind(new.plt)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn update(db: &DbPool, document_id: i32, id: i32, new: BloodTest) -> QueryResult<()> {
    sqlx::query("UPDATE bloodtest SET date = $1, wbc = $2, ne = $3, ly = $4, mo = $5, rbc = $6, hgb = $7, plt = $8 WHERE document_id = $9 AND id = $10")
        .bind(new.date)
        .bind(new.wbc)
        .bind(new.ne)
        .bind(new.ly)
        .bind(new.mo)
        .bind(new.rbc)
        .bind(new.hgb)
        .bind(new.plt)
        .bind(document_id)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
