use crate::game::session;
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<session::Command>,
    pub name: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct JoinRoom(String, u64);

pub struct WsServer {
    #[allow(dead_code)]
    rooms: HashMap<u64, Vec<usize>>,
    #[allow(dead_code)]
    sessions: HashMap<usize, Recipient<session::Command>>,
}

impl Default for WsServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, _: Connect, _: &mut Self::Context) -> Self::Result {
        // todo
        1
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, _: Disconnect, _: &mut Self::Context) -> Self::Result {
        // todo
    }
}

impl WsServer {
    pub fn new() -> Self {
        WsServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}
