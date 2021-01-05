use crate::actor::room::{RoomInfo, RoomJoin, RoomLeave};
use crate::actor::{Room, UserNo};
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use mighty::State;
use serde::{Deserialize, Serialize};

pub struct Observe {
    user_no: UserNo,
    room: Addr<Room>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
pub enum ObserveSend {
    Room(RoomInfo),
    Game(State),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObserveReceive;

impl SessionTrait for Observe {
    type Receiver = ObserveSend;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomJoin::Observe(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.room.do_send(RoomLeave::Observe(ctx.address()));
    }

    fn receive(_: &mut Session<Self>, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let _: ObserveReceive = serde_json::from_str(&*msg).unwrap();
    }
}

impl Observe {
    pub fn new(user_no: UserNo, room: Addr<Room>) -> Observe {
        Observe { user_no, room }
    }
}
