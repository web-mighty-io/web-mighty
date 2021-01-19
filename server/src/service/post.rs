use crate::app_state::AppState;
use crate::db::user::{login_user, register_user, LoginForm, RegisterForm};
use crate::dev::*;
use actix_identity::Identity;
use actix_web::{http, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginQuery {
    back: Option<String>,
}

#[post("/login")]
pub async fn login(
    id: Identity,
    form: web::Form<LoginForm>,
    state: web::Data<AppState>,
    query: web::Query<LoginQuery>,
) -> Result<HttpResponse, Error> {
    let user_no = login_user((*form).clone(), state.pool.clone())?;
    id.remember(user_no.to_string());
    Ok(HttpResponse::Found()
        .header(
            http::header::LOCATION,
            query.back.clone().unwrap_or_else(|| "/".to_owned()),
        )
        .finish()
        .into_body())
}

#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

#[post("/register")]
pub async fn register(
    id: Identity,
    form: web::Form<RegisterForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let _ = register_user((*form).clone(), state.pool.clone())?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}
