use crate::app_state::AppState;
use actix_identity::Identity;
use actix_web::{get, web, Either, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn index(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars
            .render("main.hbs", &json!({ "user_id": id }))
            .unwrap();
        HttpResponse::Ok().body(body)
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("index.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/{file:.*}")]
pub async fn resource(
    data: web::Data<AppState>,
    web::Path(file): web::Path<String>,
) -> impl Responder {
    let resources = data.get_resources();
    if let Some(body) = resources.get(&file) {
        Either::A(HttpResponse::Ok().body(body))
    } else {
        Either::B(HttpResponse::NotFound())
    }
}