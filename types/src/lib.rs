#![cfg(not(tarpaulin_include))]

use bitflags::bitflags;
use mighty::prelude::{Command, Rule, State};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
#[cfg(feature = "client")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "server")]
use {
    actix::prelude::*,
    sha2::{Digest, Sha256},
    std::str::FromStr,
    std::time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
struct CopyableHash(u64, u64, u64, u64);

#[cfg(feature = "server")]
impl FromStr for CopyableHash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 64 {
            let data = hex::decode(s)?;
            Ok(CopyableHash::from_vec(data))
        } else {
            Err(anyhow::Error::msg("invalid length"))
        }
    }
}

impl Display for CopyableHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_vec()))
    }
}

impl CopyableHash {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            ((self.0 & 0xff_00_00_00_00_00_00_00) >> 56) as u8,
            ((self.0 & 0x00_ff_00_00_00_00_00_00) >> 48) as u8,
            ((self.0 & 0x00_00_ff_00_00_00_00_00) >> 40) as u8,
            ((self.0 & 0x00_00_00_ff_00_00_00_00) >> 32) as u8,
            ((self.0 & 0x00_00_00_00_ff_00_00_00) >> 24) as u8,
            ((self.0 & 0x00_00_00_00_00_ff_00_00) >> 16) as u8,
            ((self.0 & 0x00_00_00_00_00_00_ff_00) >> 8) as u8,
            (self.0 & 0x00_00_00_00_00_00_00_ff) as u8,
            ((self.1 & 0xff_00_00_00_00_00_00_00) >> 56) as u8,
            ((self.1 & 0x00_ff_00_00_00_00_00_00) >> 48) as u8,
            ((self.1 & 0x00_00_ff_00_00_00_00_00) >> 40) as u8,
            ((self.1 & 0x00_00_00_ff_00_00_00_00) >> 32) as u8,
            ((self.1 & 0x00_00_00_00_ff_00_00_00) >> 24) as u8,
            ((self.1 & 0x00_00_00_00_00_ff_00_00) >> 16) as u8,
            ((self.1 & 0x00_00_00_00_00_00_ff_00) >> 8) as u8,
            (self.1 & 0x00_00_00_00_00_00_00_ff) as u8,
            ((self.2 & 0xff_00_00_00_00_00_00_00) >> 56) as u8,
            ((self.2 & 0x00_ff_00_00_00_00_00_00) >> 48) as u8,
            ((self.2 & 0x00_00_ff_00_00_00_00_00) >> 40) as u8,
            ((self.2 & 0x00_00_00_ff_00_00_00_00) >> 32) as u8,
            ((self.2 & 0x00_00_00_00_ff_00_00_00) >> 24) as u8,
            ((self.2 & 0x00_00_00_00_00_ff_00_00) >> 16) as u8,
            ((self.2 & 0x00_00_00_00_00_00_ff_00) >> 8) as u8,
            (self.2 & 0x00_00_00_00_00_00_00_ff) as u8,
            ((self.3 & 0xff_00_00_00_00_00_00_00) >> 56) as u8,
            ((self.3 & 0x00_ff_00_00_00_00_00_00) >> 48) as u8,
            ((self.3 & 0x00_00_ff_00_00_00_00_00) >> 40) as u8,
            ((self.3 & 0x00_00_00_ff_00_00_00_00) >> 32) as u8,
            ((self.3 & 0x00_00_00_00_ff_00_00_00) >> 24) as u8,
            ((self.3 & 0x00_00_00_00_00_ff_00_00) >> 16) as u8,
            ((self.3 & 0x00_00_00_00_00_00_ff_00) >> 8) as u8,
            (self.3 & 0x00_00_00_00_00_00_00_ff) as u8,
        ]
    }

    #[cfg(feature = "server")]
    fn from_vec(data: Vec<u8>) -> CopyableHash {
        CopyableHash(
            (data[0] as u64) << 56
                | (data[1] as u64) << 48
                | (data[2] as u64) << 40
                | (data[3] as u64) << 32
                | (data[4] as u64) << 24
                | (data[5] as u64) << 16
                | (data[6] as u64) << 8
                | data[7] as u64,
            (data[8] as u64) << 56
                | (data[9] as u64) << 48
                | (data[10] as u64) << 40
                | (data[11] as u64) << 32
                | (data[12] as u64) << 24
                | (data[13] as u64) << 16
                | (data[14] as u64) << 8
                | data[15] as u64,
            (data[16] as u64) << 56
                | (data[17] as u64) << 48
                | (data[18] as u64) << 40
                | (data[19] as u64) << 32
                | (data[20] as u64) << 24
                | (data[21] as u64) << 16
                | (data[22] as u64) << 8
                | data[23] as u64,
            (data[24] as u64) << 56
                | (data[25] as u64) << 48
                | (data[26] as u64) << 40
                | (data[27] as u64) << 32
                | (data[28] as u64) << 24
                | (data[29] as u64) << 16
                | (data[30] as u64) << 8
                | data[31] as u64,
        )
    }

    #[cfg(feature = "server")]
    pub fn generate<S: AsRef<str>>(s: S) -> CopyableHash {
        let mut hasher = Sha256::new();
        hasher.update(s.as_ref().as_bytes());
        let res = hasher.finalize();
        CopyableHash::from_vec(res[..].to_vec())
    }
}

