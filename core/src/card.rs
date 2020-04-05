use crate::game::Game;

#[derive(PartialEq)]
pub enum CardType {
    Spade,
    Diamond,
    Heart,
    Clover,
}

#[derive(PartialEq, Clone)]
pub enum Card {
    Joker(bool),
    Card(CardType, u8),
    Unknown,
}

impl Card {
    pub fn is_score(&self) -> bool {
        match self {
            Card::Joker(_) => false,
            Card::Card(_, t) => *t >= 10,
            Card::Unknown => false,
        }
    }

    pub fn less(&self, other: &Card, game: &Game) -> bool {
        let mighty = game.get_mighty();
        if self == mighty {
            return false;
        }

        if other == mighty {
            return true;
        }

        // TODO

        true
    }
}
