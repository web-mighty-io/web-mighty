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
use uuid::Uuid;

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct RoomId(pub Uuid);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct GameId(pub Uuid);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct UserNo(pub u32);

#[derive(Clone, Copy, MessageResponse, Eq, PartialEq, Hash)]
pub struct Token(pub Uuid);
