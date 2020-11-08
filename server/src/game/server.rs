use crate::game::{room, session};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Addr<session::WsSession>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: usize,
}

#[derive(Clone, Message)]
#[rtype(result = "Option<Addr<room::Room>>")]
pub struct GetRoom {
    pub room_id: usize,
}

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct MakeRoom {
    pub name: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveRoom {
    pub room_id: usize,
}

pub struct MainServer {
    rooms: HashMap<usize, Addr<room::Room>>,
    sessions: HashMap<usize, Addr<session::WsSession>>,
}

impl Default for MainServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for MainServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MainServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        // todo: generate unique id and return it
        let session_id = rand::random();
        self.sessions.insert(session_id, msg.addr);
        session_id
    }
}

impl Handler<Disconnect> for MainServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.session_id);
    }
}

impl Handler<GetRoom> for MainServer {
    type Result = Option<Addr<room::Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.room_id).cloned()
    }
}

impl Handler<MakeRoom> for MainServer {
    type Result = usize;

    fn handle(&mut self, msg: MakeRoom, _: &mut Self::Context) -> Self::Result {
        // todo: generate unique id and return it
        let room_id = rand::random();
        let room = room::Room::start_default();
        room.do_send(room::ChangeName { name: msg.name });
        self.rooms.insert(room_id, room);
        room_id
    }
}

impl Handler<RemoveRoom> for MainServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.remove(&msg.room_id);
    }
}

impl MainServer {
    pub fn new() -> Self {
        MainServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}
