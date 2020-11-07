use crate::game::{server, CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use std::time::Instant;

pub struct WsSession {
    id: usize,
    name: String,
    room: usize,
    hb: Instant,
    addr: Addr<server::WsServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                name: self.name.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                };
                fut::ready(());
            })
            .wait(ctx);
    }

}

impl WsSession {
    pub fn new(id: String, addr: Addr<server::WsServer>) -> WsSession {
        WsSession {
            id: 0,
            name: id,
            room: 0,
            hb: Instant::now(),
            addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect(self.name.clone()));
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(msg) => {
                let msg = msg.trim();

                if msg.starts_with('/') {
                    let v = msg.splitn(2, ' ').collect::<Vec<_>>();
                    match v[0] {
                        "" => {}
                        _ => {}
                    }
                }
            }
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
