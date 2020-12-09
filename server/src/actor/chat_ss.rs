use crate::actor::{server, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ChatSession {
    user_no: UserNo,
    server: Addr<server::Server>,
}

impl Actor for ChatSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        unimplemented!()
    }
}

impl ChatSession {
    pub fn new(user_no: UserNo, server: Addr<server::Server>) -> ChatSession {
        ChatSession { user_no, server }
    }
}
