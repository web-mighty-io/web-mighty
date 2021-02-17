pub mod api;
pub mod ws;

use crate::actor::mail::SendVerification;
use crate::app_state::AppState;
use crate::db::user::{get_user_info, GetInfoForm};
use crate::dev::*;
use crate::service::p404;
use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{get, web, HttpResponse, Responder};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::{json, Map};

#[get("/admin")]
pub async fn admin(id: Identity, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let info = get_user_info(&GetInfoForm::UserNo(user_no), state.pool.clone())?;
        if info.is_admin {
            let body = state.render("admin.hbs", &json!({ "id": id })).unwrap();
            Ok(HttpResponse::Ok()
                .set(header::CacheControl(vec![header::CacheDirective::Private]))
                .set(header::ContentType(mime::TEXT_HTML_UTF_8))
                .body(body))
        } else {
            Ok(p404(state).await)
        }
    } else {
        Ok(p404(state).await)
    }
}

#[get("/")]
pub async fn index(id: Identity, state: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let body = state.render("main.hbs", &json!({ "id": id })).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    } else {
        let body = state.render("index.hbs", &json!({})).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    }
}

#[get("/login")]
pub async fn login(id: Identity, state: web::Data<AppState>) -> impl Responder {
    if id.identity().is_some() {
        HttpResponse::Found().header(header::LOCATION, "/").finish()
    } else {
        let body = state.render("login.hbs", &json!({})).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    }
}

#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().finish()
}

#[get("/pre-register")]
pub async fn pre_register(id: Identity, state: web::Data<AppState>) -> impl Responder {
    if id.identity().is_some() {
        HttpResponse::Found().header(header::LOCATION, "/").finish()
    } else {
        let body = state.render("pre-register.hbs", &json!({})).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    }
}

#[get("/pre-register-complete")]
pub async fn pre_register_complete(id: Identity, state: web::Data<AppState>) -> impl Responder {
    if id.identity().is_some() {
        HttpResponse::Found().header(header::LOCATION, "/").finish()
    } else {
        let body = state.render("pre-register-complete.hbs", &json!({})).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    }
}

#[get("/register/{token}")]
pub async fn register(state: web::Data<AppState>, web::Path(token): web::Path<String>) -> Result<HttpResponse, Error> {
    let form: SendVerification = jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(state.secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?
    .claims;
    let body = state
        .render(
            "register.hbs",
            &json!({ "token": form.token, "email": form.email, "user_id": form.user_id }),
        )
        .unwrap();
    Ok(HttpResponse::Ok()
        .set(header::CacheControl(vec![header::CacheDirective::Private]))
        .set(header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(body))
}

#[get("/register-complete")]
pub async fn register_complete(id: Identity, state: web::Data<AppState>) -> impl Responder {
    if id.identity().is_some() {
        HttpResponse::Found().header(header::LOCATION, "/").finish()
    } else {
        let body = state.render("register-complete.hbs", &json!({})).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    }
}

#[get("/observe/{room_id}")]
pub async fn observe(
    id: Identity,
    state: web::Data<AppState>,
    web::Path(room_id): web::Path<String>,
) -> impl Responder {
    let mut val = Map::new();
    if let Some(id) = id.identity() {
        val.insert("id".to_owned(), json!(id));
    }
    val.insert("room_id".to_owned(), json!(room_id));

    let body = state.render("observe.hbs", &val).unwrap();
    HttpResponse::Ok()
        .set(header::CacheControl(vec![header::CacheDirective::Private]))
        .set(header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(body)
}

// #[get("/ranking")]
// pub async fn ranking(state: web::Data<AppState>) -> impl Responder {
//     let body = state.render("ranking.hbs", &json!({})).unwrap();
//     HttpResponse::Ok()
//         .set(header::CacheControl(vec![header::CacheDirective::Private]))
//         .set(header::ContentType(mime::TEXT_HTML_UTF_8))
//         .body(body)
// }

#[get("/res/{file:.*}")]
pub async fn resource(state: web::Data<AppState>, web::Path(file): web::Path<String>) -> impl Responder {
    let resources = state.get_resources();
    if let Some(body) = resources.get(&file) {
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Public]))
            .set(header::ContentType(
                mime_guess::from_path(&file).first_or(mime::TEXT_PLAIN_UTF_8),
            ))
            .body(body.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/room/{room_id}")]
pub async fn room(id: Identity, state: web::Data<AppState>, web::Path(room_id): web::Path<String>) -> impl Responder {
    if let Some(id) = id.identity() {
        let body = state.render("game.hbs", &json!({ "id": id })).unwrap();
        HttpResponse::Ok()
            .set(header::CacheControl(vec![header::CacheDirective::Private]))
            .set(header::ContentType(mime::TEXT_HTML_UTF_8))
            .body(body)
    } else {
        HttpResponse::Found()
            .header(header::LOCATION, format!("/login?back=%2Froom%2F{}", room_id))
            .finish()
    }
}

// #[get("/setting")]
// pub async fn setting(id: Identity, state: web::Data<AppState>) -> impl Responder {
//     if let Some(id) = id.identity() {
//         let body = state.render("setting.hbs", &json!({ "id": id })).unwrap();
//         HttpResponse::Ok()
//             .set(header::CacheControl(vec![header::CacheDirective::Private]))
//             .set(header::ContentType(mime::TEXT_HTML_UTF_8))
//             .body(body)
//     } else {
//         HttpResponse::Found()
//             .header(header::LOCATION, "/login?back=%2Fsetting".to_owned())
//             .finish()
//     }
// }

// #[get("/user/{user_id}")]
// pub async fn user_info(
//     id: Identity,
//     state: web::Data<AppState>,
//     web::Path(user_id): web::Path<String>,
// ) -> Result<HttpResponse, Error> {
//     let mut val = Map::new();
//     if let Some(id) = id.identity() {
//         val.insert("id".to_owned(), json!(id));
//     }
//
//     let user_no = user_id.parse().unwrap();
//     let user_info = get_user_info(GetInfoForm::UserNo(user_no), state.pool.clone())?;
//
//     val.insert("user_id".to_owned(), json!(user_info.id));
//     val.insert("name".to_owned(), json!(user_info.name));
//     val.insert("rating".to_owned(), json!(user_info.rating));
//     val.insert("is_admin".to_owned(), json!(user_info.is_admin));
//
//     let body = state.render("user.hbs", &val).unwrap();
//     Ok(HttpResponse::Ok()
//         .set(header::CacheControl(vec![header::CacheDirective::Private]))
//         .set(header::ContentType(mime::TEXT_HTML_UTF_8))
//         .body(body))
// }
