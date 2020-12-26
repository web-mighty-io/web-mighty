use crate::card::{Card, Pattern};
use crate::command::Command;
use crate::error::{Error, Result};
use crate::rule::{election, Rule};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum State {
    Election {
        // Option<Pattern> for no giruda.
        // Outer option for not going out.
        pledge: Vec<Option<(Option<Pattern>, u8)>>,
        done: Vec<bool>,
        // current user
        curr_user: usize,
        // start user
        start_user: Option<usize>,
        deck: Vec<Vec<Card>>,
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
        Self::new(&Rule::new())
    }
}

impl State {
    fn get_random_deck(rule: &Rule) -> Vec<Vec<Card>> {
        loop {
            let mut deck = rule.deck.clone();
            deck.shuffle(&mut rand::thread_rng());
            let deck = deck
                .chunks(rule.card_cnt_per_user as usize)
                .map(|v| v.to_vec())
                .collect::<Vec<_>>();

            let is_not_missed_deal = deck
                .iter()
                .map(|v| {
                    if v.len() == rule.card_cnt_per_user as usize {
                        !rule.missed_deal.is_missed_deal(&v)
                    } else {
                        false
                    }
                })
                .all(|s| s);

            if is_not_missed_deal {
                break deck;
            }
        }
    }
}

impl State {
    pub fn new(rule: &Rule) -> State {
        let mut deck = State::get_random_deck(rule);
        let left = deck.pop().unwrap();
        State::Election {
            pledge: vec![None; 5],
            done: vec![false; 5],
            curr_user: 0,
            start_user: None,
            deck,
            left,
        }
    }

    pub fn next(&self, user_id: usize, cmd: Command, rule: &Rule) -> Result<Self> {
        match self {
            State::Election {
                pledge,
                done,
                curr_user,
                start_user,
                deck,
                left,
            } => match cmd {
                Command::Pledge(x) => {
                    let mut done = done.clone();
                    let mut pledge = pledge.clone();
                    let is_ordered = rule.election.contains(election::Election::ORDERED);
                    if *curr_user != user_id && is_ordered {
                        return Err(Error::InvalidUser);
                    } //vaild users 함수 만들고 바꾸기

                    match x {
                        Some((c, p)) => {
                            if p > rule.pledge.max {
                                return Err(Error::InvalidPledge(true, rule.pledge.max));
                            }
                            if c == None && !rule.election.contains(election::Election::NO_GIRUDA_EXIST) {
                                return Err(Error::InvalidPledge(true, 0));
                            }
                            if done[user_id] {
                                return Err(Error::InvalidPledge(true, 0));
                            }
                            let start_user = if *start_user == None {
                                user_id
                            } else {
                                start_user.unwrap()
                            };
                            done[user_id] = false;
                            let max_pledge = pledge
                                .iter()
                                .map(|j| match *j {
                                    Some((_, p)) => p,
                                    _ => 0,
                                })
                                .max()
                                .unwrap();
                            let max_pledge = std::cmp::max(max_pledge, rule.pledge.min);
                            let offset = if c == None { rule.pledge.no_giruda_offset } else { 0 };
                            let max_pledge = if start_user == user_id {
                                (max_pledge as i8 + offset + rule.pledge.first_offset) as u8
                            } else {
                                (max_pledge as i8 + offset) as u8
                            };
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
                                curr_user: (user_id + 1) % (rule.user_cnt as usize),
                                start_user: Some(start_user),
                                deck: deck.clone(),
                                left: left.clone(),
                            })
                        }
                        _ => {
                            if !rule.election.contains(election::Election::PASS_FIRST) && *start_user == None {
                                return Err(Error::PassFirst);
                            }
                            done[user_id] = true;
                            let mut candidate = Vec::new();
                            let mut last_max = 0u8;
                            let not_done: Vec<usize> =
                                done.iter().enumerate().filter(|(_, &x)| !x).map(|(i, _)| i).collect();
                            let mut is_election_done = false;
                            if is_ordered && not_done.len() == 1 {
                                is_election_done = true;
                                match pledge[not_done[0]] {
                                    Some((_, c)) => {
                                        last_max = c;
                                        candidate = vec![not_done[0]];
                                    }
                                    _ => {
                                        for i in 0..rule.user_cnt {
                                            candidate.push(i as usize);
                                        }
                                    }
                                }
                            } else if !is_ordered && not_done.len() == 0 {
                                is_election_done = true;
                                for (i, p) in pledge.iter().enumerate() {
                                    match p {
                                        Some((_, c)) => match c.cmp(&last_max) {
                                            std::cmp::Ordering::Greater => {
                                                candidate = vec![i];
                                                last_max = *c;
                                            }
                                            std::cmp::Ordering::Equal => {
                                                candidate.push(i);
                                            }
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                            }
                            if is_election_done {
                                let mut deck = deck.clone();
                                let left = left.clone();
                                let president = candidate.choose(&mut rand::thread_rng()).copied().unwrap();
                                let mut pledge = pledge[president];
                                if last_max == 0 {
                                    let mut pledge_vec = vec![
                                        (Some(Pattern::Spade), rule.pledge.min),
                                        (Some(Pattern::Diamond), rule.pledge.min),
                                        (Some(Pattern::Heart), rule.pledge.min),
                                        (Some(Pattern::Clover), rule.pledge.min),
                                    ];
                                    if rule.election.contains(election::Election::NO_GIRUDA_EXIST) {
                                        pledge_vec
                                            .push((None, (rule.pledge.min as i8 + rule.pledge.no_giruda_offset) as u8));
                                    }
                                    pledge = Some(pledge_vec.choose(&mut rand::thread_rng()).copied().unwrap());
                                }
                                deck[president].append(&mut left.clone());
                                Ok(State::SelectFriend {
                                    president,
                                    giruda: pledge.unwrap().0,
                                    pledge: pledge.unwrap().1,
                                    deck,
                                })
                            } else {
                                Ok(State::Election {
                                    pledge,
                                    done,
                                    curr_user: (user_id + 1) % (rule.user_cnt as usize),
                                    start_user: *start_user,
                                    deck: deck.clone(),
                                    left: left.clone(),
                                })
                            }
                        }
                    }
                }
                Command::Random => self.next(user_id, Command::Pledge(None), rule),
                _ => Err(Error::InvalidCommand("BasicCommand::Pledge")),
            },
            _ => Ok(self.clone()),
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
