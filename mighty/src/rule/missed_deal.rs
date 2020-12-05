use crate::card::Card;
use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Serialize, Deserialize, Config, Eq)]
pub struct MissedDeal {
    pub score: i8,
    pub joker: i8,
    pub card: HashMap<Card, i8>,
    pub limit: i8,
}

impl PartialEq for MissedDeal {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.joker == other.joker && self.card == other.card && self.limit == other.limit
    }
}

impl Hash for MissedDeal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.score.hash(state);
        self.joker.hash(state);
        self.limit.hash(state);

        for (k, v) in self.card.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Default for MissedDeal {
    fn default() -> Self {
        Self::new()
    }
}

impl MissedDeal {
    pub fn new() -> MissedDeal {
        MissedDeal {
            score: 1,
            joker: 0,
            card: HashMap::new(),
            limit: 0,
        }
    }

    pub fn is_missed_deal(&self, deck: &[Card]) -> bool {
        let mut s = 0i8;

        for i in deck.iter() {
            if let Some(a) = self.card.get(i) {
                s += *a;
            } else if i.is_joker() {
                s += self.joker;
            } else if i.is_score() {
                s += self.score;
            }
        }

        s <= self.limit
    }
}
