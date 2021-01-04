pub mod db;
pub mod hub;
pub mod mail;
pub mod room;
pub mod user;

pub use db::Database;
pub use hub::Hub;
pub use mail::Mail;
pub use room::Room;
pub use user::User;

use actix::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct RoomId(pub Uuid);

impl From<Uuid> for RoomId {
    fn from(u: Uuid) -> Self {
        RoomId(u)
    }
}

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct GameId(pub Uuid);

impl From<Uuid> for GameId {
    fn from(u: Uuid) -> Self {
        GameId(u)
    }
}

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct UserNo(pub u32);

impl From<u32> for UserNo {
    fn from(u: u32) -> Self {
        UserNo(u)
    }
}

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct Token(pub Uuid);

impl From<Uuid> for Token {
    fn from(u: Uuid) -> Self {
        Token(u)
    }
}
