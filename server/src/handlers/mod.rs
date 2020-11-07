mod get;
mod post;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get::index)
        .service(post::login)
        .service(post::logout)
        .service(post::register)
        .service(get::websocket)
        .service(web::scope("/res").service(get::resource));
}

// todo: make 404 file
pub async fn p404() -> impl Responder {
    HttpResponse::NotFound()
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password_hash: String,
}
