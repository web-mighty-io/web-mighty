use crate::dev::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct AddUserForm {
    pub user_id: String,
    pub name: String,
    pub password: String,
    pub token: Uuid,
}

pub fn add_user(form: AddUserForm, pool: Pool) -> Result<()> {
    let _ = is_user_id_valid(&form.user_id);
    let _ = is_user_name_valid(&form.name);
    let _ = is_password_valid(&form.password, &form.user_id);
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 gen_time, email FROM pre_users WHERE id=$1 AND token=$2;")?;
    let res = client.query(&stmt, &[&form.user_id, &form.token])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "login failed"))?;
    let time: SystemTime = row.get(0);
    ensure!(
        time.elapsed()? <= TOKEN_VALID_DURATION,
        StatusCode::UNAUTHORIZED,
        "token expired"
    );
    let email: String = row.get(1);

    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4);")?;
    let _ = client.query(&stmt, &[&form.user_id, &form.name, &email, &form.password])?;

    let mut client = pool.get()?;
    let stmt = client.prepare("DELETE FROM pre_users WHERE id=$1;")?;
    let _ = client.query(&stmt, &[&form.user_id])?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChangeInfoForm {
    pub user_no: u32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub new_password: Option<String>,
}

pub fn change_user_info(form: ChangeInfoForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 name, email id FROM users WHERE no=$1 AND password=$2;")?;
    let res = client.query(&stmt, &[&form.user_no, &form.password])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "login failed"))?;
    let username = form.name.clone().unwrap_or_else(|| row.get(0));
    let email = form.email.clone().unwrap_or_else(|| row.get(1));
    let password = form.new_password.clone().unwrap_or_else(|| form.password.clone());
    let user_id: String = row.get(2);

    let _ = is_user_name_valid(&username);
    let _ = is_password_valid(&password, &user_id);
    let _ = is_email_valid(&email);

    let mut client = pool.get()?;
    let stmt = client.prepare("UPDATE users SET name=$1, email=$2, password=$3 WHERE no=$4;")?;
    let _ = client.query(&stmt, &[&username, &email, &password, &form.user_no])?;

    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CheckIdForm {
    pub user_id: String,
}

