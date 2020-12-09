use serde::{Deserialize, Serialize};
use crate::card::Pattern;

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Command {
    StartGame,
    //Some(giruda, pledge) or None
    Pledge(Option<(Option<Pattern>, u8)>),
    Random,
}