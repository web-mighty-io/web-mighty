use crate::game::user;
use actix::prelude::*;
use mighty::MightyGame;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeName {
    pub name: String,
}

#[derive(Clone, Copy)]
pub enum UserType {
    Player,
    Observer,
}

#[derive(Clone, Message)]
#[rtype(result = "bool")]
pub struct AddUser {
    pub id: String,
    pub addr: Addr<user::User>,
    pub user_type: UserType,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveUser {
    pub id: String,
}

#[derive(Clone, Copy)]
pub enum CommandType {
    Game,
    Chat,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Command {
    pub id: String,
    pub command_type: CommandType,
    pub content: String,
}

pub struct Room {
    name: String,
    users: Vec<String>,
    game: MightyGame,
    observers: HashSet<String>,
    session: HashMap<String, Addr<user::User>>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Handler<ChangeName> for Room {
    type Result = ();

    fn handle(&mut self, msg: ChangeName, _: &mut Self::Context) -> Self::Result {
        self.name = msg.name;
    }
}

impl Handler<AddUser> for Room {
    type Result = bool;

    fn handle(&mut self, msg: AddUser, _: &mut Self::Context) -> Self::Result {
        match msg.user_type {
            UserType::Player => {
                for i in self.users.iter_mut() {
                    if i.is_empty() {
                        *i = msg.id.clone();
                        self.session.insert(msg.id.clone(), msg.addr);
                        return true;
                    }
                }

                false
            }
            UserType::Observer => {
                self.observers.insert(msg.id.clone());
                self.session.insert(msg.id.clone(), msg.addr);
                true
            }
        }
    }
}

impl Handler<RemoveUser> for Room {
    type Result = ();

    fn handle(&mut self, msg: RemoveUser, _: &mut Self::Context) -> Self::Result {
        self.observers.remove(&msg.id);
        self.session.remove(&msg.id);

        for i in self.users.iter_mut() {
            if *i == msg.id {
                i.clear();
            }
        }
    }
}

impl Room {
    fn new() -> Room {
        Room {
            name: "".to_owned(),
            users: vec!["".to_owned(); 5],
            game: MightyGame::new(),
            observers: Default::default(),
            session: Default::default(),
        }
    }
}
