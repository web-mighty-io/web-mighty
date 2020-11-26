use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::{Pool, PoolError};
use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Display)]
pub enum DeleteUserError {
    PoolError(PoolError),
    NoUser,
    WrongPassword,
}

impl From<PoolError> for DeleteUserError {
    fn from(e: PoolError) -> Self {
        DeleteUserError::PoolError(e)
    }
}

impl From<tokio_postgres::Error> for DeleteUserError {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::from(PoolError::from(e))
    }
}

impl ResponseError for DeleteUserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DeleteUserError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            DeleteUserError::NoUser => HttpResponse::Unauthorized().body("no user found to delete"),
            DeleteUserError::WrongPassword => HttpResponse::Unauthorized().body("incorrect password"),
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteUserForm {
    pub password_hash: String,
}

pub async fn delete_user(form: &DeleteUserForm, user_id: String, pool: &Pool) -> Result<(), DeleteUserError> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT password FROM users WHERE id=$1").await?;
    let res = client.query(&stmt, &[&user_id]).await?;
    if res.is_empty() {
        return Err(DeleteUserError::NoUser);
    }
    let password: String = res[0].get(0);
    if password != form.password_hash {
        return Err(DeleteUserError::WrongPassword);
    }
    let stmt = client.prepare("DELETE FROM users WHERE id=$1").await?;
    client.query(&stmt, &[&user_id]).await?;
    Ok(())
}
