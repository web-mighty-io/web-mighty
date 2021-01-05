#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    ParseError,
    InvalidCommand(&'static str),
    InvalidPledge(bool, u8),
    InvalidUser,
    InvalidFriendFunc,
    NotLeader,
    NotPresident,
    NotInDeck,
    SameGiruda,
    WrongPattern,
    Internal(&'static str),
    WrongCard,
    PassFirst,
    JokerCall,
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
            Error::InvalidFriendFunc => write!(f, "invalid friend function"),
            Error::NotLeader => write!(f, "you are not the leader"),
            Error::NotPresident => write!(f, "you are not the president"),
            Error::NotInDeck => write!(f, "the card is not in the deck"),
            Error::WrongPattern => write!(f, "your card has wrong pattern"),
            Error::SameGiruda => write!(f, "same giruda"),
            Error::Internal(e) => write!(f, "internal error: {}", e),
            Error::WrongCard => write!(f, "you can't place this card"),
            Error::PassFirst => write!(f, "dealer should run at first turn"),
            Error::JokerCall => write!(f, "you need to place the joker"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::ParseError
    }
}
