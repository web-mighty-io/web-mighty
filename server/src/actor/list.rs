use crate::actor::hub::GetRoom;
use crate::actor::room::{RoomJoin, RoomLeave};
use crate::actor::session::{Session, SessionTrait};
use crate::actor::Hub;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{ListToClient, ListToServer};

#[derive(Debug)]
pub struct List {
    hub: Addr<Hub>,
}

impl SessionTrait for List {
    type Sender = ListToClient;

    fn receive(act: &mut Session<Self>, msg: String, ctx: &mut WebsocketContext<Session<Self>>) {
        let msg: ListToServer = ignore!(serde_json::from_str(&*msg));
        match msg {
            ListToServer::Subscribe(id) => {
                act.inner
                    .hub
                    .send(GetRoom(id))
                    .into_actor(act)
                    .then(|res, _, ctx| {
                        if let Ok(Ok(room)) = res {
                            room.do_send(RoomJoin::List(ctx.address()));
                        }

                        fut::ready(())
                    })
                    .wait(ctx);
            }
            ListToServer::Unsubscribe(id) => {
                act.inner
                    .hub
                    .send(GetRoom(id))
                    .into_actor(act)
                    .then(|res, _, ctx| {
                        if let Ok(Ok(room)) = res {
                            room.do_send(RoomLeave::List(ctx.address()));
                        }

                        fut::ready(())
                    })
                    .wait(ctx);
            }
        }
    }
}

impl List {
    pub fn new(hub: Addr<Hub>) -> List {
        List { hub }
    }
}
