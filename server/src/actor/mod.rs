pub mod db;
pub mod group;
pub mod hub;
pub mod list;
pub mod mail;
pub mod main;
pub mod observe;
pub mod room;
pub mod room_user;
pub mod session;
pub mod user;

pub use db::Database;
pub use hub::Hub;
pub use list::List;
pub use mail::Mail;
pub use main::Main;
pub use observe::Observe;
pub use room::Room;
pub use room_user::RoomUser;
pub use user::User;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, MessageResponse, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct RoomId(pub Uuid);

impl From<Uuid> for RoomId {
    fn from(u: Uuid) -> Self {
        RoomId(u)
    }
}

#[derive(Debug, Clone, Copy, MessageResponse, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct GameId(pub Uuid);

impl From<Uuid> for GameId {
    fn from(u: Uuid) -> Self {
        GameId(u)
    }
}

#[derive(Debug, Clone, Copy, MessageResponse, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct UserNo(pub u32);

impl From<u32> for UserNo {
    fn from(u: u32) -> Self {
        UserNo(u)
    }
}

#[derive(Debug, Clone, Copy, MessageResponse, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Token(pub Uuid);

impl From<Uuid> for Token {
    fn from(u: Uuid) -> Self {
        Token(u)
    }
}
