use crate::base::*;
use crate::user::UserId;
use rand::seq::SliceRandom;
use rand::Rng;
use std::str::FromStr;

/// State of basic mighty game.
///
/// - `NotStarted`: When game is not started
/// - `Election`: After passing out cards,
/// - `SelectFriend`: After election, president will select friend (or not)
/// - `InGame`: After selecting friend, they will play 10 turns
#[derive(Clone, Debug)]
pub enum BasicState {
    NotStarted,
    Election {
        // Option for no giruda
        // giruda and count of pledge
        pledge: Vec<(Option<CardType>, u8)>,
        // if each player is done
        done: Vec<bool>,
        // deck for each user (len of 5)
        deck: Vec<Vec<Card>>,
        left: Vec<Card>,
    },
    SelectFriend {
        // president in in-game user id
        president: usize,
        // pledge for president
        pledge: (Option<CardType>, u8),
        // deck for each user (len of 5)
        deck: Vec<Vec<Card>>,
    },
    InGame {
        // president in in-game user id
        president: usize,
        // friend func executed every task when friend is not determined
        // result is for person 0 to 4 (in-game user id)
        friend_func: FriendFunc,
        // 0 to 4 for in-game user id
        friend: Option<usize>,
        // giruda of this game
        giruda: Option<CardType>,
        // pledge score of ruling party
        pledge: u8,
        // score for ruling party
        score: u8,
        // deck for each user (len of 5)
        deck: Vec<Vec<Card>>,
        // score cards
        score_deck: Vec<Vec<Card>>,
        // turn count 0 to 9
        turn_count: u8,
        // placed cards in front of users
        placed_cards: Vec<Option<Card>>,
        // start user of this turn
        start_user: usize,
        // current user of this turn
        current_user: usize,
        // current pattern of this turn
        current_pattern: RushType,
        // is joker called (user can decide)
        is_joker_called: bool,
    },
    GameEnded {
        // bitmask of winners
        // ex) if 0 and 3 win: 0b01001
        winner: u8,
        // below are game info
        president: usize,
        friend: usize,
        score: u8,
        giruda: Option<CardType>,
    },
}

/// Game structure for basic mighty game.
///
/// - `users`: User List
/// - `state`: Game state
#[derive(Clone, Debug)]
pub struct BasicGame {
    users: Vec<UserId>,
    state: BasicState,
}

impl BasicGame {
    pub fn new() -> BasicGame {
        BasicGame {
            users: vec![0; 5],
            state: BasicState::NotStarted,
        }
    }

    /// Check if joker called.
    /// **Valid output only in in-game.**
    fn is_joker_called(&self) -> bool {
        match self.state {
            BasicState::InGame {
                is_joker_called, ..
            } => is_joker_called,
            _ => false,
        }
    }

    /// Get the current pattern of this turn.
    /// **Valid output only in in-game.**
    fn get_current_pattern(&self) -> RushType {
        match &self.state {
            BasicState::InGame {
                current_pattern, ..
            } => current_pattern.clone(),
            // don't need this value
            _ => RushType::Spade,
        }
    }

    /// Get the giruda of this turn.
    /// **Valid output only in in-game.**
    fn get_giruda(&self) -> Option<CardType> {
        match &self.state {
            BasicState::InGame { giruda, .. } => giruda.clone(),
            // don't need this value
            _ => None,
        }
    }

    /// Get the mighty card in game
    /// **Valid output only in in-game.**
    fn get_mighty(&self) -> Option<Card> {
        match &self.state {
            BasicState::InGame { giruda, .. } => match giruda {
                Some(CardType::Spade) => Some(Card::Normal(CardType::Diamond, 0)),
                _ => Some(Card::Normal(CardType::Spade, 0)),
            },
            // don't need this value
            _ => None,
        }
    }

