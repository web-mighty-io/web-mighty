use crate::actor::user::{UserConnect, UserDisconnect};
use crate::actor::User;
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

pub struct Main {
    user: Addr<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
pub struct MainSend;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MainReceive;

impl SessionTrait for Main {
    type Receiver = MainSend;

    fn started(&mut self, ctx: &mut WebsocketContext<Session<Self>>) {
        self.user.do_send(UserConnect::Main(ctx.address()));
    }

    fn stopped(&mut self, ctx: &mut WebsocketContext<Session<Self>>) {
        self.user.do_send(UserDisconnect::Main(ctx.address()));
    }

    fn handle(&mut self, msg: Self::Receiver, ctx: &mut WebsocketContext<Session<Self>>) {
        ctx.text(serde_json::to_string(&msg).unwrap())
    }

    fn receive(&mut self, msg: String, _: &mut WebsocketContext<Session<Self>>) {
        let _: MainReceive = serde_json::from_str(&*msg).unwrap();
    }
}

impl Main {
    pub fn new(user: Addr<User>) -> Main {
        Main { user }
    }
}
