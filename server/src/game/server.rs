use crate::game::session;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Message)]
#[rtype = "usize"]
pub struct Connect {
    pub addr: Reciptent<session::Command>,
    pub name: String,
}

#[derive(Clone, Message)]
#[rtype = "()"]
pub struct Disconnect(String);

#[derive(Clone, Message)]
#[rtype = "()"]
pub struct JoinRoom(String, u64);

pub struct WsServer {
    rooms: HashMap<u64, Vec<usize>>,
    sessions: HashMap<usize, Recipient<session::Command>>,
}

impl Actor for WsServer {
    type Context = Context<Self>;
}
