use crate::card::{Card, Pattern};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Config, Hash, Eq, PartialEq)]
pub struct JokerCall {
    cards: Vec<(Card, Card)>,
    mighty_defense: bool,
    has_power: bool,
}

impl Default for JokerCall {
    fn default() -> Self {
        Self::new()
    }
}

impl JokerCall {
    pub fn new() -> JokerCall {
        JokerCall {
            cards: vec![(Card::Normal(Pattern::Clover, 3), Card::Normal(Pattern::Spade, 3))],
            mighty_defense: true,
            has_power: false,
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
