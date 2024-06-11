pub mod auth;
pub mod documents;

type QueryResult<T> = Result<T, sqlx::Error>;

pub type DbPool = sqlx::PgPool;

pub async fn init_db(db: &DbPool) {
    sqlx::migrate!("./migrations")
        .run(db)
        .await
        .expect("Had some errors running migrations :(")
}
