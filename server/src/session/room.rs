use crate::actor::{Hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct RoomSession {
    user_no: UserNo,
    server: Addr<Hub>,
}

impl Actor for RoomSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RoomSession {
    fn handle(&mut self, _: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        unimplemented!()
    }
}

impl RoomSession {
    pub fn new(user_no: UserNo, server: Addr<Hub>) -> RoomSession {
        RoomSession { user_no, server }
    }
}
