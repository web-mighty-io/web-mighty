use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum CardType {
    Spade,
    Diamond,
    Heart,
    Clover,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParseCardTypeError;

impl FromStr for CardType {
    type Err = ParseCardTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "spade" => Ok(Self::Spade),
            "d" | "diamond" => Ok(Self::Diamond),
            "h" | "heart" => Ok(Self::Heart),
            "c" | "clover" => Ok(Self::Clover),
            _ => Err(ParseCardTypeError),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ColorType {
    Black,
    Red,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParseColorTypeError;

impl FromStr for ColorType {
    type Err = ParseColorTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "black" => Ok(Self::Black),
            "r" | "red" => Ok(Self::Red),
            _ => Err(ParseColorTypeError),
        }
    }
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

#[derive(PartialEq, Clone, Debug)]
pub enum RushType {
    Spade,
    Diamond,
    Heart,
    Clover,
    Red,
    Black,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParseRushTypeError;

impl FromStr for RushType {
    type Err = ParseRushTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "spade" => Ok(Self::Spade),
            "d" | "diamond" => Ok(Self::Diamond),
            "h" | "heart" => Ok(Self::Heart),
            "c" | "clover" => Ok(Self::Clover),
            "r" | "red" => Ok(Self::Red),
            "b" | "black" => Ok(Self::Black),
            _ => Err(ParseRushTypeError),
        }
    }
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
        Self::from(c.clone()).eq(self) || Self::from(ColorType::from(c.clone())).eq(self)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Card {
    Normal(CardType, u8),
    Joker(ColorType),
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParseCardError;

impl From<ParseCardTypeError> for ParseCardError {
    fn from(_: ParseCardTypeError) -> Self {
        Self
    }
}

impl From<ParseColorTypeError> for ParseCardError {
    fn from(_: ParseColorTypeError) -> Self {
        Self
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.get(0..1).ok_or(ParseCardError)? {
            "n" => {
                let num = s.get(2..3).ok_or(ParseCardError)?;
                let num = u8::from_str_radix(num, 13).map_err(|_| ParseCardError)?;
                Ok(Self::Normal(
                    s.get(1..2).ok_or(ParseCardError)?.parse::<CardType>()?,
                    num,
                ))
            }
            "j" => Ok(Self::Joker(
                s.get(1..).ok_or(ParseCardError)?.parse::<ColorType>()?,
            )),
            _ => Err(ParseCardError),
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
}

/// type of friend making
#[derive(PartialEq, Clone, Debug)]
pub enum FriendFunc {
    None,
    ByCard(Card),
    ByUser(usize),
    ByWinning(u8),
}

#[derive(PartialEq, Clone, Debug)]
pub enum GameError {
    CommandError(String),
    InternalError(String),
}

pub trait GameTrait {
    type State;

    // first argument in instruction is user id (always in bound)
    fn process(&self, args: Vec<String>) -> Result<Self::State, GameError>;
}

#[cfg(test)]
mod base_tests {
    use super::*;

    #[test]
    fn card_type_from_str_test() {
        assert_eq!(CardType::from_str("s"), Ok(CardType::Spade));
        assert_eq!(CardType::from_str("d"), Ok(CardType::Diamond));
        assert_eq!(CardType::from_str("h"), Ok(CardType::Heart));
        assert_eq!(CardType::from_str("c"), Ok(CardType::Clover));

        assert_eq!(CardType::from_str("spade"), Ok(CardType::Spade));
        assert_eq!(CardType::from_str("diamond"), Ok(CardType::Diamond));
        assert_eq!(CardType::from_str("heart"), Ok(CardType::Heart));
        assert_eq!(CardType::from_str("clover"), Ok(CardType::Clover));

        assert_eq!(CardType::from_str("hello"), Err(ParseCardTypeError));
    }

    #[test]
    fn color_type_from_str_test() {
        assert_eq!(ColorType::from_str("r"), Ok(ColorType::Red));
        assert_eq!(ColorType::from_str("b"), Ok(ColorType::Black));

        assert_eq!(ColorType::from_str("red"), Ok(ColorType::Red));
        assert_eq!(ColorType::from_str("black"), Ok(ColorType::Black));

        assert_eq!(ColorType::from_str("hello"), Err(ParseColorTypeError));
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
    fn rush_type_from_str_test() {
        assert_eq!(RushType::from_str("s"), Ok(RushType::Spade));
        assert_eq!(RushType::from_str("d"), Ok(RushType::Diamond));
        assert_eq!(RushType::from_str("h"), Ok(RushType::Heart));
        assert_eq!(RushType::from_str("c"), Ok(RushType::Clover));
        assert_eq!(RushType::from_str("r"), Ok(RushType::Red));
        assert_eq!(RushType::from_str("b"), Ok(RushType::Black));

        assert_eq!(RushType::from_str("spade"), Ok(RushType::Spade));
        assert_eq!(RushType::from_str("diamond"), Ok(RushType::Diamond));
        assert_eq!(RushType::from_str("heart"), Ok(RushType::Heart));
        assert_eq!(RushType::from_str("clover"), Ok(RushType::Clover));
        assert_eq!(RushType::from_str("red"), Ok(RushType::Red));
        assert_eq!(RushType::from_str("black"), Ok(RushType::Black));

        assert_eq!(RushType::from_str("hello"), Err(ParseRushTypeError));
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
    fn card_from_str_test() {
        assert_eq!(Card::from_str("ns0"), Ok(Card::Normal(CardType::Spade, 0)));
        assert_eq!(
            Card::from_str("nd4"),
            Ok(Card::Normal(CardType::Diamond, 4))
        );
        assert_eq!(Card::from_str("nh9"), Ok(Card::Normal(CardType::Heart, 9)));
        assert_eq!(
            Card::from_str("ncc"),
            Ok(Card::Normal(CardType::Clover, 12))
        );

        assert_eq!(Card::from_str("jr"), Ok(Card::Joker(ColorType::Red)));
        assert_eq!(Card::from_str("jb"), Ok(Card::Joker(ColorType::Black)));

        assert_eq!(Card::from_str("nt0"), Err(ParseCardError));
        assert_eq!(Card::from_str("nsd"), Err(ParseCardError));
        assert_eq!(Card::from_str("p"), Err(ParseCardError));
        assert_eq!(Card::from_str(""), Err(ParseCardError));
        assert_eq!(Card::from_str("hello"), Err(ParseCardError));
        assert_eq!(Card::from_str("ja"), Err(ParseCardError));
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
}
