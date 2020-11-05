use actix_identity::Identity;
use actix_web::{get, web, Either, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

#[get("/")]
pub async fn index(id: Identity, req: HttpRequest, stream: web::Payload) -> impl Responder {
    if let Some(id) = id.identity() {
        Either::A(ws::start(MyWs {}, &req, stream))
    } else {
        Either::B(HttpResponse::NotFound())
    }
}
