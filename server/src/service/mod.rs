mod delete;
mod get;
mod post;

use crate::app_state::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get::admin)
        .service(get::index)
        .service(get::list)
        .service(get::login)
        .service(get::mail)
        .service(get::observe)
        .service(get::ranking)
        .service(get::register)
        .service(get::resource)
        .service(get::room)
        .service(get::setting)
        .service(get::user_info)
        .service(
            web::scope("/ws")
                .service(get::ws::list)
                .service(get::ws::main)
                .service(get::ws::observe)
                .service(get::ws::room),
        )
        .service(post::login)
        .service(post::logout)
        .service(post::register)
        .service(delete::delete_user);
}

pub async fn p404(data: web::Data<AppState>) -> impl Responder {
    let handlebars = data.get_handlebars();
    let body = handlebars.render("404.hbs", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}
