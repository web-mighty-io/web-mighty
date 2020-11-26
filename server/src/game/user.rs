use crate::game::{room, server, session};
use actix::prelude::*;
use std::collections::HashMap;

pub enum UserState {
    List {
        rooms: Vec<usize>,
    },
    Room {
        game_state: Box<dyn mighty::MightyState>,
        room: usize,
        // 0 ~ 4: in game user, 5: observer
        user_id: usize,
    },
}

impl Default for UserState {
    fn default() -> Self {
        Self::new()
    }
}

impl UserState {
    fn new() -> UserState {
        UserState::Main
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_no: u32,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatToSend {
    pub user_no: u32,
    pub content: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatReceived {
    pub user_no: u32,
    pub content: String,
}

pub struct User {
    user_no: u32,
    state: UserState,
    rooms: HashMap<usize, Addr<room::Room>>,
    server: Addr<server::MainServer>,
    session: Addr<session::WsSession>,
}

impl Actor for User {
    type Context = Context<User>;
}

impl Handler<Connect> for User {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.user_no = msg.user_no;
    }
}

impl Handler<Disconnect> for User {
    type Result = ();

    fn handle(&mut self, _: Disconnect, _: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<ChatToSend> for User {
    type Result = ();

    fn handle(&mut self, msg: ChatToSend, _: &mut Self::Context) -> Self::Result {
        if let UserState::Room { room, .. } = self.state {
            self.rooms.get(&room).unwrap().do_send(room::Chat {
                user_no: msg.user_no,
                content: msg.content,
            });
        }
    }
}

impl Handler<ChatReceived> for User {
    type Result = ();

    fn handle(&mut self, msg: ChatReceived, _: &mut Self::Context) -> Self::Result {
        self.session.do_send(session::Chat {
            user_no: msg.user_no,
            content: msg.content,
        });
    }
}

impl User {
    pub fn new(server: Addr<server::MainServer>, session: Addr<session::WsSession>) -> User {
        User {
            user_no: 0,
            rooms: HashMap::new(),
            server,
            state: UserState::Main,
            session,
        }
    }
}
