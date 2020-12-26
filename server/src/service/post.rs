use crate::actor::db::{LoginForm, RegisterForm};
use crate::app_state::AppState;
use actix_identity::Identity;
use actix_web::{http, post, web, Error, HttpResponse, Responder};
use futures::TryFutureExt;

#[post("/login")]
pub async fn login(
    id: Identity,
    form: web::Form<LoginForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user_no = state.db.send((*form).clone()).into_future().await.unwrap()?;
    id.remember(user_no.to_string());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
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
    let _ = state.db.send((*form).clone()).into_future().await.unwrap()?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}
