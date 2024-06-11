use crate::{DbPool, QueryResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub weight: f32,
    pub height: f32,
}

#[derive(Deserialize)]
pub struct UpdatedPatient {
    pub name: String,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub weight: f32,
    pub height: f32,
}

pub async fn create(db: &DbPool, new: Patient) -> QueryResult<i32> {
    sqlx::query("INSERT INTO patient (id, name, birth_date, gender, weight, height) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(new.id)
        .bind(new.name)
        .bind(new.birth_date)
        .bind(new.gender)
        .bind(new.weight)
        .bind(new.height)
        .execute(db)
        .await?;
    Ok(new.id)
}

pub async fn update(db: &DbPool, id: i32, req: UpdatedPatient) -> QueryResult<()> {
    sqlx::query("UPDATE patient SET name = $1, birth_date = $2, gender = $3, weight = $4, height = $5, updated_at = Now() WHERE id = $6")
        .bind(req.name)
        .bind(req.birth_date)
        .bind(req.gender)
        .bind(req.weight)
        .bind(req.height)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
