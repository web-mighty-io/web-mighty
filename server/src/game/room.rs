use crate::game::user;
use actix::prelude::*;
use bitflags::bitflags;
use mighty::MightyGame;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeName {
    pub user_no: u32,
    pub name: String,
}

#[derive(Clone, Message)]
#[rtype(result = "bool")]
pub struct AddUser {
    pub user_no: u32,
    pub addr: Addr<user::User>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct AddObserver {
    pub user_no: u32,
    pub addr: Addr<user::User>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct AddListObserver {
    pub user_no: u32,
    pub addr: Addr<user::User>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveUser {
    pub user_no: u32,
}

bitflags! {
    #[derive(Message)]
    #[rtype(result = "()")]
    pub struct NotifyRoomState: u8 {
        const USER         = 0b001;
        const OBSERVER     = 0b010;
        const LIST_OBSERVER = 0b100;
    }
}

#[derive(Clone, Message)]
#[rtype(result = "mighty::Result<()>")]
pub struct GameCommand {
    pub user_no: u32,
    pub content: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Chat {
    pub user_no: u32,
    pub content: String,
}

pub struct Room {
    name: String,
    users: Vec<u32>,
    users_map: HashMap<u32, usize>,
    game: MightyGame,
    observers: HashSet<u32>,
    list_observers: HashSet<u32>,
    session: HashMap<u32, Addr<user::User>>,
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
        if msg.user_no == 0 || msg.user_no == self.users[0] {
            self.name = msg.name;
        }
    }
}

impl Handler<AddUser> for Room {
    type Result = bool;

    fn handle(&mut self, msg: AddUser, _: &mut Self::Context) -> Self::Result {
        for (i, v) in self.users.iter_mut().enumerate() {
            if *v == 0 {
                *v = msg.user_no;
                self.users_map.insert(msg.user_no, i);
                self.session.insert(msg.user_no, msg.addr);
                return true;
            }
        }

        false
    }
}

impl Handler<AddObserver> for Room {
    type Result = ();

    fn handle(&mut self, msg: AddObserver, _: &mut Self::Context) -> Self::Result {
        self.observers.insert(msg.user_no);
        self.session.insert(msg.user_no, msg.addr);
    }
}

impl Handler<AddListObserver> for Room {
    type Result = ();

    fn handle(&mut self, msg: AddListObserver, _: &mut Self::Context) -> Self::Result {
        self.list_observers.insert(msg.user_no);
        self.session.insert(msg.user_no, msg.addr);
    }
}

impl Handler<RemoveUser> for Room {
    type Result = ();

    fn handle(&mut self, msg: RemoveUser, _: &mut Self::Context) -> Self::Result {
        self.observers.remove(&msg.user_no);
        self.session.remove(&msg.user_no);

        for i in self.users.iter_mut() {
            if *i == msg.user_no {
                *i = 0;
                self.users_map.remove(&msg.user_no);
                break;
            }
        }
    }
}

impl Handler<NotifyRoomState> for Room {
    type Result = ();

    fn handle(&mut self, msg: NotifyRoomState, _: &mut Self::Context) -> Self::Result {
        if msg.contains(NotifyRoomState::USER) {
            // for i in self.users.iter() {
            //     self.session.get(i).unwrap().do_send()
            // }
        }

        if msg.contains(NotifyRoomState::OBSERVER) {}

        if msg.contains(NotifyRoomState::LIST_OBSERVER) {}
    }
}

impl Handler<GameCommand> for Room {
    type Result = mighty::Result<()>;

    fn handle(&mut self, msg: GameCommand, _: &mut Self::Context) -> Self::Result {
        if let Some(i) = self.users_map.get(&msg.user_no) {
            self.game.next(*i, &*msg.content)
        } else {
            Err(mighty::Error::InvalidUser(0))
        }
    }
}

impl Handler<Chat> for Room {
    type Result = ();

    fn handle(&mut self, msg: Chat, _: &mut Self::Context) -> Self::Result {
        for i in self.users.iter() {
            self.session.get(&i).unwrap().do_send(user::ChatReceived {
                user_no: msg.user_no,
                content: msg.content.clone(),
            });
        }
    }
}

impl Room {
    fn new() -> Room {
        Room {
            name: "".to_owned(),
            users: vec![0; 5],
            users_map: HashMap::new(),
            game: MightyGame::new(),
            observers: HashSet::new(),
            list_observers: HashSet::new(),
            session: HashMap::new(),
        }
    }
}
