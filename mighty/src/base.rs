use crate::basic::BasicState;
use crate::error::Result;
use parse_display::{Display, FromStr, ParseError};

#[derive(PartialEq, Clone, Debug, Display, FromStr, Copy)]
pub enum CardType {
    #[display("s")]
    Spade,
    #[display("d")]
    Diamond,
    #[display("h")]
    Heart,
    #[display("c")]
    Clover,
}

#[derive(PartialEq, Clone, Debug, Display, FromStr, Copy)]
pub enum ColorType {
    #[display("b")]
    Black,
    #[display("r")]
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

#[derive(PartialEq, Clone, Debug, Display, FromStr, Copy)]
pub enum RushType {
    #[display("s")]
    Spade,
    #[display("d")]
    Diamond,
    #[display("h")]
    Heart,
    #[display("c")]
    Clover,
    #[display("r")]
    Red,
    #[display("b")]
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

#[derive(PartialEq, Clone, Debug, Display)]
pub enum Card {
    #[display("{0}{1:x}")]
    Normal(CardType, u8),
    #[display("j{0}")]
    Joker(ColorType),
}

impl std::str::FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.get(0..1).ok_or_else(ParseError::new)? {
            "s" | "d" | "h" | "c" => {
                let num = s.get(1..2).ok_or_else(ParseError::new)?;
                let num = u8::from_str_radix(num, 13).map_err(|_| ParseError::new())?;
                Ok(Self::Normal(
                    s.get(0..1)
                        .ok_or_else(ParseError::new)?
                        .parse::<CardType>()
                        .unwrap(),
                    num,
                ))
            }
            "j" => Ok(Self::Joker(
                s.get(1..)
                    .ok_or_else(ParseError::new)?
                    .parse::<ColorType>()?,
            )),
            _ => Err(ParseError::new()),
        }
    }
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
    fn next(&self, cmd: String) -> Result<Box<dyn MightyState>>;

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

    pub fn next(&mut self, cmd: String) -> Result<()> {
        let next_state = self.state.last().unwrap().next(cmd)?;
        self.state.push(next_state);
        Ok(())
    }

    pub fn generate(&self, user: usize) -> Box<dyn MightyState> {
        self.state.last().unwrap().generate(user)
    }
}

#[cfg(test)]
mod base_tests {
    use super::*;
    use parse_display::ParseError;

    #[test]
    fn card_type_from_str_test() {
        assert_eq!("s".parse(), Ok(CardType::Spade));
        assert_eq!("d".parse(), Ok(CardType::Diamond));
        assert_eq!("h".parse(), Ok(CardType::Heart));
        assert_eq!("c".parse(), Ok(CardType::Clover));

        assert_eq!("hello".parse::<CardType>(), Err(ParseError::new()));
    }

    #[test]
    fn card_type_display_test() {
        assert_eq!(CardType::Spade.to_string(), "s");
        assert_eq!(CardType::Diamond.to_string(), "d");
        assert_eq!(CardType::Heart.to_string(), "h");
        assert_eq!(CardType::Clover.to_string(), "c");
    }

    #[test]
    fn color_type_from_str_test() {
        assert_eq!("r".parse(), Ok(ColorType::Red));
        assert_eq!("b".parse(), Ok(ColorType::Black));

        assert_eq!("hello".parse::<ColorType>(), Err(ParseError::new()));
    }

