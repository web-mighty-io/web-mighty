use crate::actor::hub::GetRoom;
use crate::actor::room::{RoomJoin, RoomLeave};
use crate::actor::session::{Session, SessionTrait};
use crate::actor::Hub;
use crate::db::game::{get_room_list, GetRoomListForm};
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{ListToClient, ListToServer};

#[derive(Debug)]
pub struct List {
    hub: Addr<Hub>,
    pool: Pool,
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
            ListToServer::GetRoomList { user_num } => {
                let form = GetRoomListForm { user_num };
                let room_list = ignore!(get_room_list(&form, act.inner.pool.clone()));
                ctx.notify(ListToClient::RoomList(room_list));
            }
        }
    }
}

impl List {
    pub fn new(hub: Addr<Hub>, pool: Pool) -> List {
        List { hub, pool }
    }
}
