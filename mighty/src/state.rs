use crate::card::{Card, Pattern};
use crate::command::Command;
use crate::error::{Result, Error};
use crate::rule::{Rule, election};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum State {
    NotStarted,
    Election {
        // Option<Pattern> for no giruda.
        // Outer option for not going out.
        pledge: Vec<Option<(Option<Pattern>, u8)>>,
        done: Vec<bool>,
        deck: Vec<Vec<Card>>,
        // leftover cards
        left: Vec<Card>,
        // current user
        curr_user: usize,
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
    fn get_random_deck(rule : &Rule) -> Vec<Vec<Card>> {
        loop {
            let mut deck = rule.deck.clone();
            deck.shuffle(&mut rand::thread_rng());
            let deck = deck.chunks(rule.card_cnt_per_user as usize).map(|v| v.to_vec()).collect::<Vec<_>>();

            let is_not_missed_deal = deck
                .iter()
                .map(|v| {
                    if v.len() ==  rule.card_cnt_per_user as usize{
                        !rule.missed_deal.is_missed_deal(&v)
                    } else {
                        false
                    }
                })
                .all(|s| s );

            if is_not_missed_deal {
                break deck;
            }
        }
    }
}

impl State {
    pub fn new() -> State {
        unimplemented!()
    }

    pub fn next(&self, user_id: usize, cmd: Command, rule: &Rule) -> Result<Self> {
        match self {
            State::NotStarted => match cmd {
                Command::StartGame => {
                    if user_id != 0 {
                        return Err(Error::NotLeader);
                    }

                    let mut deck = State::get_random_deck(rule);
                    let left = deck.pop().unwrap();
                    let mut curr_user = 0;
                    if rule.election.contains(election::Election::PASS_FIRST) {
                        curr_user = 1;
                    }

                    Ok(State::Election {
                        pledge: vec![None; 5],
                        done: vec![false; 5],
                        deck,
                        left,
                        curr_user,
                    })
                }
                _ => Err(Error::InvalidCommand("BasicCommand::StartGame")),
            }
            
            State::Election {
                pledge,
                done,
                deck,
                left,
                curr_user,
            } => match cmd {
                Command::Pledge(x) => {
                    let mut done = done.clone();
                    let mut pledge = pledge.clone();

                    if *curr_user != user_id && rule.election.contains(election::Election::ORDERED) {
                        return Err(Error::InvalidUser(*curr_user));
                    } 

                    match x {
                        Some((c, p)) => {
                            if p > rule.pledge.max {
                                return Err(Error::InvalidPledge(true, rule.pledge.max));
                            }
                            if matches!(c, None) && !rule.election.contains(election::Election::NO_GIRUDA_EXIST) {
                                return Err(Error::InvalidPledge(true, 0));
                            }
                            if done[user_id] {
                                return Err(Error::InvalidPledge(true, 0));
                            }

                            done[user_id] = false;
                            let max_pledge = pledge.iter().map(|j| 
                                match *j  {
                                    Some((_, p)) => { p },
                                    _ => 0,
                                }
                            ).max().unwrap();
                            let max_pledge = std::cmp::max(max_pledge, rule.pledge.min);
                            let offset = if matches!(c, None) { rule.pledge.no_giruda_offset } else { 0 };
                            let max_pledge = (max_pledge as i8 + offset) as u8;
                            if p < max_pledge {
                                return Err(Error::InvalidPledge(false, max_pledge));
                            }
                            if p == max_pledge && rule.election.contains(election::Election::INCREASING) {
                                return Err(Error::InvalidPledge(false, max_pledge));
                            }
    
                            pledge[user_id] = Some((c, p));
    
                            Ok(State::Election {
                                pledge,
                                done,
                                deck: deck.clone(),
                                left: left.clone(),
                                curr_user: (user_id + 1) % (rule.user_cnt as usize)
                            })
                        }
                        _ => {
                            done[user_id] = true;
                            // not done
                            Ok(State::Election {
                                pledge,
                                done,
                                deck: deck.clone(),
                                left: left.clone(),
                                curr_user: (user_id + 1) % (rule.user_cnt as usize)
                            })
                            
                        }
                    } 
                }
                Command::Random => self.next(user_id, Command::Pledge(None), rule),
                _ => Err(Error::InvalidCommand("BasicCommand::Pledge")),
            },
            _ => {
                Ok(self.clone())
            }
        }
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
