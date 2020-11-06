use crate::game::session;
use actix::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Message)]
#[rtype = "usize"]
pub struct Connect(Reciptent<session::Command>);

#[derive(Clone, Message)]
#[rtype = "()"]
pub struct Disconnect(usize);

#[derive(Clone, Message)]
#[rtype = "()"]
pub struct JoinRoom(usize, u64);

pub struct GameServer {
    rooms: HashMap<u64, Vec<usize>>,
    sessions: HashMap<usize, Recipient<session::Command>>,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}