/// Unique room id from sha256
#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RoomUid(CopyableHash);

#[cfg(feature = "server")]
impl FromStr for RoomUid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RoomUid(CopyableHash::from_str(s)?))
    }
}

impl Display for RoomUid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "server")]
impl RoomUid {
    pub fn generate<S: AsRef<str>>(s: S) -> RoomUid {
        RoomUid(CopyableHash::generate(s))
    }

    pub fn generate_random() -> RoomUid {
        RoomUid::generate(format!(
            "room-{}-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
            rand::random::<u8>()
        ))
    }
}

/// Short 6-digit room id during room is alive.
/// If room is removed, then this id is useless and can be representing other room.
#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RoomId(pub u32);

#[cfg(feature = "server")]
impl From<u32> for RoomId {
    fn from(u: u32) -> Self {
        RoomId(u)
    }
}

impl Display for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique game id.
/// Every game would have game id even though they're in same room.
#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct GameId(CopyableHash);

#[cfg(feature = "server")]
impl FromStr for GameId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GameId(CopyableHash::from_str(s)?))
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "server")]
impl GameId {
    pub fn generate<S: AsRef<str>>(s: S) -> GameId {
        GameId(CopyableHash::generate(s))
    }

    pub fn generate_random() -> GameId {
        GameId::generate(format!(
            "game-{}-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
            rand::random::<u8>()
        ))
    }
}

/// Unique user id.
/// It starts with 100.
/// If the value is 0, it represents no user.
/// If the value if 1~99, it will be bot(ghost).
#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct UserNo(pub u32);

#[cfg(feature = "server")]
impl From<u32> for UserNo {
    fn from(u: u32) -> Self {
        UserNo(u)
    }
}

impl Display for UserNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique rule hash
/// It uses sha256 to hash.
#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(MessageResponse))]
pub struct RuleHash(CopyableHash);

#[cfg(feature = "server")]
impl FromStr for RuleHash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RuleHash(CopyableHash::from_str(s)?))
    }
}

impl Display for RuleHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "server")]
impl RuleHash {
    pub fn generate(rule: &Rule) -> RuleHash {
        RuleHash(CopyableHash::generate(serde_json::to_string(rule).unwrap()))
    }
}

/// Information of room
///
/// - `uid`: uid of room
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
    pub uid: RoomUid,
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
    pub email: String,
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
    RoomList(Vec<RoomId>),
}

/// Websocket message for room listing to server
///
/// - `Subscribe`: Subscribe for changes in `room_id`
/// - `Unsubscribe`: Unsubscribe for changes in `room_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListToServer {
    Subscribe(RoomId),
    Unsubscribe(RoomId),
    GetRoomList { user_num: (u32, u32) },
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
    Chat(String, UserNo),
}

/// Websocket message for observer connection to server
///
/// - `Chat`: When observer chat
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ObserveToServer {
    Chat(String),
}

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
    Chat(String, UserNo),
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
    Chat(String),
}
