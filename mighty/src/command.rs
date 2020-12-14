use crate::card::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Command {
    StartGame,
    //Some(giruda, pledge) or None
    Pledge(Option<(Option<Pattern>, u8)>),
    Random,
}
