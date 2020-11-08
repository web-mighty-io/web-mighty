mod get;
mod post;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get::index)
        .service(post::login)
        .service(post::logout)
        .service(post::register)
        .service(post::delete_user)
        .service(web::scope("/res").service(get::resource));
}

// todo: make 404 file
pub async fn p404() -> impl Responder {
    HttpResponse::NotFound()
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub user_id: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct DeleteUserForm {
    pub user_id: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub user_id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}
