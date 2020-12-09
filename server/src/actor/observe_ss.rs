use crate::actor::{server, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ObserveSession {
    user_no: UserNo,
    server: Addr<server::Server>,
}

impl Actor for ObserveSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ObserveSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        unimplemented!()
    }
}

impl ObserveSession {
    pub fn new(user_no: UserNo, server: Addr<server::Server>) -> ObserveSession {
        ObserveSession { user_no, server }
    }
}
