use crate::base::RushType;
use parse_display::{Display, ParseError};

#[derive(PartialEq, Clone, Debug, Display)]
pub enum Error {
    #[display("parse error")]
    ParseError,
    #[display("invalid command, expected: {0}")]
    InvalidCommand(&'static str),
    #[display("invalid pledge")]
    InvalidPledge(bool, u8),
    #[display("expected user {0}")]
    InvalidUser(usize),
    #[display("you are not the leader")]
    NotLeader,
    #[display("you are not the president")]
    NotPresident,
    #[display("the card is not in the deck")]
    NotInDeck,
    #[display("your card is not {0}")]
    WrongCardType(RushType),
    #[display("it is the same giruda")]
    SameGiruda,
    #[display("internal error: {0}")]
    Internal(&'static str),
    #[display("you can't place this card")]
    WrongCard,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl std::convert::From<ParseError> for Error {
    fn from(_: ParseError) -> Self {
        Error::ParseError
    }
}
