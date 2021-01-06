use crate::actor::hub::GetRoom;
use crate::actor::room::{RoomInfo, RoomJoin, RoomLeave};
use crate::actor::{Hub, RoomId};
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

pub struct List {
    hub: Addr<Hub>,
}

#[derive(Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub enum ListSend {
    Room(RoomInfo),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ListReceive {
    Subscribe(RoomId),
    Unsubscribe(RoomId),
}

impl SessionTrait for List {
    type Sender = ListSend;

    fn receive(act: &mut Session<Self>, msg: String, ctx: &mut WebsocketContext<Session<Self>>) {
        let msg: ListReceive = ignore!(serde_json::from_str(&*msg));
        match msg {
            ListReceive::Subscribe(id) => {
                ignore!(ignore!(send(act, ctx, act.inner.hub.clone(), GetRoom(id))))
                    .do_send(RoomJoin::List(ctx.address()));
            }
            ListReceive::Unsubscribe(id) => {
                ignore!(ignore!(send(act, ctx, act.inner.hub.clone(), GetRoom(id))))
                    .do_send(RoomLeave::List(ctx.address()));
            }
        }
    }
}

impl List {
    pub fn new(hub: Addr<Hub>) -> List {
        List { hub }
    }
}
