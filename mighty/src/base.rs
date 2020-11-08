use crate::basic::BasicState;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Copy)]
pub enum CardType {
    Spade,
    Diamond,
    Heart,
    Clover,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Copy)]
pub enum ColorType {
    Black,
    Red,
}

impl From<CardType> for ColorType {
    fn from(c: CardType) -> Self {
        match c {
            CardType::Spade | CardType::Clover => Self::Black,
            CardType::Diamond | CardType::Heart => Self::Red,
        }
    }
}

impl From<RushType> for ColorType {
    fn from(c: RushType) -> Self {
        match c {
            RushType::Spade | RushType::Clover | RushType::Black => Self::Black,
            RushType::Diamond | RushType::Heart | RushType::Red => Self::Red,
        }
    }
}

impl ColorType {
    pub fn contains(&self, rhs: &CardType) -> bool {
        match self {
            ColorType::Black => matches!(rhs, CardType::Spade | CardType::Clover),
            ColorType::Red => matches!(rhs, CardType::Diamond | CardType::Heart),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Copy)]
pub enum RushType {
    Spade,
    Diamond,
    Heart,
    Clover,
    Red,
    Black,
}

impl From<CardType> for RushType {
    fn from(c: CardType) -> Self {
        match c {
            CardType::Spade => Self::Spade,
            CardType::Diamond => Self::Diamond,
            CardType::Heart => Self::Heart,
            CardType::Clover => Self::Clover,
        }
    }
}

impl From<ColorType> for RushType {
    fn from(c: ColorType) -> Self {
        match c {
            ColorType::Black => Self::Black,
            ColorType::Red => Self::Red,
        }
    }
}

impl From<Card> for RushType {
    fn from(c: Card) -> Self {
        match c {
            Card::Normal(t, _) => Self::from(t),
            Card::Joker(t) => Self::from(t),
        }
    }
}

impl RushType {
    pub fn contains(&self, c: &CardType) -> bool {
        Self::from(*c) == *self || Self::from(ColorType::from(*c)) == *self
    }

