use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Friend: u8 {
        const CARD  = 0b00001;
        const PICK  = 0b00010;
        const FIRST = 0b00100;
        const LAST  = 0b01000;
        const FAKE  = 0b10000;
    }
}