    // true if lhs < rhs
    // undefined when lhs == rhs
    /// todo: make tests
    pub fn compare_cards(&self, lhs: &Card, rhs: &Card) -> bool {
        if let Some(mighty) = self.get_mighty() {
            if *lhs == mighty {
                return false;
            }
            if *rhs == mighty {
                return true;
            }
        }

        let cur_pat = self.get_current_pattern();
        let cur_color = ColorType::from(cur_pat.clone());
        let giruda = self.get_giruda();
        let giruda_color = giruda.clone().map(|c| ColorType::from(c));

        match lhs {
            Card::Normal(c1, n1) => {
                match rhs {
                    Card::Normal(c2, n2) => {
                        if let Some(giruda) = giruda {
                            if *c1 == giruda && *c2 == giruda {
                                return n1 > n2;
                            } else if *c1 == giruda || *c2 == giruda {
                                return *c2 == giruda;
                            }
                        }

                        if cur_pat.contains(c1) && cur_pat.contains(c2) {
                            n1 > n2
                        } else if cur_pat.contains(c1) || cur_pat.contains(c2) {
                            cur_pat.contains(c2)
                        } else {
                            // actually this is meaningless
                            n1 > n2
                        }
                    }

                    Card::Joker(c2) => {
                        if *c2 != cur_color {
                            false
                        } else if self.is_joker_called() {
                            false
                        } else if let Some(giruda) = giruda {
                            if *c1 == giruda {
                                *c2 == giruda_color.unwrap()
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    }
                }
            }

            Card::Joker(c1) => match rhs {
                Card::Normal(c2, _) => {
                    if *c1 != cur_color {
                        true
                    } else if self.is_joker_called() {
                        true
                    } else if let Some(giruda) = giruda {
                        if *c2 == giruda {
                            *c1 == giruda_color.unwrap()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }

                // no need to check if joker is called
                Card::Joker(c2) => *c2 == cur_color,
            },
        }
    }
}

impl GameTrait for BasicGame {
    type State = BasicState;

    fn get_users(&self) -> &Vec<UserId> {
        &self.users
    }

    fn get_users_mut(&mut self) -> &mut Vec<UserId> {
        &mut self.users
    }

    /// Process the given arguments and change the game state.
    /// First argument has to be the *in-game user id*
    /// who sent this command. **(always in bounds)**
    /// Second argument has to be the state of the game
    /// for checking command
    /// Third and after is different for each state.
    fn process(&self, args: Vec<String>) -> Result<BasicState, GameError> {
        match &self.state {
            // command is 'n'
            BasicState::NotStarted => {
                if args.len() != 2 {
                    return Err(GameError::CommandError(format!(
                        "command length should be 2, actual: {}",
                        args.len()
                    )));
                }

                if args[1] != "n" {
                    return Err(GameError::CommandError(format!(
                        "game state is not same. expected: 'n', actual: {}",
                        args[1]
                    )));
                }

                // todo: handle miss deal
                let mut deck = Card::new_deck()
                    .chunks(10)
                    .map(|v| v.to_vec())
                    .collect::<Vec<_>>();

                let left = deck.pop().ok_or(GameError::InternalError(format!(
                    "deck is not successfully created"
                )))?;

                Ok(BasicState::Election {
                    pledge: vec![(None, 0); 5],
                    done: vec![false; 5],
                    deck,
                    left,
                })
            }

            // command is 'e'
            BasicState::Election {
                pledge,
                done,
                deck,
                left,
            } => {
                if args.len() != 4 {
                    return Err(GameError::CommandError(format!(
                        "command length should be 4, actual: {}",
                        args.len()
                    )));
                }

                if args[1] != "e" {
                    return Err(GameError::CommandError(format!(
                        "game state is not same. expected: 'e', actual: {}",
                        args[1]
                    )));
                }

                if args[2].len() != 1 {
                    return Err(GameError::CommandError(format!(
                        "third argument length should be 1, actual: {}",
                        args[2].len()
                    )));
                }

                // 's': spade
                // 'd': diamond
                // 'h': heart
                // 'c': clover
                // 'n': none (no giruda)
                // 'x': done selecting
                if "sdbcnx".contains(&args[2]) {
                    return Err(GameError::CommandError(format!(
                        "thrid agrument should be one of 's', 'd', 'b', 'c', 'n', 'x', actual: {}",
                        args[2]
                    )));
                }

                let i = args[0].parse::<usize>().unwrap();
                let mut done = done.clone();

                if args[2] == "x" {
                    done[i] = true;

                    if done.iter().fold(true, |a, &b| a && b) {
                        let mut candidate = Vec::new();

                        let mut last_max = 0;
                        for i in 0..5 {
                            let (_, c) = pledge[i];
                            if c > last_max {
                                candidate = vec![i];
                                last_max = c;
                            } else if c == last_max {
                                candidate.push(i);
                            }
                        }

                        if last_max == 0 {
                            // todo: when nobody came out
                            Ok(self.state.clone())
                        } else {
                            let president = *candidate
                                .choose(&mut rand::thread_rng())
                                .unwrap_or(&rand::thread_rng().gen_range(0, 5));
                            let mut deck = deck.clone();

                            deck[president].append(&mut left.clone());

                            Ok(BasicState::SelectFriend {
                                president,
                                pledge: pledge[president].clone(),
                                deck,
                            })
                        }
                    } else {
                        Ok(BasicState::Election {
                            pledge: pledge.clone(),
                            done,
                            deck: deck.clone(),
                            left: left.clone(),
                        })
                    }
                } else {
                    let c = args[3].parse::<u8>().unwrap_or(21);
                    if c > 20 {
                        return Err(GameError::CommandError(format!(
                            "maximum pledge should be 20, actual: {}",
                            c
                        )));
                    }

                    done[i] = false;
                    let mut pledge = pledge.clone();

                    // todo: handle maximum of before
                    if args[2] == "n" {
                        if c < 12 {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than 12 in no giruda mode, actual: {}",
                                c
                            )));
                        }
                        pledge[i] = (None, c);
                    } else {
                        if c < 13 {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than 12 in no giruda mode, actual: {}",
                                c
                            )));
                        }
                        pledge[i] = (CardType::from_str(&args[2]).ok(), c);
                    }

                    Ok(BasicState::Election {
                        pledge,
                        done,
                        deck: deck.clone(),
                        left: left.clone(),
                    })
                }
            }

