use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Deserialize, Serialize)]
    pub struct Visibility: u8 {
        const PRESIDENT = 0b001;
        const FRIEND    = 0b010;
        const OTHER     = 0b100;
    }
}
