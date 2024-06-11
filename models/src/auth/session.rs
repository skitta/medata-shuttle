use crate::{DbPool, QueryResult};

pub async fn create(db: &DbPool, user_id: &i32) -> QueryResult<String> {
    let session_id = rand::random::<u64>().to_string();

    sqlx::query("INSERT INTO sessions (session_id, user_id) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET session_id = EXCLUDED.session_id")
        .bind(&session_id)
        .bind(user_id)
        .execute(db)
        .await?;

    Ok(session_id)
}

pub async fn find(db: &DbPool, session_id: String) -> QueryResult<()> {
    sqlx::query("SELECT * FROM sessions WHERE session_id = $1 AND expires > CURRENT_TIMESTAMP")
        .bind(session_id)
        .execute(db)
        .await
        .map(|_| ())
}

pub async fn delete(db: &DbPool, session_id: String) -> QueryResult<()> {
    sqlx::query("DELETE FROM sessions WHERE session_id = $1")
        .bind(session_id)
        .execute(db)
        .await
        .map(|_| ())
}
