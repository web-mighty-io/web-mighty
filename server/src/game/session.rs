use crate::game::{server, user, CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;

pub struct WsSession {
    name: String,
    session: Option<Addr<user::User>>,
    hb: Instant,
    server: Addr<server::MainServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.server
            .send(server::Connect {
                name: self.name.clone(),
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.session = Some(res),
                    _ => ctx.stop(),
                };
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl WsSession {
    pub fn new(id: String, addr: Addr<server::MainServer>) -> WsSession {
        WsSession {
            name: id,
            session: None,
            hb: Instant::now(),
            server: addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // act.server.do_send(server::Disconnect { name: act.user_id });
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
                        "/join" => print!("join"),
                        "/leave" => print!("leave"),
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
