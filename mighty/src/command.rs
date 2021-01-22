use crate::card::{Card, Pattern, Rush};
use crate::state::FriendFunc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Command {
    //Some(giruda, pledge) or None
    Pledge(Option<(Option<Pattern>, u8)>),
    //drop card, friend function
    SelectFriend(Vec<Card>, FriendFunc),
    ChangePledge(Option<Pattern>),
    Go(Card, Rush, bool),
    Random,
}
