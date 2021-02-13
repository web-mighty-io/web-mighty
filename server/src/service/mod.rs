mod delete;
mod get;
mod post;

use crate::app_state::AppState;
use actix_web::http::header;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get::admin)
        .service(get::index)
        .service(get::login)
        .service(get::logout)
        .service(get::mail)
        .service(get::observe)
        // .service(get::ranking)
        .service(get::regenerate_token)
        .service(get::register)
        .service(get::resource)
        .service(get::room)
        // .service(get::setting)
        // .service(get::user_info)
        .service(
            web::scope("/ws")
                .service(get::ws::list)
                .service(get::ws::main)
                .service(get::ws::observe)
                .service(get::ws::room),
        )
        .service(post::login)
        .service(post::pre_register)
        .service(post::register)
        .service(delete::delete_user);
}

pub async fn p404(state: web::Data<AppState>) -> HttpResponse {
    let body = state.render("404.hbs", &json!({})).unwrap();
    HttpResponse::Ok()
        .set(header::CacheControl(vec![header::CacheDirective::Private]))
        .set(header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(body)
}
