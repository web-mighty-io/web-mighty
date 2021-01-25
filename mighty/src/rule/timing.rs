use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Config, Hash, Eq, PartialEq)]
pub struct Timing {
    pub election_one_turn: u8,
    pub election_total: u8,
    pub selectfriend_time: u8,
    pub ingame_one_turn: u8,
    pub ingame_total: u8,
}

impl Default for Timing {
    fn default() -> Self {
        Self::new()
    }
}

impl Timing {
    pub fn new() -> Timing {
        Timing {
            election_one_turn: 0u8,
            election_total: 0u8,
            selectfriend_time: 0u8,
            ingame_one_turn: 0u8,
            ingame_total: 0u8,
        }
    }
}
