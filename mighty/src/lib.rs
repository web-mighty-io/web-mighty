mod card;
mod command;
pub mod error;
#[cfg(feature = "server")]
mod game;
mod rule;
mod state;

pub mod prelude {
    pub use crate::card::{Card, Color, Pattern, Rush};
    pub use crate::command::Command;
    #[cfg(feature = "server")]
    pub use crate::game::Game;
    pub use crate::rule::prelude::*;
    pub use crate::state::{FriendFunc, State};
}
