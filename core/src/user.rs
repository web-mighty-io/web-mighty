pub enum CardType {
    Heart,
    Diamond,
    Spade,
    Clover,
}

pub enum Card {
    Jocker(bool),
    Card(CardType, u8),
    Unknown,
    None,
}

pub struct User {
    pub uid: u64,
    pub name: String,
}
