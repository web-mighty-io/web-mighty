use crate::actor::user::{UserConnect, UserDisconnect};
use crate::actor::User;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

pub struct RoomUser {
    user: Addr<User>,
}

#[derive(Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct RoomUserSend;

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomUserReceive;

impl SessionTrait for RoomUser {
    type Receiver = RoomUserSend;

    fn started(&mut self, ctx: &mut WebsocketContext<Session<Self>>) {
        self.user.do_send(UserConnect::Room(ctx.address()));
    }

    fn stopped(&mut self, ctx: &mut WebsocketContext<Session<Self>>) {
        self.user.do_send(UserDisconnect::Room(ctx.address()));
    }

    fn receive(&mut self, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let _: RoomUserReceive = serde_json::from_str(&*msg).unwrap();
    }
}

impl RoomUser {
    pub fn new(user: Addr<User>) -> RoomUser {
        RoomUser { user }
    }
}
