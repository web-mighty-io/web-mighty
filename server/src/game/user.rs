use crate::game::{room, server, session};
use actix::prelude::*;
use std::collections::HashMap;

pub enum UserState {
    Main,
    List {
        rooms: Vec<usize>,
    },
    Room {
        game_state: Box<dyn mighty::MightyState>,
        room: usize,
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
    pub name: String,
}

pub struct User {
    rooms: HashMap<usize, Addr<room::Room>>,
    server: Addr<server::MainServer>,
    name: String,
    state: UserState,
    session: Addr<session::WsSession>
}

impl Actor for User {
    type Context = Context<User>;
}

impl Handler<Connect> for User {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.name = msg.name;
    }
}

impl User {
    pub fn new(server: Addr<server::MainServer>, session: Addr<session::WsSession>) -> User {
        User {
            rooms: HashMap::new(),
            server,
            name: "".to_owned(),
            state: UserState::Main,
            session,
        }
    }
}
