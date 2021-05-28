mod delete;
mod get;
mod post;

use crate::app_state::AppState;
use actix_web::http::header;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get::pages::admin)
        .service(get::pages::index)
        .service(get::pages::login)
        .service(get::pages::observe)
        .service(get::pages::list)
        .service(get::pages::ranking)
        .service(get::pages::pre_register)
        .service(get::pages::pre_register_complete)
        .service(get::pages::register)
        .service(get::pages::register_complete)
        .service(get::pages::resource)
        .service(get::pages::room)
        .service(get::pages::setting)
        .service(get::pages::user_info)
        .service(get::pages::wasm_not_supported)
        .service(
            web::scope("/ws")
                .service(get::ws::list)
                .service(get::ws::main)
                .service(get::ws::observe)
                .service(get::ws::room),
        )
        .service(post::credentials::login)
        .service(post::credentials::logout)
        .service(post::credentials::regenerate_token)
        .service(post::credentials::pre_register)
        .service(post::credentials::register)
        .service(post::credentials::validate_email)
        .service(post::credentials::validate_user_id)
        .service(delete::delete_user);
}

pub async fn p404(state: web::Data<AppState>) -> HttpResponse {
    let body = state.render("404.hbs", &json!({})).unwrap();
    HttpResponse::Ok()
        .set(header::CacheControl(vec![header::CacheDirective::Private]))
        .set(header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(body)
}