    pub fn is_same_type(&self, c: &Card) -> bool {
        let r = Self::from(c.clone());
        if *self == r {
            return true;
        }
        match *self {
            RushType::Red => ColorType::from(r) == ColorType::Red,
            RushType::Diamond | RushType::Heart => r == RushType::Red,
            RushType::Black => ColorType::from(r) == ColorType::Black,
            RushType::Spade | RushType::Clover => r == RushType::Black,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Card {
    Normal(CardType, u8),
    Joker(ColorType),
}

impl Card {
    /// New card deck (not shuffled)
    pub fn new_deck() -> Vec<Card> {
        let mut v = Vec::with_capacity(54);

        for i in 0..13 {
            v.push(Card::Normal(CardType::Spade, i));
        }
        for i in 0..13 {
            v.push(Card::Normal(CardType::Diamond, i));
        }
        for i in 0..13 {
            v.push(Card::Normal(CardType::Heart, i));
        }
        for i in 0..13 {
            v.push(Card::Normal(CardType::Clover, i));
        }

        v.push(Card::Joker(ColorType::Black));
        v.push(Card::Joker(ColorType::Red));

        v
    }

    pub fn is_score(&self) -> bool {
        match self {
            Card::Normal(_, n) => *n >= 9 || *n == 0,
            Card::Joker(_) => false,
        }
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Card::Joker(_))
    }
}

pub trait MightyState {
    fn next(&self, cmd: Vec<u8>) -> Result<Box<dyn MightyState>>;

    fn generate(&self, user: usize) -> Box<dyn MightyState>;
}

pub struct MightyGame {
    state: Vec<Box<dyn MightyState>>,
}

impl Default for MightyGame {
    fn default() -> Self {
        Self::new()
    }
}

impl MightyGame {
    pub fn new() -> MightyGame {
        MightyGame::with("basic")
    }

    // todo: implement when other rule is implemented
    pub fn with<S: AsRef<str>>(_: S) -> MightyGame {
        MightyGame {
            state: vec![Box::new(BasicState::new())],
        }
    }

    pub fn next(&mut self, cmd: Vec<u8>) -> Result<()> {
        let next_state = self.state.last().unwrap().next(cmd)?;
        self.state.push(next_state);
        Ok(())
    }

    pub fn generate(&self, user: usize) -> Box<dyn MightyState> {
        self.state.last().unwrap().generate(user)
    }

    pub fn last(&self) -> &Box<dyn MightyState> {
        self.state.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_type_from() {
        assert_eq!(ColorType::from(CardType::Spade), ColorType::Black);
        assert_eq!(ColorType::from(CardType::Diamond), ColorType::Red);
        assert_eq!(ColorType::from(CardType::Heart), ColorType::Red);
        assert_eq!(ColorType::from(CardType::Clover), ColorType::Black);
        assert_eq!(ColorType::from(RushType::Spade), ColorType::Black);
        assert_eq!(ColorType::from(RushType::Diamond), ColorType::Red);
        assert_eq!(ColorType::from(RushType::Heart), ColorType::Red);
        assert_eq!(ColorType::from(RushType::Clover), ColorType::Black);
        assert_eq!(ColorType::from(RushType::Black), ColorType::Black);
        assert_eq!(ColorType::from(RushType::Red), ColorType::Red);
    }

    #[test]
    fn color_type_contains() {
        assert!(ColorType::Black.contains(&CardType::Spade));
        assert!(ColorType::Red.contains(&CardType::Diamond));
        assert!(ColorType::Red.contains(&CardType::Heart));
        assert!(ColorType::Black.contains(&CardType::Clover));
    }

    #[test]
    fn rush_type_from() {
        assert_eq!(RushType::from(CardType::Spade), RushType::Spade);
        assert_eq!(RushType::from(CardType::Diamond), RushType::Diamond);
        assert_eq!(RushType::from(CardType::Heart), RushType::Heart);
        assert_eq!(RushType::from(CardType::Clover), RushType::Clover);

        assert_eq!(RushType::from(ColorType::Black), RushType::Black);
        assert_eq!(RushType::from(ColorType::Red), RushType::Red);

        assert_eq!(RushType::from(Card::Normal(CardType::Spade, 0)), RushType::Spade);
        assert_eq!(RushType::from(Card::Normal(CardType::Diamond, 0)), RushType::Diamond);
        assert_eq!(RushType::from(Card::Normal(CardType::Heart, 0)), RushType::Heart);
        assert_eq!(RushType::from(Card::Normal(CardType::Clover, 0)), RushType::Clover);
        assert_eq!(RushType::from(Card::Joker(ColorType::Black)), RushType::Black);
        assert_eq!(RushType::from(Card::Joker(ColorType::Red)), RushType::Red);
    }

    #[test]
    fn rush_type_contains() {
        assert!(RushType::Spade.contains(&CardType::Spade));
        assert!(RushType::Diamond.contains(&CardType::Diamond));
        assert!(RushType::Heart.contains(&CardType::Heart));
        assert!(RushType::Clover.contains(&CardType::Clover));

        assert!(RushType::Black.contains(&CardType::Spade));
        assert!(RushType::Red.contains(&CardType::Diamond));
        assert!(RushType::Red.contains(&CardType::Heart));
        assert!(RushType::Black.contains(&CardType::Clover));
    }

    #[test]
    fn card_new_deck() {
        let v = Card::new_deck();
        assert_eq!(v.len(), 54);

        assert_eq!(v[0], Card::Normal(CardType::Spade, 0));
        assert_eq!(v[13], Card::Normal(CardType::Diamond, 0));
        assert_eq!(v[26], Card::Normal(CardType::Heart, 0));
        assert_eq!(v[39], Card::Normal(CardType::Clover, 0));
        assert_eq!(v[53], Card::Joker(ColorType::Red));
    }

    #[test]
    fn card_is_score() {
        assert_eq!(Card::Normal(CardType::Spade, 9).is_score(), true);
        assert_eq!(Card::Normal(CardType::Diamond, 8).is_score(), false);
        assert_eq!(Card::Joker(ColorType::Red).is_score(), false);
    }

    #[test]
    fn card_is_joker() {
        assert_eq!(Card::Joker(ColorType::Red).is_joker(), true);
        assert_eq!(Card::Normal(CardType::Spade, 5).is_joker(), false);
    }
}
