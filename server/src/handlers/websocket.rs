use crate::game::session::GameSession;
use actix_identity::Identity;
use actix_web::{get, web, Either, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

#[get("/")]
pub async fn index(id: Identity, req: HttpRequest, stream: web::Payload) -> impl Responder {
    if let Some(id) = id.identity() {
        Either::A(ws::start(GameSession::new(id), &req, stream))
    } else {
        Either::B(HttpResponse::NotFound())
    }
}
