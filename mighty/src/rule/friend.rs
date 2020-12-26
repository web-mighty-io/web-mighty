use crate::card::Card;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum FriendFunc {
    None,
    ByCard(Card),
    ByUser(usize),
    First,
    Last,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Friend: u8 {
        const CARD  = 0b000001;
        const PICK  = 0b000010;
        const FIRST = 0b000100;
        const LAST  = 0b001000;
        const FAKE  = 0b010000;
        const NONE  = 0b100000;
    }
}
