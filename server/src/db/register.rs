use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::{Pool, PoolError};
use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Display)]
pub enum RegisterError {
    PoolError(PoolError),
    InvalidUsername,
    UserIdExist,
    InvalidPassword,
}

impl From<PoolError> for RegisterError {
    fn from(e: PoolError) -> Self {
        RegisterError::PoolError(e)
    }
}

impl From<tokio_postgres::Error> for RegisterError {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::from(PoolError::from(e))
    }
}

impl ResponseError for RegisterError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            RegisterError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            RegisterError::InvalidUsername => HttpResponse::BadRequest().body("username not allowed"),
            RegisterError::UserIdExist => HttpResponse::Conflict().body("userid exists"),
            RegisterError::InvalidPassword => HttpResponse::BadRequest().body("password is not allowed"),
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub user_id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

pub async fn register(form: &RegisterForm, pool: &Pool) -> Result<(), RegisterError> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT id FROM users WHERE id=$1").await?;
    let res = client.query(&stmt, &[&form.user_id]).await?;
    if !res.is_empty() {
        return Err(RegisterError::UserIdExist);
    }

    let client = pool.get().await?;
    let stmt = client
        .prepare(
            "INSERT INTO users (id, name, password, email)
            VALUES ($1, $2, $3, $4);",
        )
        .await?;
    let _ = client
        .query(
            &stmt,
            &[&form.user_id, &form.username, &form.password_hash, &form.email],
        )
        .await?;
    Ok(())
}
