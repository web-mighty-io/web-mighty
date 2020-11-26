mod delete;
mod get;
mod post;

use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get::index)
        .service(post::login)
        .service(post::logout)
        .service(post::register)
        .service(delete::delete_user)
        .service(get::websocket)
        .service(web::scope("/res").service(get::resource));
}

// todo: make 404 file
pub async fn p404() -> impl Responder {
    HttpResponse::NotFound()
}
