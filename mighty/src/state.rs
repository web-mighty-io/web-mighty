use crate::card::{Card, Pattern};
use crate::command::Command;
use crate::error::Result;
use crate::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum State {
    Election {
        // Option<Pattern> for no giruda.
        // Outer option for not going out.
        pledge: Vec<Option<(Option<Pattern>, u8)>>,
        done: Vec<bool>,
        deck: Vec<Vec<Card>>,
        // leftover cards
        left: Vec<Card>,
    },
    SelectFriend {
        president: usize,
        giruda: Option<Pattern>,
        pledge: u8,
        deck: Vec<Vec<Card>>,
    },
    InGame {
        president: usize,
    },
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> State {
        unimplemented!()
    }

    pub fn next(&self, user_id: usize, cmd: Command, rule: &Rule) -> Result<Self> {
        unimplemented!()
    }

    /// Valid users to action next time.
    /// Result is 8-bit integer which contains 0 or 1 for each user.
    /// If all users all valid to action, the result would be `(1 << N) - 1`
    pub fn valid_users(&self, rule: &Rule) -> u8 {
        unimplemented!()
    }

    pub fn is_finished(&self) -> bool {
        unimplemented!()
    }
}
