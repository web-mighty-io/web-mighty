use crate::actor::{Hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct MainSession {
    user_no: UserNo,
    server: Addr<Hub>,
}

impl Actor for MainSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MainSession {
    fn handle(&mut self, _: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        unimplemented!()
    }
}

impl MainSession {
    pub fn new(user_no: UserNo, server: Addr<Hub>) -> MainSession {
        MainSession { user_no, server }
    }
}
