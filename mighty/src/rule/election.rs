use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Election: u8 {
        const INCREASING       = 0b0001;
        const ORDERED          = 0b0010;
        const PASS_FIRST       = 0b0100;
        const NO_GIRUDA_EXIST  = 0b1000;
    }
}
