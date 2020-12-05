use crate::command::Command;
use crate::error::Result;
use crate::rule::Rule;
use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Game {
    rule_name: String,
    rule: Rule,
    state: State,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        unimplemented!()
    }

    pub fn change_rule(&mut self, rule_name: String, rule: Rule) {
        self.rule_name = rule_name;
        self.rule = rule;
    }

    pub fn valid_users(&self) -> u8 {
        self.state.valid_users(&self.rule)
    }

    pub fn next(&mut self, user_id: usize, cmd: Command) -> Result<bool> {
        self.state = self.state.next(user_id, cmd, &self.rule)?;
        Ok(self.state.is_finished())
    }
}
