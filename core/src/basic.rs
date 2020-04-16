use crate::base::{Card, CardType, DeckTrait, GameState, GameTrait, UserTrait};
use crate::user::UserId;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BasicDeck {
    cards: Vec<Card>,
    index: usize,
}

impl DeckTrait for BasicDeck {
    fn new() -> Self {
        let mut cards = Vec::with_capacity(54);
        for i in 1..=13 {
            cards.push(Card::Normal(CardType::Spade, i));
        }
        for i in 1..=13 {
            cards.push(Card::Normal(CardType::Diamond, i));
        }
        for i in 1..=13 {
            cards.push(Card::Normal(CardType::Heart, i));
        }
        for i in 1..=13 {
            cards.push(Card::Normal(CardType::Clover, i));
        }
        cards.push(Card::Joker(CardType::Red));
        BasicDeck { cards, index: 0 }
    }

    fn get_list(&self) -> &Vec<Card> {
        &self.cards
    }

    fn get_list_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}

pub struct BasicUser {
    id: UserId,
    deck: Vec<Card>,
    front: Card,
    score: Vec<Card>,
}

impl UserTrait for BasicUser {
    fn get_user_id(&self) -> u64 {
        self.id
    }

    fn get_deck(&self) -> &Vec<Card> {
        &self.deck
    }

    fn get_deck_mut(&mut self) -> &mut Vec<Card> {
        &mut self.deck
    }

    fn get_front_card(&self) -> &Card {
        unimplemented!()
    }

    fn get_front_card_mut(&mut self) -> &mut Card {
        unimplemented!()
    }
}

pub struct BasicGame<D, U>
where
    D: DeckTrait,
    U: UserTrait,
{
    deck: D,
    users: Vec<U>,
    state: GameState,
    giruda: Option<CardType>,
}

impl<D, U> GameTrait for BasicGame<D, U>
where
    D: DeckTrait,
    U: UserTrait,
{
    type Deck = D;
    type User = U;

    fn is_joker_called(&self) -> bool {
        unimplemented!()
    }

    fn get_leading_type(&self) -> Option<&CardType> {
        unimplemented!()
    }

    fn get_giruda(&self) -> Option<CardType> {
        self.giruda.clone()
    }

    fn get_state(&self) -> &GameState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    fn get_user_id(&self) -> &Vec<u64> {
        unimplemented!()
    }

    fn get_users(&self) -> &HashMap<u64, Self::User, RandomState> {
        unimplemented!()
    }
}
