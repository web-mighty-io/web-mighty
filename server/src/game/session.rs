use crate::game::{server, user, CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};
use crate::util::ExAddr;
use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub enum Command {
    Join { room: usize },
    Chat { from: u32, content: String },
    Leave,
}

#[derive(Clone, Message)]
#[rtype(result = "serde_json::Result<()>")]
pub struct Chat {
    pub user_no: u32,
    pub content: String,
}

pub struct WsSession {
    user_no: u32,
    user_addr: ExAddr<user::User>,
    hb: Instant,
    server: Addr<server::MainServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.server
            .send(server::Connect {
                user_no: self.user_no,
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.user_addr.set_addr(res),
                    _ => ctx.stop(),
                };
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl WsSession {
    pub fn new(user_no: u32, addr: Addr<server::MainServer>) -> WsSession {
        WsSession {
            user_no,
            user_addr: ExAddr::new(),
            hb: Instant::now(),
            server: addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.disconnect(ctx);
                return;
            }
            ctx.ping(b"");
        });
    }

    fn disconnect(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.user_addr.do_send(user::Disconnect);
        ctx.stop();
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
                let msg: serde_json::Result<Command> = serde_json::from_str(&*msg);
                let msg = match msg {
                    Ok(msg) => msg,
                    Err(_) => return,
                };

                match msg {
                    Command::Join { .. } => {}
                    Command::Chat { from, content } => {
                        self.user_addr.do_send(user::ChatToSend { user_no: from, content });
                    }
                    Command::Leave => {
                        self.disconnect(ctx);
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
                self.disconnect(ctx);
            }
            _ => {}
        }
    }
}

impl Handler<Chat> for WsSession {
    type Result = serde_json::Result<()>;

    fn handle(&mut self, msg: Chat, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&Command::Chat {
            from: msg.user_no,
            content: msg.content,
        })?);

        Ok(())
    }
}
