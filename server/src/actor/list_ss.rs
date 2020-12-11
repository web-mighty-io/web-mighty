use crate::actor::{hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ListSession {
    user_no: UserNo,
    server: Addr<hub::Hub>,
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
    pub fn new(user_no: UserNo, server: Addr<hub::Hub>) -> ListSession {
        ListSession { user_no, server }
    }
}
