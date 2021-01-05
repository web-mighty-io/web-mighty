use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;
use serde::Serialize;
use std::time::SystemTime;

pub trait SessionTrait: Sized + Unpin + 'static {
    type Receiver: Message<Result = ()> + Serialize + Send;

    fn started(&mut self, _: &mut WebsocketContext<Session<Self>>) {}

    fn stopped(&mut self, _: &mut WebsocketContext<Session<Self>>) {}

    fn handle(&mut self, msg: Self::Receiver, ctx: &mut WebsocketContext<Session<Self>>) {
        ctx.text(serde_json::to_string(&msg).unwrap())
    }

    fn receive(&mut self, msg: String, ctx: &mut WebsocketContext<Session<Self>>);

    fn make(self) -> Session<Self> {
        Session::new(self)
    }
}

pub struct Session<T>
where
    T: SessionTrait,
{
    inner: T,
    hb: SystemTime,
}

impl<T> Actor for Session<T>
where
    T: SessionTrait + Unpin + 'static,
{
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.inner.started(ctx);
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if act.hb.elapsed().unwrap() > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.inner.stopped(ctx);
    }
}

impl<T> Handler<T::Receiver> for Session<T>
where
    T: SessionTrait + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: T::Receiver, ctx: &mut Self::Context) {
        self.inner.handle(msg, ctx);
    }
}

impl<T> StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session<T>
where
    T: SessionTrait,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match try_ctx!(msg, ctx) {
            ws::Message::Text(msg) => {
                self.inner.receive(msg, ctx);
            }
            ws::Message::Ping(msg) => {
                self.hb = SystemTime::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = SystemTime::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl<T> Session<T>
where
    T: SessionTrait,
{
    pub fn new(inner: T) -> Session<T> {
        Session {
            inner,
            hb: SystemTime::now(),
        }
    }
}
