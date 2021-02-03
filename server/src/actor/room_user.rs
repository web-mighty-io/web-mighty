use crate::actor::session::{Session, SessionTrait};
use crate::actor::user::{UserCommand, UserConnect, UserDisconnect};
use crate::actor::User;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;

#[derive(Debug)]
pub struct RoomUser {
    user: Addr<User>,
}

impl SessionTrait for RoomUser {
    type Sender = RoomUserToClient;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserConnect::Room(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserDisconnect::Room(ctx.address()));
    }

    fn receive(act: &mut Session<Self>, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let msg: RoomUserToServer = serde_json::from_str(&*msg).unwrap();
        act.inner.user.do_send(UserCommand(msg));
    }
}

impl RoomUser {
    pub fn new(user: Addr<User>) -> RoomUser {
        RoomUser { user }
    }
}