pub fn check_user_id(form: CheckIdForm, pool: Pool) -> Result<bool> {
    let mut client = pool.get()?;
    let stmt =
        client.prepare("SELECT 1 FROM ( SELECT id FROM pre_users UNION ALL SELECT id FROM users) a WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.user_id])?;
    Ok(!res.is_empty())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CheckEmailForm {
    pub email: String,
}

pub fn check_user_email(form: CheckEmailForm, pool: Pool) -> Result<bool> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 no FROM users WHERE email=$1;")?;
    let res = client.query(&stmt, &[&form.email])?;
    Ok(!res.is_empty())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteForm {
    pub password: String,
}

pub fn delete_user(user_no: u32, form: DeleteForm, pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 no FROM users WHERE no=$1 AND password=$2;")?;
    let res = client.query(&stmt, &[&user_no, &form.password])?;
    ensure!(!res.is_empty(), StatusCode::UNAUTHORIZED, "password doesn't match");
    let stmt = client.prepare("DELETE FROM users WHERE no=$1")?;
    client.query(&stmt, &[&user_no])?;
    Ok(())
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoginForm {
    pub user_id: String,
    pub password: String,
}

pub fn login_user(form: LoginForm, pool: Pool) -> Result<u32> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT 1 no FROM users WHERE id=$1 AND password=$2;")?;
    let res = client.query(&stmt, &[&form.user_id, &form.password])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "login failed"))?;
    Ok(row.get(0))
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetEmailForm {
    pub user_id: String,
}

pub fn get_user_email(form: GetEmailForm, pool: Pool) -> Result<String> {
    let mut client = pool.get()?;
    let stmt = client.prepare("SELECT email FROM users WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.user_id])?;
    let row = res.first().ok_or_else(|| err!(StatusCode::NOT_FOUND, "no user"))?;
    Ok(row.get(0))
}

#[derive(Deserialize, Serialize, Clone)]
pub enum GetInfoForm {
    UserNo(u32),
    UserId(String),
}

pub fn get_user_info(form: GetInfoForm, pool: Pool) -> Result<UserInfo> {
    let mut client = pool.get()?;
    let res = match &form {
        GetInfoForm::UserNo(no) => {
            let stmt = client.prepare("SELECT 1 no, id, name, rating, is_admin FROM users WHERE no=$1;")?;
            client.query(&stmt, &[no])?
        }
        GetInfoForm::UserId(id) => {
            let stmt = client.prepare("SELECT 1 no, id, name, rating, is_admin FROM users WHERE id=$1;")?;
            client.query(&stmt, &[id])?
        }
    };
    let row = res.first().ok_or_else(|| err!(StatusCode::NOT_FOUND, "no user"))?;
    Ok(UserInfo {
        no: UserNo(row.get(0)),
        id: row.get(1),
        name: row.get(2),
        rating: row.get(3),
        room: None,
        is_admin: row.get(4),
    })
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegenerateTokenForm {
    pub user_id: String,
    pub email: String,
}

pub fn regenerate_user_token(form: RegenerateTokenForm, pool: Pool) -> Result<Uuid> {
    let mut client = pool.get()?;
    let stmt = client.prepare(
        "UPDATE pre_users SET token = UUID_GENERATE_V4(), gen_time = NOW() WHERE id=$1 AND email=$2 RETURNING token;",
    )?;
    let res = client.query(&stmt, &[&form.user_id, &form.email])?;
    let row = res
        .first()
        .ok_or_else(|| err!(StatusCode::UNAUTHORIZED, "login failed"))?;
    Ok(row.get(0))
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RegisterForm {
    pub user_id: String,
    pub email: String,
}

pub fn register_user(form: RegisterForm, pool: Pool) -> Result<()> {
    let _ = is_user_id_valid(&form.user_id);
    let _ = is_email_valid(&form.email);
    let mut client = pool.get()?;
    let stmt =
        client.prepare("SELECT 1 FROM ( SELECT id FROM pre_users UNION ALL SELECT id FROM users) a WHERE id=$1;")?;
    let res = client.query(&stmt, &[&form.user_id])?;
    ensure!(res.is_empty(), StatusCode::UNAUTHORIZED, "username already in use");
    let mut client = pool.get()?;
    let stmt = client.prepare("INSERT INTO pre_users (id, email) VALUES ($1, $2);")?;
    let _ = client.query(&stmt, &[&form.user_id, &form.email])?;
    Ok(())
}

pub fn is_user_name_valid(user_name: &str) -> Result<()> {
    let id_regex = Regex::new(r"[a-zA-z]{4,20}$").unwrap();
    ensure!(
        id_regex.is_match(user_name),
        StatusCode::UNAUTHORIZED,
        "only english is allowed for user name"
    );
    Ok(())
}

pub fn is_user_id_valid(user_id: &str) -> Result<()> {
    let id_regex = Regex::new(r"[a-zA-z0-9]{4,12}$").unwrap();
    ensure!(
        id_regex.is_match(user_id),
        StatusCode::UNAUTHORIZED,
        "only english and number is allowed for user id"
    );
    Ok(())
}

pub fn is_password_valid(password: &str, user_id: &str) -> Result<()> {
    let pwd_regex = Regex::new(r"[a-zA-z0-9]{4,12}$").unwrap();
    ensure!(
        pwd_regex.is_match(password),
        StatusCode::UNAUTHORIZED,
        "only english and number is allowed for password"
    );
    ensure!(
        !password.contains(user_id),
        StatusCode::UNAUTHORIZED,
        "password can't contain user id"
    );
    Ok(())
}

pub fn is_email_valid(email: &str) -> Result<()> {
    let email_regex = Regex::new(r"[a-zA-Z0-9._-]{3,}@[a-zA-Z0-9.-]{3,}\.[a-zA-Z]{2,4}").unwrap();
    ensure!(
        email_regex.is_match(email),
        StatusCode::UNAUTHORIZED,
        "not effective email address"
    );
    Ok(())
}
