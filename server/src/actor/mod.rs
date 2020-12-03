mod chat_ss;
mod room_ss;
mod list_ss;
mod server;
mod main_ss;
mod observe_ss;
mod room;
mod user;

pub use chat_ss::ChatSession;
pub use room_ss::RoomSession;
pub use list_ss::ListSession;
pub use server::Server;
pub use main_ss::MainSession;
pub use observe_ss::ObserveSession;

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
pub struct UserId(pub u32);
