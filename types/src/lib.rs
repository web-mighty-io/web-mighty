#[cfg(feature = "server")]
use actix::prelude::*;
use bitflags::bitflags;
use mighty::prelude::{Command, Rule, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RoomId(pub Uuid);

impl From<Uuid> for RoomId {
    fn from(u: Uuid) -> Self {
        RoomId(u)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct GameId(pub Uuid);

impl From<Uuid> for GameId {
    fn from(u: Uuid) -> Self {
        GameId(u)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct UserNo(pub u32);

impl From<u32> for UserNo {
    fn from(u: u32) -> Self {
        UserNo(u)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct Token(pub Uuid);

impl From<Uuid> for Token {
    fn from(u: Uuid) -> Self {
        Token(u)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message, MessageResponse))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub struct RoomInfo {
    pub id: RoomId,
    pub name: String,
    pub rule: Rule,
    pub is_rank: bool,
    pub head: UserNo,
    pub user: Vec<UserNo>,
    pub observer_cnt: usize,
    pub is_game: bool,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct UserStatus: u8 {
        const ROOM_MASK    = 0b1100;
        const IN_GAME      = 0b1100;
        const IN_ROOM      = 0b0100;
        const ONLINE       = 0b0011;
        const ABSENT       = 0b0010;
        const DISCONNECTED = 0b0001;
        const OFFLINE      = 0b0000;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum ListToClient {
    Room(RoomInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListToServer {
    Subscribe(RoomId),
    Unsubscribe(RoomId),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub struct MainToClient {
    pub user_no: UserNo,
    pub status: UserStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MainToServer {
    Subscribe(UserNo),
    Unsubscribe(UserNo),
    Update,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum ObserveToClient {
    Room(RoomInfo),
    Game(State),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObserveToServer;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum RoomUserToClient {
    Room(RoomInfo),
    Game(State),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RoomUserToServer {
    Start,
    ChangeName(String),
    ChangeRule(Rule),
    Command(Command),
}
