use crate::game::{room, session, user};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Message)]
#[rtype(result = "Addr<user::User>")]
pub struct Connect {
    pub user_no: u32,
    pub addr: Addr<session::WsSession>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveSession {
    pub user_no: u32,
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

// room number should be greater than 0
pub struct MainServer {
    room_addr: HashMap<usize, Addr<room::Room>>,
    sessions: HashMap<u32, Addr<user::User>>,
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
    type Result = Addr<user::User>;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.sessions.get(&msg.user_no) {
            addr.clone()
        } else {
            let session = user::User::new(ctx.address(), msg.addr).start();
            session.do_send(user::Connect {
                user_no: msg.user_no,
            });
            self.sessions.insert(msg.user_no, session.clone());
            session
        }
    }
}

impl Handler<RemoveSession> for MainServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.user_no);
    }
}

impl Handler<GetRoom> for MainServer {
    type Result = Option<Addr<room::Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.room_addr.get(&msg.room_id).cloned()
    }
}

impl Handler<MakeRoom> for MainServer {
    type Result = usize;

    fn handle(&mut self, msg: MakeRoom, _: &mut Self::Context) -> Self::Result {
        // todo: generate unique id and return it
        let room_id = rand::random();
        let room = room::Room::start_default();
        room.do_send(room::ChangeName {
            user_no: 0,
            name: msg.name,
        });
        self.room_addr.insert(room_id, room);
        room_id
    }
}

impl Handler<RemoveRoom> for MainServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveRoom, _: &mut Self::Context) -> Self::Result {
        self.room_addr.remove(&msg.room_id);
    }
}

impl MainServer {
    pub fn new() -> Self {
        MainServer {
            room_addr: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}
