use crate::base::{Card, CardType, DeckTrait, GameTrait, StateTrait, UserTrait};
use crate::user::UserId;

pub struct BasicDeck {
    cards: Vec<Card>,
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
        BasicDeck { cards }
    }

    fn get_list(&self) -> &Vec<Card> {
        &self.cards
    }

    fn get_mut_list(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}

pub struct BasicUser {
    id: UserId,
}

impl UserTrait for BasicUser {
    fn get_user_id(&self) -> u64 {
        self.id
    }
}

pub struct BasicState {}

impl StateTrait for BasicState {
    fn current_player(&self) -> Option<UserId> {
        unimplemented!()
    }

    fn next(&mut self) {
        unimplemented!()
    }
}

pub struct BasicGame<D, U, S>
where
    D: DeckTrait,
    U: UserTrait,
    S: StateTrait,
{
    deck: D,
    users: Vec<U>,
    state: S,
}

impl<D, U, S> GameTrait for BasicGame<D, U, S>
where
    D: DeckTrait,
    U: UserTrait,
    S: StateTrait,
{
    type Deck = D;
    type User = U;
    type State = S;

    fn is_joker_called(&self) -> bool {
        unimplemented!()
    }

    fn get_current_pattern(&self) -> CardType {
        unimplemented!()
    }

    fn get_giruda(&self) -> Option<CardType> {
        unimplemented!()
    }
}
