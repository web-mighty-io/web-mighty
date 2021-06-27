use crate::app_state::AppState;
use crate::db::user::{
    check_user_email, check_user_id, login_user, pre_register_user, regenerate_user_token, register_user,
    CheckEmailForm, CheckIdForm, LoginForm, PreRegisterForm, RegenerateTokenForm, RegisterForm,
};
use crate::dev::*;
use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{post, web, HttpResponse};
use serde::Serialize;

#[post("/login")]
pub async fn login(
    id: Identity,
    form: web::Json<LoginForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_no = login_user(&*form, state.pool.clone())?;
    id.remember(user_no.to_string());
    Ok(HttpResponse::Ok().finish())
}

#[post("/pre-register")]
pub async fn pre_register(form: web::Json<PreRegisterForm>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let form = pre_register_user(&*form, state.pool.clone())?;
    state.mail.do_send(form);
    Ok(HttpResponse::Ok().finish())
}

#[post("/regenerate-token")]
pub async fn regenerate_token(
    form: web::Path<RegenerateTokenForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let form = regenerate_user_token(&*form, state.pool.clone())?;
    state.mail.do_send(form);
    Ok(HttpResponse::Found().insert_header((header::LOCATION, "/")).finish())
}

#[post("/register")]
pub async fn register(
    id: Identity,
    form: web::Json<RegisterForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let _ = register_user(&*form, state.pool.clone())?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Clone, Serialize)]
struct ValidateUserIdResult {
    user_id: String,
    exists: bool,
}

#[post("/validate-user-id")]
pub async fn validate_user_id(form: web::Json<CheckIdForm>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let exists = check_user_id(&*form, state.pool.clone())?;
    Ok(HttpResponse::Ok().body(
        serde_json::to_string(&ValidateUserIdResult {
            user_id: form.user_id.clone(),
            exists,
        })
        .unwrap(),
    ))
}

#[derive(Debug, Clone, Serialize)]
struct ValidateEmailResult {
    email: String,
    exists: bool,
}

#[post("/validate-email")]
pub async fn validate_email(
    form: web::Json<CheckEmailForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let exists = check_user_email(&*form, state.pool.clone())?;
    Ok(HttpResponse::Ok().body(
        serde_json::to_string(&ValidateEmailResult {
            email: form.email.clone(),
            exists,
        })
        .unwrap(),
    ))
}