    #[test]
    fn color_type_from_test() {
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
    fn color_type_contains_test() {
        assert!(ColorType::Black.contains(&CardType::Spade));
        assert!(ColorType::Red.contains(&CardType::Diamond));
        assert!(ColorType::Red.contains(&CardType::Heart));
        assert!(ColorType::Black.contains(&CardType::Clover));
    }

    #[test]
    fn color_type_display_test() {
        assert_eq!(ColorType::Red.to_string(), "r");
        assert_eq!(ColorType::Black.to_string(), "b");
    }

    #[test]
    fn rush_type_from_str_test() {
        assert_eq!("s".parse(), Ok(RushType::Spade));
        assert_eq!("d".parse(), Ok(RushType::Diamond));
        assert_eq!("h".parse(), Ok(RushType::Heart));
        assert_eq!("c".parse(), Ok(RushType::Clover));
        assert_eq!("r".parse(), Ok(RushType::Red));
        assert_eq!("b".parse(), Ok(RushType::Black));

        assert_eq!("hello".parse::<RushType>(), Err(ParseError::new()));
    }

    #[test]
    fn rush_type_from_test() {
        assert_eq!(RushType::from(CardType::Spade), RushType::Spade);
        assert_eq!(RushType::from(CardType::Diamond), RushType::Diamond);
        assert_eq!(RushType::from(CardType::Heart), RushType::Heart);
        assert_eq!(RushType::from(CardType::Clover), RushType::Clover);

        assert_eq!(RushType::from(ColorType::Black), RushType::Black);
        assert_eq!(RushType::from(ColorType::Red), RushType::Red);

        assert_eq!(
            RushType::from(Card::Normal(CardType::Spade, 0)),
            RushType::Spade
        );
        assert_eq!(
            RushType::from(Card::Normal(CardType::Diamond, 0)),
            RushType::Diamond
        );
        assert_eq!(
            RushType::from(Card::Normal(CardType::Heart, 0)),
            RushType::Heart
        );
        assert_eq!(
            RushType::from(Card::Normal(CardType::Clover, 0)),
            RushType::Clover
        );
        assert_eq!(
            RushType::from(Card::Joker(ColorType::Black)),
            RushType::Black
        );
        assert_eq!(RushType::from(Card::Joker(ColorType::Red)), RushType::Red);
    }

    #[test]
    fn rush_type_contains_test() {
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
    fn rush_type_display_test() {
        assert_eq!(RushType::Black.to_string(), "b");
        assert_eq!(RushType::Red.to_string(), "r");
        assert_eq!(RushType::Spade.to_string(), "s");
        assert_eq!(RushType::Diamond.to_string(), "d");
        assert_eq!(RushType::Heart.to_string(), "h");
        assert_eq!(RushType::Clover.to_string(), "c");
    }

    #[test]
    fn card_from_str_test() {
        assert_eq!("s0".parse(), Ok(Card::Normal(CardType::Spade, 0)));
        assert_eq!("d4".parse(), Ok(Card::Normal(CardType::Diamond, 4)));
        assert_eq!("h9".parse(), Ok(Card::Normal(CardType::Heart, 9)));
        assert_eq!("cc".parse(), Ok(Card::Normal(CardType::Clover, 12)));
        assert_eq!("jr".parse(), Ok(Card::Joker(ColorType::Red)));
        assert_eq!("jb".parse(), Ok(Card::Joker(ColorType::Black)));

        assert_eq!("t0".parse::<Card>(), Err(ParseError::new()));
        assert_eq!("sd".parse::<Card>(), Err(ParseError::new()));
        assert_eq!("p".parse::<Card>(), Err(ParseError::new()));
        assert_eq!("".parse::<Card>(), Err(ParseError::new()));
        assert_eq!("hello".parse::<Card>(), Err(ParseError::new()));
        assert_eq!("ja".parse::<Card>(), Err(ParseError::new()));
    }

    #[test]
    fn card_new_deck_test() {
        let v = Card::new_deck();
        assert_eq!(v.len(), 54);

        assert_eq!(v[0], Card::Normal(CardType::Spade, 0));
        assert_eq!(v[13], Card::Normal(CardType::Diamond, 0));
        assert_eq!(v[26], Card::Normal(CardType::Heart, 0));
        assert_eq!(v[39], Card::Normal(CardType::Clover, 0));
        assert_eq!(v[53], Card::Joker(ColorType::Red));
    }

    #[test]
    fn card_is_score_test() {
        assert_eq!(Card::Normal(CardType::Spade, 9).is_score(), true);
        assert_eq!(Card::Normal(CardType::Diamond, 8).is_score(), false);
        assert_eq!(Card::Joker(ColorType::Red).is_score(), false);
    }

    #[test]
    fn card_is_joker_test() {
        assert_eq!(Card::Joker(ColorType::Red).is_joker(), true);
        assert_eq!(Card::Normal(CardType::Spade, 5).is_joker(), false);
    }

    #[test]
    fn card_display_test() {
        assert_eq!(Card::Normal(CardType::Spade, 0).to_string(), "s0");
        assert_eq!(Card::Normal(CardType::Diamond, 5).to_string(), "d5");
        assert_eq!(Card::Normal(CardType::Heart, 8).to_string(), "h8");
        assert_eq!(Card::Normal(CardType::Clover, 12).to_string(), "cc");
        assert_eq!(Card::Joker(ColorType::Red).to_string(), "jr");
        assert_eq!(Card::Joker(ColorType::Black).to_string(), "jb");
    }
}
