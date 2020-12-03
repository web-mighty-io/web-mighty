use crate::actor::{server, UserId};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ListSession {
    user_no: UserId,
    server: Addr<server::Server>,
}

impl Actor for ListSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ListSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        unimplemented!()
    }
}

impl ListSession {
    pub fn new(user_no: UserId, server: Addr<server::Server>) -> ListSession {
        ListSession { user_no, server }
    }
}
