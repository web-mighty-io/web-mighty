use crate::card::{Card, Color, Pattern};
use bitflags::_core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Preset {
    #[serde(rename = "f")]
    FullDeck,
    #[serde(rename = "o")]
    SingleJoker,
}

impl Preset {
    pub fn build(self) -> Deck {
        DeckBuilder::from(self).build()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeckBuilder(BTreeMap<Card, u8>);

impl From<Preset> for DeckBuilder {
    fn from(p: Preset) -> Self {
        match p {
            Preset::FullDeck => {
                let mut s = BTreeMap::new();

                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Spade, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Diamond, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Heart, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Clover, i), 1);
                }

                s.insert(Card::Joker(Color::Black), 1);
                s.insert(Card::Joker(Color::Red), 1);

                DeckBuilder(s)
            }
            Preset::SingleJoker => {
                let mut s = BTreeMap::new();

                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Spade, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Diamond, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Heart, i), 1);
                }
                for i in 2..15 {
                    s.insert(Card::Normal(Pattern::Clover, i), 1);
                }

                s.insert(Card::Joker(Color::Black), 1);

                DeckBuilder(s)
            }
        }
    }
}

impl FromStr for DeckBuilder {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Default for DeckBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckBuilder {
    pub fn new() -> Self {
        DeckBuilder(BTreeMap::new())
    }

    pub fn double(mut self) -> Self {
        for (_, v) in self.0.iter_mut() {
            *v *= 2;
        }
        self
    }

    pub fn remove(mut self, card: &Card) -> Self {
        if if let Some(v) = self.0.get_mut(card) {
            *v -= 1;
            *v == 0
        } else {
            false
        } {
            self.0.remove(card);
        }
        self
    }

    pub fn push(mut self, card: Card) -> Self {
        if let Some(v) = self.0.get_mut(&card) {
            *v += 1;
        } else {
            self.0.insert(card, 1);
        }
        self
    }

    pub fn set(mut self, card: Card, cnt: u8) -> Self {
        if cnt == 0 {
            self.0.remove(&card);
        } else if let Some(v) = self.0.get_mut(&card) {
            *v = cnt;
        } else {
            self.0.insert(card, cnt);
        }
        self
    }

    pub fn build(&self) -> Deck {
        let v = self
            .0
            .iter()
            .map(|(c, i)| vec![*c; *i as usize])
            .flatten()
            .collect::<Vec<_>>();
        let b = v.iter().fold(0, |mut x, c| {
            if let Card::Joker(c) = c {
                x |= match c {
                    Color::Black => 0b10,
                    Color::Red => 0b01,
                };
            }
            x
        });
        Deck(v, b)
    }
}

/// Deck structure
///
/// u8 is for joker:
/// 0b x x
///    | └ Is Red Joker valid
///    └-- Is Black Joker valid
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Deck(pub Vec<Card>, pub u8);
