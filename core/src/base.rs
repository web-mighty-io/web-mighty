use crate::user::UserId;
use rand::seq::SliceRandom;

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
    Other(String),
}

pub trait DeckTrait {
    fn new() -> Self;

    fn get_list(&self) -> &Vec<Card>;

    fn get_mut_list(&mut self) -> &mut Vec<Card>;

    fn shuffle(&mut self) {
        let v = self.get_mut_list();
        v.shuffle(&mut rand::thread_rng());
    }

    fn next(&mut self) -> Option<Card> {
        let v = self.get_mut_list();
        v.pop()
    }
}

pub trait UserTrait {
    fn get_user_id(&self) -> UserId;
}

pub trait StateTrait {
    fn current_player(&self) -> Option<UserId>;

    fn next(&mut self);
}

pub trait GameTrait {
    type Deck: DeckTrait;
    type User: UserTrait;
    type State: StateTrait;

    fn is_score_card(card: &Card) -> bool {
        match card {
            Card::Normal(_, t) => *t >= 10,
            Card::Joker(_) => false,
            Card::Other(_) => false,
        }
    }

    fn compare_cards(&self, lhs: Card, rhs: Card) -> bool {
        if let Some(mighty) = self.get_mighty() {
            if lhs == mighty {
                return false;
            }
            if rhs == mighty {
                return true;
            }
        }

        if let Card::Joker(_) = lhs {
            return self.is_joker_called();
        }
        if let Card::Joker(_) = rhs {
            return !self.is_joker_called();
        }

        if let Card::Normal(left_pat, left_num) = lhs {
            if let Card::Normal(right_pat, right_num) = rhs {
                let a = CardType::contains(&left_pat, &self.get_current_pattern());
                let b = CardType::contains(&right_pat, &self.get_current_pattern());

                if (a && !b) || (!a && b) {
                    return a;
                }

                return left_num < right_num;
            }
        }

        unreachable!()
    }

    fn is_joker_called(&self) -> bool;

    fn get_current_pattern(&self) -> CardType;

    fn get_giruda(&self) -> Option<CardType>;

    fn get_mighty(&self) -> Option<Card> {
        match self.get_giruda() {
            Some(CardType::Spade) => Some(Card::Normal(CardType::Heart, 1)),
            Some(_) => Some(Card::Normal(CardType::Spade, 1)),
            None => None,
        }
    }
}
