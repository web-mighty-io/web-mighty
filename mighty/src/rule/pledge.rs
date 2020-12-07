use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Config, Hash, Eq, PartialEq)]
pub struct Pledge {
    min: u8,
    max: u8,
    no_giruda_offset: i8,
    change_cost: u8,
    first_offset: i8,
}

impl Default for Pledge {
    fn default() -> Self {
        Self::new()
    }
}

impl Pledge {
    pub fn new() -> Pledge {
        Pledge {
            min: 13,
            max: 20,
            no_giruda_offset: -1,
            change_cost: 2,
            first_offset: 0,
        }
    }

    pub fn valid(&self) -> bool {
        self.min < self.max
    }
}
