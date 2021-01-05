use crate::actor::hub::{GetRoom, HubConnect};
use crate::actor::{List, Main, Observe, RoomUser, UserNo};
use crate::app_state::AppState;
use crate::dev::*;
use actix_identity::Identity;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::TryFutureExt;
use std::str::FromStr;
use uuid::Uuid;

#[get("/list")]
pub async fn list(
    id: Identity,
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        ws::start(List::new(data.hub.clone()).make(), &req, stream)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/main")]
pub async fn main(
    id: Identity,
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let addr = data
            .hub
            .send(HubConnect(UserNo(user_no)))
            .into_future()
            .await
            .unwrap()?;
        ws::start(Main::new(addr, data.hub.clone()).make(), &req, stream)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/observe/{room_id}")]
pub async fn observe(
    id: Identity,
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
    web::Path(room_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let room_id = Uuid::from_str(&*room_id).unwrap().into();
        let addr = data.hub.send(GetRoom(room_id)).into_future().await.unwrap()?;
        ws::start(Observe::new(UserNo(user_no), addr).make(), &req, stream)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/room")]
pub async fn room(
    id: Identity,
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let addr = data
            .hub
            .send(HubConnect(UserNo(user_no)))
            .into_future()
            .await
            .unwrap()?;
        ws::start(RoomUser::new(addr).make(), &req, stream)
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
