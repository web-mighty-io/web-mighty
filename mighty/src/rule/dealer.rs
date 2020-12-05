use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Dealer {
    Friend,
    Winner,
    Random,
}
