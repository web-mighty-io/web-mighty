use crate::actor::{hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct MainSession {
    user_no: UserNo,
    server: Addr<hub::Hub>,
}

impl Actor for MainSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MainSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        unimplemented!()
    }
}

impl MainSession {
    pub fn new(user_no: UserNo, server: Addr<hub::Hub>) -> MainSession {
        MainSession { user_no, server }
    }
}
