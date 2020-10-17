use std::str::FromStr;
use std::{error, fmt};

#[derive(PartialEq, Clone)]
pub enum CardType {
    Spade,
    Diamond,
    Heart,
    Clover,
}

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

#[derive(PartialEq, Clone)]
pub enum ColorType {
    Black,
    Red,
}

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
            CardType::Spade => Self::Black,
            CardType::Diamond => Self::Red,
            CardType::Heart => Self::Red,
            CardType::Clover => Self::Black,
        }
    }
}

impl From<RushType> for ColorType {
    fn from(c: RushType) -> Self {
        match c {
            RushType::Spade => Self::Black,
            RushType::Diamond => Self::Red,
            RushType::Heart => Self::Red,
            RushType::Clover => Self::Black,
            RushType::Red => Self::Red,
            RushType::Black => Self::Black,
        }
    }
}

impl ColorType {
    pub fn contains(lhs: ColorType, rhs: CardType) -> bool {
        if lhs == ColorType::Black {
            rhs == CardType::Spade || rhs == CardType::Clover
        } else {
            rhs == CardType::Diamond || rhs == CardType::Heart
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum RushType {
    Spade,
    Diamond,
    Heart,
    Clover,
    Red,
    Black,
}

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

#[derive(PartialEq, Clone)]
pub enum Card {
    Normal(CardType, u8),
    Joker(ColorType),
}

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
                let num = u8::from_str_radix(num, 16).map_err(|_| ParseCardError)?;
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

        for i in 1..=13 {
            v.push(Card::Normal(CardType::Clover, i));
        }
        for i in 1..=13 {
            v.push(Card::Normal(CardType::Diamond, i));
        }
        for i in 1..=13 {
            v.push(Card::Normal(CardType::Heart, i));
        }
        for i in 1..=13 {
            v.push(Card::Normal(CardType::Clover, i));
        }

        v.push(Card::Joker(ColorType::Black));
        v.push(Card::Joker(ColorType::Red));

        v
    }
}

/// type of friend making
#[derive(Clone)]
pub enum FriendFunc {
    None,
    ByCard(Card),
    ByUser(usize),
    ByWinning(u8),
}

pub enum GameError {
    CommandError(String),
    InternalError(String),
}

pub trait GameTrait {
    type State;

    // first argument in instruction is user id (always in bound)
    fn process(&self, args: Vec<String>) -> Result<Self::State, GameError>;
}

impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::CommandError(s) => {
                write!(f, "{}", s)
            }
            GameError::InternalError(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::CommandError(s) => {
                write!(f, "{}", s)
            }
            GameError::InternalError(s) => {
                write!(f, "{}", s)
            }
        }
    }
}