use crate::db::{Error, Result, TOKEN_VALID_DURATION};
use actix_web::http::StatusCode;
use deadpool_postgres::{Pool, };
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct AddUserForm {
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub token: Uuid,
}

pub async fn add_user(form: &AddUserForm, pool: &Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT 1 generated_time, email FROM pre_users WHERE id=$1 AND token=$2;")
        .await?;
    let res = client.query(&stmt, &[&form.user_id, &form.token]).await?;
    let row = res
        .first()
        .ok_or_else(|| Error::new(StatusCode::UNAUTHORIZED, "login failed"))?;
    let time: SystemTime = row.get(0);
    if time.elapsed().unwrap_or_else(|_| Duration::from_secs(0)) >= TOKEN_VALID_DURATION {
        return Error::result(StatusCode::UNAUTHORIZED, "token expired");
    }
    let email: String = row.get(1);

    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4);")
        .await?;
    let _ = client
        .query(&stmt, &[&form.user_id, &form.username, &email, &form.password])
        .await?;

    let client = pool.get().await?;
    let stmt = client.prepare("DELETE FROM pre_users WHERE id=$1;").await?;
    let _ = client.query(&stmt, &[&form.user_id]).await?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct CheckIdForm {
    pub user_id: String,
}

pub async fn check_id(form: &CheckIdForm, pool: &Pool) -> Result<bool> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT 1 FROM ( SELECT id FROM pre_users UNION ALL SELECT id FROM users) a WHERE id=$1;")
        .await?;
    let res = client.query(&stmt, &[&form.user_id]).await?;
    Ok(!res.is_empty())
}

#[derive(Deserialize, Serialize)]
pub struct CheckEmailForm {
    pub email: String,
}

pub async fn check_email(form: &CheckEmailForm, pool: &Pool) -> Result<bool> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT 1 email FROM email WHERE email=$1;").await?;
    let res = client.query(&stmt, &[&form.email]).await?;
    Ok(!res.is_empty())
}

#[derive(Deserialize, Serialize)]
pub struct DeleteForm {
    pub password_hash: String,
}

pub async fn delete(user_no: u32, form: &DeleteForm, pool: &Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT 1 number FROM users WHERE number=$1 AND password=$2;")
        .await?;
    let res = client.query(&stmt, &[&user_no, &form.password_hash]).await?;
    if res.is_empty() {
        return Error::result(StatusCode::UNAUTHORIZED, "password doesn't match");
    }
    let stmt = client.prepare("DELETE FROM users WHERE number=$1").await?;
    client.query(&stmt, &[&user_no]).await?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub user_id: String,
    pub password_hash: String,
}

pub async fn login(form: &LoginForm, pool: &Pool) -> Result<u32> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT 1 number FROM users WHERE id=$1 AND password=$2;")
        .await?;
    let res = client.query(&stmt, &[&form.user_id, &form.password_hash]).await?;
    let row = res
        .first()
        .ok_or_else(|| Error::new(StatusCode::UNAUTHORIZED, "login failed"))?;
    Ok(row.get(0))
}

#[derive(Deserialize, Serialize)]
pub struct RegenerateTokenForm {
    pub user_id: String,
    pub email: String,
}

pub async fn regenerate_token(form: &RegenerateTokenForm, pool: &Pool) -> Result<Uuid> {
    let client = pool.get().await?;
    let stmt = client.prepare("UPDATE pre_users SET token = UUID_GENERATE_V4(), generated_time = NOW() WHERE id=$1 AND email=$2 RETURNING token;").await?;
    let res = client.query(&stmt, &[&form.user_id, &form.email]).await?;
    let row = res
        .first()
        .ok_or_else(|| Error::new(StatusCode::UNAUTHORIZED, "login failed"))?;
    Ok(row.get(0))
}

#[derive(Deserialize, Serialize)]
pub struct RegisterForm {
    pub user_id: String,
    pub email: String,
}

pub async fn register(form: &RegisterForm, pool: &Pool) -> Result<()> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("SELECT 1 FROM ( SELECT id FROM pre_users UNION ALL SELECT id FROM users) a WHERE id=$1;")
        .await?;
    let res = client.query(&stmt, &[&form.user_id]).await?;
    if !res.is_empty() {
        return Error::result(StatusCode::UNAUTHORIZED, "username already in use");
    }

    let client = pool.get().await?;
    let stmt = client
        .prepare("INSERT INTO pre_users (id, email) VALUES ($1, $2);")
        .await?;
    let _ = client.query(&stmt, &[&form.user_id, &form.email]).await?;
    Ok(())
}
