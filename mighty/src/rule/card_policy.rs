use crate::card::Card;
use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum CardPolicy {
    Valid,
    NoEffect,
    Invalid,
    InvalidForFirst,
}

/// Card Policies
///
/// All types of cards has two policies: First turn & Last turn
#[derive(Debug, Clone, Config, Serialize, Deserialize, Eq)]
pub struct Policy {
    pub mighty: (CardPolicy, CardPolicy),
    pub giruda: (CardPolicy, CardPolicy),
    pub joker: (CardPolicy, CardPolicy),
    pub joker_call: (CardPolicy, CardPolicy),
    pub card: HashMap<Card, (CardPolicy, CardPolicy)>,
}

impl PartialEq for Policy {
    fn eq(&self, other: &Self) -> bool {
        self.mighty == other.mighty
            && self.giruda == other.giruda
            && self.joker == other.joker
            && self.joker_call == other.joker_call
            && self.card == other.card
    }
}

impl Hash for Policy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mighty.hash(state);
        self.giruda.hash(state);
        self.joker.hash(state);
        self.joker_call.hash(state);

        for (k, v) in self.card.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy {
    pub fn new() -> Policy {
        Policy {
            mighty: (CardPolicy::Valid, CardPolicy::Valid),
            giruda: (CardPolicy::Invalid, CardPolicy::Valid),
            joker: (CardPolicy::NoEffect, CardPolicy::NoEffect),
            joker_call: (CardPolicy::Valid, CardPolicy::Valid),
            card: HashMap::new(),
        }
    }
}
