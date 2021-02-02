use crate::actor::hub::{GetRoom, HubConnect};
use crate::actor::{List, Main, Observe, RoomUser};
use crate::app_state::AppState;
use crate::dev::*;
use crate::service::p404;
use actix_identity::Identity;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::TryFutureExt;

#[get("/list")]
pub async fn list(
    id: Identity,
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        ws::start(List::new(state.hub.clone()).make(), &req, stream)
    } else {
        Ok(p404(state).await)
    }
}

#[get("/main")]
pub async fn main(
    id: Identity,
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let addr = state
            .hub
            .send(HubConnect(UserNo(user_no)))
            .into_future()
            .await
            .unwrap()?;
        ws::start(Main::new(addr, state.hub.clone()).make(), &req, stream)
    } else {
        Ok(p404(state).await)
    }
}

#[get("/observe/{room_id}")]
pub async fn observe(
    id: Identity,
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
    web::Path(room_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        let room_id = room_id.parse::<u32>().unwrap().into();
        let addr = state.hub.send(GetRoom(room_id)).into_future().await.unwrap()?;
        ws::start(Observe::new(addr).make(), &req, stream)
    } else {
        Ok(p404(state).await)
    }
}

#[get("/room")]
pub async fn room(
    id: Identity,
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let user_no = id.parse().unwrap();
        let addr = state
            .hub
            .send(HubConnect(UserNo(user_no)))
            .into_future()
            .await
            .unwrap()?;
        ws::start(RoomUser::new(addr).make(), &req, stream)
    } else {
        Ok(p404(state).await)
    }
}
