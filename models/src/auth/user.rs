use crate::{DbPool, QueryResult};
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct RegisterUser {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    name: String,
    password: String,
}

pub async fn create(db: &DbPool, newuser: RegisterUser) -> QueryResult<()> {
    let hashed_password = bcrypt::hash(newuser.password, 10).unwrap();
    // 执行创建用户的SQL语句
    sqlx::query("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)")
        .bind(newuser.name)
        .bind(newuser.email)
        .bind(hashed_password)
        .execute(db)
        .await
        .map(|_| ())
}

pub async fn verify(db: &DbPool, loginuser: LoginUser) -> Result<i32, String> {
    let user = sqlx::query("SELECT * FROM users WHERE name = $1")
        .bind(&loginuser.name)
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;

    let check_password =
        bcrypt::verify(&loginuser.password, user.get("password")).map_err(|e| e.to_string())?;

    if check_password {
        Ok(user.get::<i32, _>("id"))
    } else {
        Err("Invalid password".into())
    }
}
