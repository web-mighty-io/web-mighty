use crate::actor::room::{Chat, RoomJoin, RoomLeave};
use crate::actor::session::{Session, SessionTrait};
use crate::actor::Room;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{ObserveToClient, ObserveToServer, UserNo};

#[derive(Debug)]
pub struct Observe {
    room: Addr<Room>,
    no: UserNo,
}

impl SessionTrait for Observe {
    type Sender = ObserveToClient;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomJoin::Observe(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomLeave::Observe(ctx.address()));
    }

    fn receive(act: &mut Session<Self>, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let msg: ObserveToServer = serde_json::from_str(&*msg).unwrap();
        match msg {
            ObserveToServer::Chat(chat) => act.inner.room.do_send(Chat::Observe(chat, act.inner.no)),
        }
    }
}

impl Observe {
    pub fn new(room: Addr<Room>, no: UserNo) -> Observe {
        Observe { room, no }
    }
}
