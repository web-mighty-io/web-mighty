use crate::card::{Card, Pattern};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Config, Hash, Eq, PartialEq)]
pub struct JokerCall {
    pub cards: Vec<(Card, Card)>,
    pub mighty_defense: bool,
    pub has_power: bool,
}

impl Default for JokerCall {
    fn default() -> Self {
        Self::new()
    }
}

impl JokerCall {
    pub fn new() -> JokerCall {
        JokerCall {
            cards: vec![(Card::Normal(Pattern::Clover, 2), Card::Normal(Pattern::Spade, 2))],
            mighty_defense: true,
            has_power: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
