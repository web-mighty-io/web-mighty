use crate::db;
use crate::handlers::{LoginForm, RegisterForm};
use actix_identity::Identity;
use actix_web::{http, post, web, Error, HttpResponse, Responder};
use deadpool_postgres::Pool;

#[post("/login")]
pub async fn login(
    id: Identity,
    form: web::Form<LoginForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let _ = db::login(&form, &db_pool).await?;
    id.remember(form.username.clone());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}

#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[post("/register")]
pub async fn register(id: Identity, form: web::Form<RegisterForm>) -> impl Responder {
    HttpResponse::NotImplemented()
}
