pub mod db;
mod hub;
mod mail;
mod room;
mod user;

pub use db::error;
pub use db::Database;
pub use hub::Hub;
pub use mail::Mail;
pub use room::Room;
pub use user::User;

use actix::prelude::*;
use std::time::Duration;
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
const LAST_ACTIVITY_INTERVAL: Duration = Duration::from_secs(30);
const CHECK_ACTIVITY_INTERVAL: Duration = Duration::from_secs(15);
const RECONNECTION_TIME: Duration = Duration::from_secs(10);
const ABSENT_TIME: Duration = Duration::from_secs(300);

const MAX_CHAT_HISTORY: usize = 50;

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct RoomId(pub Uuid);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct GameId(pub Uuid);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct UserNo(pub u32);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct Token(pub Uuid);
