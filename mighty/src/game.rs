use crate::command::Command;
use crate::error::{Result, Error};
use crate::rule::Rule;
use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Game {
    rule: Rule,
    state: State,
    valid_users: u8,
}

impl Game {
    pub fn new(rule: Rule) -> Game {
        let state = State::new(&rule);
        let valid_users = state.valid_users(&rule);
        Game {
            rule,
            state,
            valid_users,
        }
    }

    pub fn valid_users(&self) -> u8 {
        self.valid_users
    }

    pub fn is_finished(&self) -> bool {
        self.valid_users == 0
    }

    pub fn next(&mut self, user_id: usize, cmd: Command) -> Result<bool> {
        if self.valid_users & (1u8 << user_id) > 0 {
            self.state = self.state.next(user_id, cmd, &self.rule)?;
            self.valid_users = self.state.valid_users(&self.rule);
            Ok(self.valid_users == 0)
        } else {
            Err(Error::InvalidUser)
        }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }
}
