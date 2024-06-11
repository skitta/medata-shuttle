use crate::{DbPool, QueryResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct EnrollGroup {
    id: i32,
    name: String,
    description: String,
}

pub async fn list(db: &DbPool) -> QueryResult<Vec<EnrollGroup>> {
    sqlx::query_as::<_, EnrollGroup>("SELECT * FROM enrollgroup")
        .fetch_all(db)
        .await
}
