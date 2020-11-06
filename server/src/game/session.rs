use crate::game::{server, CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};
use actix::prelude::*;
use actix_web_actors::ws;
use mighty::MightyGame;
use std::time::Instant;

#[derive(Message)]
#[rtype = "()"]
pub struct Command;

pub struct GameSession {
    id: String,
    room: usize,
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

    }
}

impl GameSession {
    pub fn new(id: String) -> GameSession {
        GameSession { id, room: 0 }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect(self.id));
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
