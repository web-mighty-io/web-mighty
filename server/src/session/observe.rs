use crate::actor::{Hub, UserNo};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;

pub struct ObserveSession {
    user_no: UserNo,
    server: Addr<Hub>,
}

impl Actor for ObserveSession {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ObserveSession {
    fn handle(&mut self, _: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        unimplemented!()
    }
}

impl ObserveSession {
    pub fn new(user_no: UserNo, server: Addr<Hub>) -> ObserveSession {
        ObserveSession { user_no, server }
    }
}
