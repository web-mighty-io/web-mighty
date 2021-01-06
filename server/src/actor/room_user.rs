use crate::actor::room::RoomInfo;
use crate::actor::user::{UserCommand, UserConnect, UserDisconnect};
use crate::actor::User;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};
use mighty::prelude::{State, Rule, Command};

pub struct RoomUser {
    user: Addr<User>,
}

#[derive(Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub enum RoomUserSend {
    Room(RoomInfo),
    Game(State),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RoomUserReceive {
    Start,
    ChangeName(String),
    ChangeRule(Rule),
    Command(Command),
}

impl SessionTrait for RoomUser {
    type Sender = RoomUserSend;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserConnect::Room(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserDisconnect::Room(ctx.address()));
    }

    fn receive(act: &mut Session<Self>, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let msg: RoomUserReceive = serde_json::from_str(&*msg).unwrap();
        act.inner.user.do_send(UserCommand(msg));
    }
}

impl RoomUser {
    pub fn new(user: Addr<User>) -> RoomUser {
        RoomUser { user }
    }
}
