use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
#[cfg(feature = "client")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, Hash, Ord, PartialOrd)]
pub enum Pattern {
    #[serde(rename = "s")]
    Spade,
    #[serde(rename = "d")]
    Diamond,
    #[serde(rename = "h")]
    Heart,
    #[serde(rename = "c")]
    Clover,
}

impl TryFrom<Card> for Pattern {
    type Error = &'static str;

    fn try_from(c: Card) -> Result<Self, Self::Error> {
        if let Card::Normal(p, _) = c {
            Ok(p)
        } else {
            Err("Joker has no pattern")
        }
    }
}

#[cfg_attr(feature = "client", wasm_bindgen)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, Hash, Ord, PartialOrd)]
pub enum Color {
    #[serde(rename = "b")]
    Black,
    #[serde(rename = "r")]
    Red,
}

impl From<Pattern> for Color {
    fn from(c: Pattern) -> Self {
        match c {
            Pattern::Spade | Pattern::Clover => Self::Black,
            Pattern::Diamond | Pattern::Heart => Self::Red,
        }
    }
}

impl From<Rush> for Color {
    fn from(c: Rush) -> Self {
        if (Rush::SPADE | Rush::CLOVER).contains(c) {
            Self::Black
        } else {
            Self::Red
        }
    }
}

impl Color {
    pub fn is_color_of(&self, rhs: &Pattern) -> bool {
        match self {
            Color::Black => matches!(rhs, Pattern::Spade | Pattern::Clover),
            Color::Red => matches!(rhs, Pattern::Diamond | Pattern::Heart),
        }
    }

    pub fn invert(&self) -> Color {
        match self {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        }
    }
}

bitflags! {
    #[cfg_attr(feature = "client", wasm_bindgen)]
    #[derive(Serialize, Deserialize)]
    pub struct Rush: u8 {
        const SPADE   = 0b0001;
        const DIAMOND = 0b0010;
        const HEART   = 0b0100;
        const CLOVER  = 0b1000;
    }
}

impl From<Pattern> for Rush {
    fn from(p: Pattern) -> Self {
        match p {
            Pattern::Spade => Rush::SPADE,
            Pattern::Diamond => Rush::DIAMOND,
            Pattern::Heart => Rush::HEART,
            Pattern::Clover => Rush::CLOVER,
        }
    }
}

impl From<Color> for Rush {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => Rush::black(),
            Color::Red => Rush::red(),
        }
    }
}

impl From<Card> for Rush {
    fn from(c: Card) -> Self {
        match c {
            Card::Normal(t, _) => Self::from(t),
            Card::Joker(t) => Self::from(t),
        }
    }
}

impl Rush {
    pub fn black() -> Rush {
        Rush::SPADE | Rush::CLOVER
    }

    pub fn red() -> Rush {
        Rush::DIAMOND | Rush::HEART
    }

    pub fn any() -> Rush {
        Rush::all()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug, Hash, Ord, PartialOrd)]
#[serde(untagged)]
pub enum Card {
    Normal(Pattern, u8),
    Joker(Color),
}

impl Card {
    pub fn is_score(&self) -> bool {
        match self {
            Card::Normal(_, n) => *n >= 10,
            Card::Joker(_) => false,
        }
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Card::Joker(_))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pattern_card_from() {
        assert_eq!(Pattern::try_from(Card::Normal(Pattern::Spade, 14)), Ok(Pattern::Spade));
        assert!(Pattern::try_from(Card::Joker(Color::Black)).is_err());
        assert!(Pattern::try_from(Card::Joker(Color::Red)).is_err());
    }
    #[test]
    fn color_type_from() {
        assert_eq!(Color::from(Pattern::Spade), Color::Black);
        assert_eq!(Color::from(Pattern::Diamond), Color::Red);
        assert_eq!(Color::from(Pattern::Heart), Color::Red);
        assert_eq!(Color::from(Pattern::Clover), Color::Black);
    }

    #[test]
    fn color_type_contains() {
        assert!(Color::Black.is_color_of(&Pattern::Spade));
        assert!(Color::Red.is_color_of(&Pattern::Diamond));
        assert!(Color::Red.is_color_of(&Pattern::Heart));
        assert!(Color::Black.is_color_of(&Pattern::Clover));
    }

    #[test]
    fn rush_pattern_from() {
        assert_eq!(Rush::from(Pattern::Spade), Rush::SPADE);
        assert_eq!(Rush::from(Pattern::Diamond), Rush::DIAMOND);
        assert_eq!(Rush::from(Pattern::Heart), Rush::HEART);
        assert_eq!(Rush::from(Pattern::Clover), Rush::CLOVER);
    }

    #[test]
    fn rush_color_from() {
        assert_eq!(Rush::from(Color::Black), Rush::black());
        assert_eq!(Rush::from(Color::Red), Rush::red());
    }

    #[test]
    fn color_rush_from() {
        assert_eq!(Color::from(Rush::black()), Color::Black);
        assert_eq!(Color::from(Rush::red()), Color::Red);
        assert_eq!(Color::from(Rush::SPADE), Color::Black);
        assert_eq!(Color::from(Rush::CLOVER), Color::Black);
        assert_eq!(Color::from(Rush::DIAMOND), Color::Red);
        assert_eq!(Color::from(Rush::HEART), Color::Red);
    }

    #[test]
    fn rush_card_from() {
        assert_eq!(Rush::from(Card::Normal(Pattern::Spade, 2)), Rush::SPADE);
        assert_eq!(Rush::from(Card::Normal(Pattern::Diamond, 3)), Rush::DIAMOND);
        assert_eq!(Rush::from(Card::Normal(Pattern::Clover, 4)), Rush::CLOVER);
        assert_eq!(Rush::from(Card::Normal(Pattern::Heart, 5)), Rush::HEART);
        assert_eq!(Rush::from(Card::Joker(Color::Red)), Rush::red());
        assert_eq!(Rush::from(Card::Joker(Color::Black)), Rush::black());
    }

    #[test]
    fn card_is_score() {
        assert!(Card::Normal(Pattern::Spade, 10).is_score());
        assert!(!Card::Normal(Pattern::Spade, 9).is_score());
        assert!(!Card::Normal(Pattern::Diamond, 8).is_score());
        assert!(!Card::Joker(Color::Red).is_score());
    }

    #[test]
    fn card_is_joker() {
        assert!(Card::Joker(Color::Red).is_joker());
        assert!(Card::Joker(Color::Black).is_joker());
        assert!(!Card::Normal(Pattern::Spade, 5).is_joker());
    }
}
