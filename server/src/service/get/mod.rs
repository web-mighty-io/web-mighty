pub mod api;
pub mod ws;

use crate::app_state::AppState;
use actix_identity::Identity;
use actix_web::{get, http, web, HttpResponse, Responder};
use serde_json::{json, Map};

#[get("/admin")]
pub async fn admin(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        // todo
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/")]
pub async fn index(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("main.hbs", &json!({ "user_id": id })).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("index.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/join/{room_id}")]
pub async fn join(id: Identity, data: web::Data<AppState>, web::Path(room_id): web::Path<String>) -> impl Responder {
    if let Some(id) = id.identity() {
        // todo
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, format!("/login?back=join_{}", room_id))
            .finish()
    }
}

#[get("/list")]
pub async fn list(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("list.hbs", &json!({ "user_id": id })).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, "/login?back=list")
            .finish()
    }
}

#[get("/login")]
pub async fn login(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("login.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/mail/{token}")]
pub async fn mail(data: web::Data<AppState>, web::Path(token): web::Path<String>) -> impl Responder {
    let handlebars = data.get_handlebars();
    let body = handlebars.render("mail.hbs", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/observe/{room_id}")]
pub async fn observe(id: Identity, data: web::Data<AppState>, web::Path(room_id): web::Path<String>) -> impl Responder {
    let handlebars = data.get_handlebars();
    let body = handlebars.render("observe.hbs", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/ranking")]
pub async fn ranking(data: web::Data<AppState>) -> impl Responder {
    let handlebars = data.get_handlebars();
    let body = handlebars.render("ranking.hbs", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/register")]
pub async fn register(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        HttpResponse::Found().header(http::header::LOCATION, "/").finish()
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("register.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/res/{file:.*}")]
pub async fn resource(data: web::Data<AppState>, web::Path(file): web::Path<String>) -> impl Responder {
    let resources = data.get_resources();
    if let Some(body) = resources.get(&file) {
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/room/{room_id}")]
pub async fn room(id: Identity, data: web::Data<AppState>, web::Path(room_id): web::Path<String>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("room.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, format!("/login?back=room_{}", room_id))
            .finish()
    }
}

#[get("/setting")]
pub async fn setting(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("setting.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, "/login?back=setting".to_owned())
            .finish()
    }
}

#[get("/user/{user_id}")]
pub async fn user(id: Identity, data: web::Data<AppState>, web::Path(user_id): web::Path<String>) -> impl Responder {
    let mut val = Map::new();
    if let Some(id) = id.identity() {
        val.insert("id".to_owned(), json!(id));
    }
    let handlebars = data.get_handlebars();
    let body = handlebars.render("user.hbs", &val).unwrap();
    HttpResponse::Ok().body(body)
}
