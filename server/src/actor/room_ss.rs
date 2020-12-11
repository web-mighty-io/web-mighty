use crate::actor::{hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct RoomSession {
    user_no: UserNo,
    server: Addr<hub::Hub>,
}

impl Actor for RoomSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RoomSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        unimplemented!()
    }
}

impl RoomSession {
    pub fn new(user_no: UserNo, server: Addr<hub::Hub>) -> RoomSession {
        RoomSession { user_no, server }
    }
}