            // command is 'f'
            // third argument:
            // 0: no friend (no extra argument)
            // 1: user that have special card (1 extra argument)
            // 2: picked user (1 extra argument)
            // over 3: conditional friend
            // 3: n-th turn winner
            BasicState::SelectFriend {
                president,
                pledge,
                deck,
            } => {
                if args.len() < 3 {
                    return Err(GameError::CommandError(format!(
                        "command length should be greater or equal than 3, actual: {}",
                        args.len()
                    )));
                }

                if args[1] != "f" {
                    return Err(GameError::CommandError(format!(
                        "game state is not same. expected: 'f', actual: {}",
                        args[1]
                    )));
                }

                let i = args[0].parse::<usize>().unwrap();

                if i != *president {
                    return Err(GameError::CommandError(format!(
                        "you are not the president of this game"
                    )));
                }

                let fn_type = args[2].parse::<usize>().map_err(|_| {
                    GameError::CommandError(format!(
                        "unrecognized function type of friend, expected: 0~3, actual: {}",
                        args[2]
                    ))
                })?;

                let friend_func = match fn_type {
                    1 => {
                        if args.len() != 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be 4, actual: {}",
                                args.len()
                            )));
                        }

                        let card = args[3].parse::<Card>().map_err(|_| {
                            GameError::CommandError(format!(
                                "failed to parse card, actual: {}",
                                args[3]
                            ))
                        })?;

                        FriendFunc::ByCard(card)
                    }
                    2 => {
                        if args.len() != 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be 4, actual: {}",
                                args.len()
                            )));
                        }

                        FriendFunc::ByUser(args[3].parse::<usize>().unwrap_or(*president))
                    }
                    3 => {
                        if args.len() != 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be 4, actual: {}",
                                args.len()
                            )));
                        }

                        FriendFunc::ByWinning(args[3].parse::<u8>().unwrap_or(0))
                    }
                    _ => FriendFunc::None,
                };

                let (_, pledge) = pledge;

                Ok(BasicState::InGame {
                    president: *president,
                    friend_func,
                    friend: None,
                    giruda: None,
                    pledge: *pledge,
                    score: 0,
                    deck: deck.clone(),
                    score_deck: Vec::new(),
                    turn_count: 0,
                    placed_cards: vec![None; 5],
                    start_user: *president,
                    current_user: *president,
                    current_pattern: RushType::Spade,
                    is_joker_called: false,
                })
            }

            // command is 'g'
            BasicState::InGame { .. } => {
                // todo
                Ok(self.state.clone())
            }

            // command is 'd'
            BasicState::GameEnded { .. } => {
                // todo
                Ok(self.state.clone())
            }
        }
    }
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn compare_cards_test() {
        fn make_game(giruda: &str, current_pattern: &str, is_joker_called: bool) -> BasicGame {
            BasicGame {
                users: vec![],
                state: BasicState::InGame {
                    president: 0,
                    friend_func: FriendFunc::None,
                    friend: Option::None,
                    giruda: CardType::from_str(giruda).ok(),
                    pledge: 0,
                    score: 0,
                    deck: vec![],
                    score_deck: vec![],
                    turn_count: 0,
                    placed_cards: vec![],
                    start_user: 0,
                    current_user: 0,
                    current_pattern: RushType::from_str(current_pattern).unwrap(),
                    is_joker_called,
                },
            }
        }

        fn compare_cards(game: &BasicGame, c1: &str, c2: &str) -> bool {
            game.compare_cards(&Card::from_str(c1).unwrap(), &Card::from_str(c2).unwrap())
        }

        let g = make_game("s", "s", false);
        assert_eq!(compare_cards(&g, "s1", "s0"), true);
        assert_eq!(compare_cards(&g, "s0", "d0"), true);
        assert_eq!(compare_cards(&g, "d0", "s0"), false);
        assert_eq!(compare_cards(&g, "d1", "s0"), true);

        let g = make_game("s", "d", false);
        assert_eq!(compare_cards(&g, "h1", "h0"), true);
        assert_eq!(compare_cards(&g, "h1", "d0"), true);
        assert_eq!(compare_cards(&g, "d1", "s0"), true);
        assert_eq!(compare_cards(&g, "d1", "jb"), false);
        assert_eq!(compare_cards(&g, "d1", "jr"), true);
    }
}
