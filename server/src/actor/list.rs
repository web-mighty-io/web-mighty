use crate::actor::hub::GetRoom;
use crate::actor::room::{RoomJoin, RoomLeave};
use crate::actor::Hub;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{ListToClient, ListToServer};

pub struct List {
    hub: Addr<Hub>,
}

impl SessionTrait for List {
    type Sender = ListToClient;

    fn receive(act: &mut Session<Self>, msg: String, ctx: &mut WebsocketContext<Session<Self>>) {
        let msg: ListToServer = ignore!(serde_json::from_str(&*msg));
        match msg {
            ListToServer::Subscribe(id) => {
                ignore!(ignore!(send(act, ctx, act.inner.hub.clone(), GetRoom(id))))
                    .do_send(RoomJoin::List(ctx.address()));
            }
            ListToServer::Unsubscribe(id) => {
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
