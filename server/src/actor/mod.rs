mod chat_ss;
mod hub;
mod list_ss;
mod main_ss;
mod observe_ss;
mod room;
mod room_ss;
mod user;

pub use chat_ss::ChatSession;
pub use hub::Hub;
pub use list_ss::ListSession;
pub use main_ss::MainSession;
pub use observe_ss::ObserveSession;
pub use room_ss::RoomSession;

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
