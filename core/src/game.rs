use crate::card::{Card, CardType};
use crate::friend::Friend::UnDecided;
use crate::user::{User, UserId};
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Friend {
    Card(Vec<Card>),
    Person(UserId),
    // TODO: Add other conditions
}

pub enum GameState {
    Election,
    Setup,
    Game {
        round_count: u8,
        round_progress: u8,
        start_player: UserId,
        cards: [Card; 5],
    },
    Finished {
        winners: Vec<UserId>,
    },
}

pub struct GameUser {
    id: UserId,
    in_hand: Vec<Card>,
    gained: Vec<Card>,
    score: i32,
}

impl GameUser {
    pub fn new(id: UserId) -> GameUser {
        GameUser {
            id,
            in_hand: Vec::new(),
            gained: Vec::new(),
            score: 0,
        }
    }
}

pub struct Game {
    user_list: [UserId; 5],
    users: HashMap<UserId, GameUser>,
    state: GameState,
    friend: Option<UserId>,
    friend_cond: Friend,
    president: UserId,
    giruda: CardType,
    mighty: Card,
}

impl Game {
    pub fn new() -> Game {
        Game {
            user_list: [0; 5],
            users: HashMap::new(),
            state: GameState::Election,
            friend: None,
            friend_cond: Friend::Person(0),
            president: 0,
            giruda: CardType::Spade,
            mighty: Card::Unknown,
        }
    }

    pub fn delete_user(&mut self, user_id: UserId) {
        for i in self.user_list.iter_mut() {
            if *i == user_id {
                *i = 0;
            }
        }
        self.users.remove(&user_id);
    }

    pub fn place_card(&mut self, user: &User, card: &Card) {
        // TODO
    }

    pub fn set_giruda(&mut self, giruda: CardType) {
        self.giruda = giruda;
        self.mighty = if self.giruda == CardType::Spade {
            Card::Card(CardType::Diamond, 1)
        } else {
            Card::Card(CardType::Spade, 1)
        }
    }

    pub fn get_mighty(&self) -> &Card {
        &self.mighty
    }
}
