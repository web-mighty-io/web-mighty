#![cfg(not(tarpaulin_include))]

#[cfg(feature = "server")]
use actix::prelude::*;
use bitflags::bitflags;
use mighty::prelude::{Command, Rule, State};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Unique room id.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RoomUuid(pub Uuid);

impl From<Uuid> for RoomUuid {
    fn from(u: Uuid) -> Self {
        RoomUuid(u)
    }
}

/// Short 6-digit room id during room is alive.
/// If room is removed, then this id is useless and can be representing other room.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RoomId(pub u32);

impl From<u32> for RoomId {
    fn from(u: u32) -> Self {
        RoomId(u)
    }
}

/// Unique game id.
/// Every game would have game id even though they're in same room.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct GameId(pub Uuid);

impl From<Uuid> for GameId {
    fn from(u: Uuid) -> Self {
        GameId(u)
    }
}

/// Unique user id.
/// It starts with 10.
/// If the value is 0, it represents no user.
/// If the value if 1~9, it will be bot(ghost).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct UserNo(pub u32);

impl From<u32> for UserNo {
    fn from(u: u32) -> Self {
        UserNo(u)
    }
}

/// Unique rule hash
/// It uses sha256 to hash.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RuleHash(pub String);

impl RuleHash {
    pub fn from_rule(r: &Rule) -> Self {
        let mut hasher = Sha256::new();
        let s = serde_json::to_string(r).unwrap();
        hasher.update(s.as_bytes());
        let res = hasher.finalize();
        RuleHash(hex::encode(res))
    }
}

/// Token for mail
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct Token(pub Uuid);

impl From<Uuid> for Token {
    fn from(u: Uuid) -> Self {
        Token(u)
    }
}

/// Information of room
///
/// - `uuid`: uuid of room
/// - `id`: id of room
/// - `name`: name of room
/// - `rule`: mighty rule of room
/// - `is_rank`: if this room is rank
/// - `head`: head of this room
/// - `user`: user list who plays game
/// - `observer`: observer list
/// - `is_game`: if room is on gaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message, MessageResponse))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub struct RoomInfo {
    pub uuid: RoomUuid,
    pub id: RoomId,
    pub name: String,
    pub rule: RuleHash,
    pub is_rank: bool,
    pub head: UserNo,
    pub user: Vec<UserNo>,
    pub observer_cnt: usize,
    pub is_game: bool,
}

/// Simplified information of room for in the list
///
/// - `id`: id of room
/// - `name`: name of room
/// - `rule_name`: name of rule
/// - `is_rank`: if this room is rank
/// - `user_cnt`: count of users
/// - `observer_cnt`: count of observers
/// - `is_game`: if this room is on gaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message, MessageResponse))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub struct SimpleRoomInfo {
    pub id: RoomId,
    pub name: String,
    pub rule: RuleHash,
    pub is_rank: bool,
    pub user_cnt: usize,
    pub observer_cnt: usize,
    pub is_game: bool,
}

impl From<RoomInfo> for SimpleRoomInfo {
    fn from(info: RoomInfo) -> Self {
        SimpleRoomInfo {
            id: info.id,
            name: info.name,
            rule: info.rule,
            is_rank: info.is_rank,
            user_cnt: info.user.len(),
            observer_cnt: info.observer_cnt,
            is_game: info.is_game,
        }
    }
}

/// Information of user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message, MessageResponse))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub struct UserInfo {
    pub no: UserNo,
    pub id: String,
    pub name: String,
    pub rating: u32,
    pub room: Option<RoomId>,
    pub is_admin: bool,
}

bitflags! {
    /// Status of user
    ///
    /// - `ROOM_MASK`: used internal; for masking room related bits
    /// - `ROOM_DISCONN`: if the user is in game but the user left, this would be activated
    /// - `IN_GAME`: if the user is in game
    /// - `IN_ROOM`: if the user is in room
    /// - `ONLINE`: if the user is online
    /// - `ABSENT`: if the user is online but doing nothing for `ABSENT_TIME`
    /// - `DISCONNECTED`: right after user disconnected
    /// - `OFFLINE`: if the user is disconnected for `RECONNECTION_TIME`
    #[derive(Serialize, Deserialize)]
    pub struct UserStatus: u8 {
        const ROOM_MASK    = 0b11100;
        const ROOM_DISCONN = 0b11100;
        const IN_GAME      = 0b01100;
        const IN_ROOM      = 0b00100;
        const ONLINE       = 0b00011;
        const ABSENT       = 0b00010;
        const DISCONNECTED = 0b00001;
        const OFFLINE      = 0b00000;
    }
}

/// Websocket message for room listing to client
///
/// - `Room`: Sends the info of room
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum ListToClient {
    Room(SimpleRoomInfo),
}

/// Websocket message for room listing to server
///
/// - `Subscribe`: Subscribe for changes in `room_id`
/// - `Unsubscribe`: Unsubscribe for changes in `room_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListToServer {
    Subscribe(RoomId),
    Unsubscribe(RoomId),
}

/// Websocket message for main connection to client
///
/// - `UserStatus`: Sends the status of user
/// - `UserInfo`: Sends the information of user
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum MainToClient {
    UserStatus(UserNo, UserStatus),
    UserInfo(UserInfo),
}

/// Websocket message for main connection to server
///
/// - `Subscribe`: Subscribe for changes of user state
/// - `Unsubscribe`: Unsubscribe for changes of user state
/// - `GetInfo`: Request user info
/// - `Update`: Check for user movement
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MainToServer {
    Subscribe(UserNo),
    Unsubscribe(UserNo),
    Update,
}

/// Websocket message for observer connection to client
///
/// - `Room`: Information of room
/// - `Game`: Information of game
/// - `Chat`: For receiving chats
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum ObserveToClient {
    Room(RoomInfo),
    Game(State),
}

/// Websocket message for observer connection to server
///
/// - `Chat`: When observer chat
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObserveToServer;

/// Websocket message for room connection to client
///
/// - `Room`: Information of room
/// - `Game`: Information of game
/// - `Chat`: For receiving chats
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(Message))]
#[cfg_attr(feature = "server", rtype(result = "()"))]
pub enum RoomUserToClient {
    Room(RoomInfo),
    Game(State),
}

/// Websocket message for room connection to server
///
/// - `Start`: Starts the game
/// - `ChangeName`: Change the name of the room
/// - `ChangeRule`: Change the rule of the room
/// - `Command`: Command for next move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomUserToServer {
    Start,
    ChangeName(String),
    ChangeRule(Rule),
    Command(Command),
}
