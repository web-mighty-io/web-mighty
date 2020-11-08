use crate::db;
use crate::handlers::{DeleteUserForm, LoginForm, RegisterForm};
use actix_identity::Identity;
use actix_web::{http, post, web, Error, HttpResponse, Responder};
use deadpool_postgres::Pool;

#[post("/login")]
pub async fn login(id: Identity, form: web::Form<LoginForm>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let _ = db::login(&form, &db_pool).await?;
    id.remember(form.user_id.clone());
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
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let _ = db::register(&form, &db_pool).await?;
    id.remember(form.user_id.clone());
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}

#[post("/delete_user")]
pub async fn delete_user(
    id: Identity,
    form: web::Form<DeleteUserForm>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    if !matches!(id.identity(), None) {
        let _ = db::delete_user(&form, &db_pool).await?;
        id.forget();
    }
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}
