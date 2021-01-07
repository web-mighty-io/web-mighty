use crate::actor::room::{RoomJoin, RoomLeave};
use crate::actor::Room;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{ObserveToClient, ObserveToServer};

pub struct Observe {
    room: Addr<Room>,
}

impl SessionTrait for Observe {
    type Sender = ObserveToClient;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomJoin::Observe(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomLeave::Observe(ctx.address()));
    }

    fn receive(_: &mut Session<Self>, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let _: ObserveToServer = serde_json::from_str(&*msg).unwrap();
    }
}

impl Observe {
    pub fn new(room: Addr<Room>) -> Observe {
        Observe { room }
    }
}
