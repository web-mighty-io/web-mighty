use crate::actor::{Hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ListSession {
    user_no: UserNo,
    server: Addr<Hub>,
}

impl Actor for ListSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ListSession {
    fn handle(&mut self, _: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        unimplemented!()
    }
}

impl ListSession {
    pub fn new(user_no: UserNo, server: Addr<Hub>) -> ListSession {
        ListSession { user_no, server }
    }
}
