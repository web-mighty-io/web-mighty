use crate::user::UserId;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum CardType {
    Spade,
    Diamond,
    Heart,
    Clover,
    Black,
    Red,
}

impl CardType {
    pub fn contains(lhs: &CardType, rhs: &CardType) -> bool {
        if *lhs == *rhs {
            true
        } else {
            (*lhs == CardType::Red && (*rhs == CardType::Diamond || *rhs == CardType::Heart))
                || (*lhs == CardType::Black
                    && (*rhs == CardType::Spade || *rhs == CardType::Clover))
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Card {
    Normal(CardType, u8),
    Joker(CardType),
}

impl Card {
    fn is_score_card(&self) -> bool {
        match self {
            Card::Normal(_, t) => *t >= 10,
            Card::Joker(_) => false,
        }
    }

    fn get_type(&self) -> &CardType {
        match self {
            Card::Normal(t, _) => t,
            Card::Joker(t) => t,
        }
    }
}

pub trait DeckTrait {
    fn new() -> Self;

    fn get_list(&self) -> &Vec<Card>;

    fn get_list_mut(&mut self) -> &mut Vec<Card>;

    fn len(&self) -> usize {
        self.get_list().len()
    }

    fn shuffle(&mut self) {
        let v = self.get_list_mut();
        v.shuffle(&mut rand::thread_rng());
    }

    fn next(&mut self) -> Option<Card> {
        self.get_list_mut().pop()
    }
}

pub trait UserTrait {
    fn get_user_id(&self) -> UserId;

    fn get_deck(&self) -> &Vec<Card>;

    fn get_deck_mut(&mut self) -> &mut Vec<Card>;

    fn get_front_card(&self) -> &Card;

    fn get_front_card_mut(&mut self) -> &mut Card;

    fn put_card(&mut self, card: &Card) {
        let pos = self.get_deck().iter().position(|x| *x == *card).unwrap();
        self.get_deck_mut().remove(pos);
        *self.get_front_card_mut() = card.clone();
    }
}

pub enum GameState {
    Election,
    Game {
        step: u8,
        start: UserId,
        current: UserId,
    },
    Finish,
}

pub trait GameTrait {
    type Deck: DeckTrait;
    type User: UserTrait;

    fn compare_cards(&self, lhs: &Card, rhs: &Card) -> bool {
        let mighty = self.get_mighty();
        if *lhs == mighty {
            return false;
        }
        if *rhs == mighty {
            return true;
        }

        if let Card::Joker(_) = lhs {
            return self.is_joker_called();
        }
        if let Card::Joker(_) = rhs {
            return !self.is_joker_called();
        }

        if let Card::Normal(left_pat, left_num) = lhs {
            if let Card::Normal(right_pat, right_num) = rhs {
                let a = CardType::contains(left_pat, self.get_leading_type().unwrap());
                let b = CardType::contains(right_pat, self.get_leading_type().unwrap());

                if (a && !b) || (!a && b) {
                    return a;
                }

                return *left_num < *right_num;
            }
        }

        unreachable!()
    }

    fn is_joker_called(&self) -> bool;

    fn get_leading_type(&self) -> Option<&CardType> {
        match self.get_state() {
            GameState::Game {
                step: _,
                start,
                current: _,
            } => Some(
                self.get_users()
                    .get(start)
                    .unwrap()
                    .get_front_card()
                    .get_type(),
            ),
            _ => None,
        }
    }

    fn get_giruda(&self) -> Option<CardType>;

    fn get_mighty(&self) -> Card {
        match self.get_giruda() {
            Some(CardType::Spade) => Card::Normal(CardType::Heart, 1),
            _ => Card::Normal(CardType::Spade, 1),
        }
    }

    fn get_state(&self) -> &GameState;

    fn get_state_mut(&mut self) -> &mut GameState;

    fn get_user_id(&self) -> &Vec<UserId>;

    fn get_users(&self) -> &HashMap<UserId, Self::User>;
}
