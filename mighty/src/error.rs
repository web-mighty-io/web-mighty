#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    ParseError,
    InvalidCommand(&'static str),
    InvalidPledge(bool, u8),
    InvalidUser,
    NotLeader,
    NotPresident,
    NotInDeck,
    SameGiruda,
    WrongCardType,
    Internal(&'static str),
    WrongCard,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError => write!(f, "parse error"),
            Error::InvalidCommand(c) => write!(f, "invalid command, expected: {}", c),
            Error::InvalidPledge(..) => write!(f, "invalid pledge"),
            Error::InvalidUser => write!(f, "invalid user"),
            Error::NotLeader => write!(f, "you are not the leader"),
            Error::NotPresident => write!(f, "you are not the president"),
            Error::NotInDeck => write!(f, "the card is not in the deck"),
            Error::WrongCardType => write!(f, "your card is not"),
            Error::SameGiruda => write!(f, "same giruda"),
            Error::Internal(e) => write!(f, "internal error: {}", e),
            Error::WrongCard => write!(f, "you can't place this card"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::ParseError
    }
}
