use crate::app_state::AppState;
use crate::db::user::{login_user, pre_register_user, register_user, LoginForm, PreRegisterForm, RegisterForm};
use crate::dev::*;
use actix_identity::Identity;
use actix_web::{post, web, HttpResponse};

#[post("/login")]
pub async fn login(
    id: Identity,
    form: web::Json<LoginForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_no = login_user((*form).clone(), state.pool.clone())?;
    id.remember(user_no.to_string());
    Ok(HttpResponse::Ok().finish())
}

#[post("/pre-register")]
pub async fn pre_register(form: web::Json<PreRegisterForm>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let form = pre_register_user((*form).clone(), state.pool.clone())?;
    state.mail.do_send(form);
    Ok(HttpResponse::Ok().finish())
}

#[post("/register")]
pub async fn register(
    id: Identity,
    form: web::Json<RegisterForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let _ = register_user((*form).clone(), state.pool.clone())?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Ok().finish())
}
