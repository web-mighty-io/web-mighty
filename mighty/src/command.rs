use crate::card::{Card, Pattern};
use crate::rule::friend;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Command {
    StartGame,
    //Some(giruda, pledge) or None
    Pledge(Option<(Option<Pattern>, u8)>),
    //drop card, friend function
    SelectFriend(Vec<Card>, friend::FriendFunc),
    ChangePledge(Option<Pattern>),
    Random,
}
