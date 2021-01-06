use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Config, Hash, Eq, PartialEq)]
pub struct Pledge {
    pub min: u8,
    pub max: u8,
    pub no_giruda_offset: i8,
    pub change_cost: u8,
    pub first_offset: i8,
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
