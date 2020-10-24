use crate::base::*;
use crate::user::UserId;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::Ordering;
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
        // if friend is known to other people
        is_friend_known: bool,
        // giruda of this game
        giruda: Option<CardType>,
        // pledge score of ruling party
        pledge: u8,
        // deck for each user (len of 5)
        deck: Vec<Vec<Card>>,
        // score cards
        score_deck: Vec<Vec<Card>>,
        // turn count 0 to 9
        turn_count: u8,
        // placed cards in front of users
        placed_cards: Vec<Card>,
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

impl Default for BasicGame {
    fn default() -> Self {
        Self::new()
    }
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
    fn get_mighty(&self) -> Card {
        match &self.state {
            BasicState::InGame { giruda, .. } => match giruda {
                Some(CardType::Spade) => Card::Normal(CardType::Diamond, 0),
                _ => Card::Normal(CardType::Spade, 0),
            },
            // don't need this value
            _ => Card::Normal(CardType::Spade, 0),
        }
    }

    pub fn get_state(&self) -> &str {
        match self.state {
            BasicState::NotStarted => "n",
            BasicState::Election { .. } => "e",
            BasicState::SelectFriend { .. } => "f",
            BasicState::InGame { .. } => "g",
            BasicState::GameEnded { .. } => "d",
        }
    }

    fn get_random_deck() -> Vec<Vec<Card>> {
        loop {
            let mut deck = Card::new_deck();
            deck.shuffle(&mut rand::thread_rng());
            let deck = deck.chunks(10).map(|v| v.to_vec()).collect::<Vec<_>>();

            let is_not_missed_deal = deck
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|c| {
                            if Card::Normal(CardType::Spade, 0).eq(c) {
                                -2
                            } else if c.is_score() {
                                2
                            } else if matches!(c, Card::Joker(..)) {
                                -1
                            } else {
                                0
                            }
                        })
                        .sum::<isize>()
                })
                .all(|s| s > 2);

            if is_not_missed_deal {
                break deck;
            }
        }
    }

    // true if lhs < rhs
    // undefined when lhs == rhs
    pub fn compare_cards(&self, lhs: &Card, rhs: &Card) -> bool {
        let mighty = self.get_mighty();
        if *lhs == mighty {
            return false;
        }
        if *rhs == mighty {
            return true;
        }

        let cur_pat = self.get_current_pattern();
        let cur_color = ColorType::from(cur_pat.clone());
        let giruda = self.get_giruda();
        let giruda_color = giruda.clone().map(ColorType::from);

        match lhs {
            Card::Normal(c1, n1) => match rhs {
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
                    if *c2 != cur_color || self.is_joker_called() {
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
            },

            Card::Joker(c1) => match rhs {
                Card::Normal(c2, _) => {
                    if *c1 != cur_color || self.is_joker_called() {
                        true
                    } else if let Some(giruda) = giruda {
                        if *c2 == giruda {
                            *c1 != giruda_color.unwrap()
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
    fn process<S: AsRef<str>>(&self, args: Vec<S>) -> Result<BasicState, GameError> {
        let args = args.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

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
                        "game state is not same. expected: 'n', actual: '{}'",
                        args[1]
                    )));
                }

                let mut deck = BasicGame::get_random_deck();
                let left = deck.pop().unwrap();

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
                        "game state is not same. expected: 'e', actual: '{}'",
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
                if !"sdhcnx".contains(&args[2]) {
                    return Err(GameError::CommandError(format!(
                        "third argument should be one of 's', 'd', 'h', 'c', 'n', 'x', actual: '{}'",
                        args[2]
                    )));
                }

                let i = args[0].parse::<usize>().unwrap();
                let mut done = done.clone();

                if args[2] == "x" {
                    done[i] = true;

                    if done.iter().all(|x| *x) {
                        let mut candidate = Vec::new();

                        let mut last_max = 0u8;
                        for (i, p) in pledge.iter().enumerate() {
                            let (_, c) = p;
                            match c.cmp(&last_max) {
                                Ordering::Greater => {
                                    candidate = vec![i];
                                    last_max = *c;
                                }
                                Ordering::Equal => {
                                    candidate.push(i);
                                }
                                _ => {}
                            }
                        }

                        let mut deck = deck.clone();
                        let president = if last_max == 0 {
                            rand::thread_rng().gen_range(0, 5)
                        } else {
                            *candidate
                                .choose(&mut rand::thread_rng())
                                .unwrap_or(&rand::thread_rng().gen_range(0, 5))
                        };
                        deck[president].append(&mut left.clone());
                        Ok(BasicState::SelectFriend {
                            president,
                            pledge: pledge[president].clone(),
                            deck,
                        })
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
                    let mut curr_max = 0u8;
                    for p in pledge.iter() {
                        let (_, c) = p;
                        if curr_max < *c {
                            curr_max = *c;
                        }
                    }

                    if args[2] == "n" {
                        if c < 12 {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than 12 in no giruda game, actual: {}",
                                c
                            )));
                        }
                        if c < curr_max - 1 {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than current maximum - 1: {}, actual: {}",
                                curr_max - 1, c
                            )));
                        }
                        pledge[i] = (None, c);
                    } else {
                        if c < 13 {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than 13, actual: {}",
                                c
                            )));
                        }
                        if c < curr_max {
                            return Err(GameError::CommandError(format!(
                                "pledge should be greater or equal than current maximum: {}, actual: {}",
                                curr_max, c
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
                        "game state is not same. expected: 'f', actual: '{}'",
                        args[1]
                    )));
                }

                let i = args[0].parse::<usize>().unwrap();

                if i != *president {
                    return Err(GameError::CommandError(
                        "you are not the president of this game".to_owned(),
                    ));
                }

                let fn_type = args[2].parse::<usize>().map_err(|_| {
                    GameError::CommandError(format!(
                        "unrecognized function type of friend, expected: 0~3, actual: {}",
                        args[2]
                    ))
                })?;

                let friend_func = match fn_type {
                    1 => {
                        if args.len() < 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be greater or equal than 4, actual: {}",
                                args.len()
                            )));
                        }

                        let card = args[3].parse::<Card>().map_err(|_| {
                            GameError::CommandError(format!(
                                "failed to parse card, actual: '{}'",
                                args[3]
                            ))
                        })?;

                        FriendFunc::ByCard(card)
                    }
                    2 => {
                        if args.len() < 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be greater or equal than 4, actual: {}",
                                args.len()
                            )));
                        }

                        FriendFunc::ByUser(args[3].parse::<usize>().unwrap_or(*president))
                    }
                    3 => {
                        if args.len() < 4 {
                            return Err(GameError::CommandError(format!(
                                "command length should be greater or equal than 4, actual: {}",
                                args.len()
                            )));
                        }

                        FriendFunc::ByWinning(args[3].parse::<u8>().unwrap_or(0))
                    }
                    _ => FriendFunc::None,
                };

                if args.len() < 8 {
                    return Err(GameError::CommandError(format!(
                        "command length should be greater or equal than 8, actual: {}",
                        args.len()
                    )));
                }
                let mut deck = deck.clone();
                for item in args.iter().take(8).skip(4) {
                    let card = item.parse::<Card>().map_err(|_| {
                        GameError::CommandError("error occurred when parsing card".to_owned())
                    })?;
                    let idx = match deck[i].iter().position(|x| *x == card) {
                        Some(x) => x,
                        _ => {
                            return Err(GameError::CommandError(
                                "the drop card is not in your deck".to_owned(),
                            ));
                        }
                    };
                    deck[i].remove(idx);
                }

                let (_, pledge) = pledge;
                let friend = match &friend_func {
                    FriendFunc::None => None,
                    FriendFunc::ByCard(c) => {
                        let mut res = None;

                        for (i, d) in deck.iter().enumerate() {
                            if d.contains(c) {
                                res = Some(i);
                            }
                        }

                        res
                    }
                    FriendFunc::ByUser(u) => Some(*u),
                    FriendFunc::ByWinning(_) => None,
                };

                let is_friend_known = match &friend_func {
                    FriendFunc::None => true,
                    FriendFunc::ByCard(_) => false,
                    FriendFunc::ByUser(_) => true,
                    FriendFunc::ByWinning(_) => false,
                };

                Ok(BasicState::InGame {
                    president: *president,
                    friend_func,
                    friend,
                    is_friend_known,
                    giruda: None,
                    pledge: *pledge,
                    deck,
                    score_deck: Vec::new(),
                    turn_count: 0,
                    placed_cards: vec![Card::Normal(CardType::Spade, 0); 5],
                    start_user: *president,
                    current_user: *president,
                    current_pattern: RushType::Spade,
                    is_joker_called: false,
                })
            }

            // command is 'g'
            BasicState::InGame {
                president,
                friend_func,
                friend,
                is_friend_known,
                giruda,
                pledge,
                deck,
                score_deck,
                turn_count,
                placed_cards,
                start_user,
                current_user,
                current_pattern,
                is_joker_called,
            } => {
                if args.len() < 3 {
                    return Err(GameError::CommandError(format!(
                        "command length should be greater or equal to 3, actual: {}",
                        args.len()
                    )));
                }

                if args[1] != "g" {
                    return Err(GameError::CommandError(format!(
                        "game state is not same. expected: 'g', actual: '{}'",
                        args[1]
                    )));
                }

                let i = args[0].parse::<usize>().unwrap();

                if i != *current_user {
                    return Err(GameError::CommandError(
                        "you are not the current player".to_owned(),
                    ));
                }

                let card = args[2].parse::<Card>().map_err(|_| {
                    GameError::CommandError("error occurred when parsing card".to_owned())
                })?;

                if !deck[i].contains(&card) {
                    return Err(GameError::CommandError(
                        "your card is not in deck".to_owned(),
                    ));
                }

                let mut friend = *friend;
                let mut is_friend_known = *is_friend_known;
                let mut deck = deck.clone();
                let mut score_deck = score_deck.clone();
                let mut turn_count = *turn_count;
                let mut placed_cards = placed_cards.clone();
                let mut start_user = *start_user;
                let mut current_pattern = current_pattern.clone();
                let mut is_joker_called = *is_joker_called;

                for j in 0..deck.len() {
                    if deck[i][j] == card {
                        deck[i].remove(j);
                        break;
                    }
                }

                placed_cards[i] = card.clone();

                is_friend_known = match friend_func {
                    FriendFunc::ByCard(c) => card.eq(c),
                    _ => is_friend_known,
                };

                if *current_user == start_user {
                    current_pattern = RushType::from(card.clone());

                    match card {
                        Card::Normal(t, n) => {
                            let mut joker_calls = Vec::new();

                            if let Some(giruda) = giruda {
                                joker_calls.push(if CardType::Clover.eq(giruda) {
                                    CardType::Spade
                                } else {
                                    CardType::Clover
                                });

                                joker_calls.push(if CardType::Heart.eq(giruda) {
                                    CardType::Diamond
                                } else {
                                    CardType::Heart
                                });
                            } else {
                                joker_calls.push(CardType::Clover);
                                joker_calls.push(CardType::Heart);
                            }

                            if joker_calls.contains(&t) && n == 2 {
                                is_joker_called = if args.len() >= 4 {
                                    match args[3] {
                                        "t" => true,
                                        "f" => false,
                                        _ => {
                                            return Err(GameError::CommandError(format!(
                                                "joker call command expected: t or f, actual: {}",
                                                args[3]
                                            )))
                                        }
                                    }
                                } else {
                                    true
                                }
                            }
                        }

                        Card::Joker(t) => {
                            if args.len() < 4 {
                                return Err(GameError::CommandError(format!(
                                    "command length should be greater or equal to 4, actual: {}",
                                    args.len()
                                )));
                            }

                            current_pattern = RushType::from_str(args[3]).map_err(|_| {
                                GameError::CommandError(
                                    "error occurred when parsing rush type".to_owned(),
                                )
                            })?;

                            let containing = match t {
                                ColorType::Black => {
                                    current_pattern == RushType::Black
                                        || current_pattern == RushType::Spade
                                        || current_pattern == RushType::Clover
                                }
                                ColorType::Red => {
                                    current_pattern == RushType::Red
                                        || current_pattern == RushType::Diamond
                                        || current_pattern == RushType::Heart
                                }
                            };

                            if !containing {
                                return Err(GameError::CommandError(
                                    "rush type is not in joker type".to_owned(),
                                ));
                            }
                        }
                    }
                }

                let mut next_user = (*current_user + 1) % 5;

                if next_user == start_user {
                    let mut winner = Option::<usize>::None;

                    for i in 0..5 {
                        let c = &placed_cards[i];

                        match c {
                            Card::Normal(t, _) => {
                                if turn_count == 0 && current_pattern.contains(t) {
                                    continue;
                                }
                            }
                            Card::Joker(_) => {
                                if turn_count == 0 || turn_count == 9 {
                                    continue;
                                }
                            }
                        }

                        winner = match winner {
                            Some(j) => {
                                if self.compare_cards(&placed_cards[i], &placed_cards[j]) {
                                    Some(j)
                                } else {
                                    Some(i)
                                }
                            }
                            None => Some(i),
                        };
                    }

                    let winner = winner.ok_or_else(|| {
                        GameError::InternalError(
                            "internal error occurred when calculating score".to_owned(),
                        )
                    })?;

                    if friend == None {
                        friend = match friend_func {
                            FriendFunc::ByWinning(j) => {
                                if *j == turn_count {
                                    is_friend_known = true;
                                    Some(winner)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };
                    }

                    if i != *president || (is_friend_known && matches!(friend, Some(j) if j == i)) {
                        let mut score_cards = placed_cards
                            .iter()
                            .filter_map(|c| if c.is_score() { Some(c.clone()) } else { None })
                            .collect::<Vec<_>>();
                        score_deck[winner].append(&mut score_cards);
                    }

                    start_user = winner;
                    next_user = start_user;
                    turn_count += 1;

                    if turn_count == 10 {
                        let mut mul = 1;
                        let friend = match friend {
                            Some(x) => {
                                if *giruda == None {
                                    mul = 2;
                                }
                                x
                            }
                            _ => {
                                mul = 2;
                                6
                            }
                        };
                        let mut score = 0;
                        let mut winner = 0;
                        for i in 0..5 {
                            if i == *president || i == friend {
                                score += score_deck[i as usize].len() as u8;
                                winner += 1 << i;
                            }
                        } // >
                        if score >= *pledge {
                            score = mul * (score - 10);
                        } else {
                            score = *pledge + score - 20;
                            winner = (1 << 5) - winner;
                        }; // >
                        return Ok(BasicState::GameEnded {
                            winner,
                            president: *president,
                            friend,
                            score,
                        });
                    }
                }

                Ok(BasicState::InGame {
                    president: *president,
                    friend_func: friend_func.clone(),
                    friend,
                    is_friend_known,
                    giruda: giruda.clone(),
                    pledge: *pledge,
                    deck,
                    score_deck,
                    turn_count,
                    placed_cards,
                    start_user,
                    current_user: next_user,
                    current_pattern,
                    is_joker_called,
                })
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
                    is_friend_known: false,
                    giruda: CardType::from_str(giruda).ok(),
                    pledge: 0,
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
        assert_eq!(compare_cards(&g, "jb", "d1"), true);
        assert_eq!(compare_cards(&g, "d1", "jr"), true);
        assert_eq!(compare_cards(&g, "jr", "d1"), false);
        assert_eq!(compare_cards(&g, "jr", "s1"), true);
        assert_eq!(compare_cards(&g, "s1", "jr"), false);

        let g = make_game("d", "c", true);
        assert_eq!(compare_cards(&g, "jb", "c1"), true);
        assert_eq!(compare_cards(&g, "c1", "jb"), false);
        assert_eq!(compare_cards(&g, "jb", "c3"), true);
        assert_eq!(compare_cards(&g, "c3", "jb"), false);

        let g = make_game("", "c", false);
        assert_eq!(compare_cards(&g, "jb", "jr"), false);
        assert_eq!(compare_cards(&g, "s0", "jb"), false);
        assert_eq!(compare_cards(&g, "jb", "s0"), true);
        assert_eq!(compare_cards(&g, "jb", "c0"), false);
        assert_eq!(compare_cards(&g, "c0", "jb"), true);
        assert_eq!(compare_cards(&g, "s1", "c1"), true);
        assert_eq!(compare_cards(&g, "c1", "c0"), true);

        let g = make_game("", "c", true);
        assert_eq!(compare_cards(&g, "c1", "jb"), false);
        assert_eq!(compare_cards(&g, "jb", "c1"), true);

        let g = make_game("s", "c", false);
        assert_eq!(compare_cards(&g, "jb", "s1"), false);
        assert_eq!(compare_cards(&g, "s1", "jb"), true);
    }

    #[test]
    fn user_test() {
        let mut g = BasicGame::new();

        assert_eq!(g.len(), 0);
        assert_eq!(g.is_empty(), true);
        assert_eq!(g.add_user(1), true);
        assert_eq!(g.add_user(1), false);

        assert_eq!(g.len(), 1);
        assert_eq!(g.is_empty(), false);
        assert_eq!(g.add_user(2), true);
        assert_eq!(g.add_user(3), true);

        assert_eq!(g.remove_user(4), false);
        assert_eq!(g.remove_user(2), true);
        assert_eq!(g.remove_user(2), false);

        assert_eq!(g.len(), 2);
        assert_eq!(g.get_index(1), Some(0));
        assert_eq!(g.get_index(2), None);
        assert_eq!(g.get_index(3), Some(2));

        assert_eq!(g.get_user_list(), vec![1, 3]);

        assert_eq!(g.add_user(4), true);
        assert_eq!(g.add_user(5), true);
        assert_eq!(g.add_user(6), true);
        assert_eq!(g.add_user(7), false);
    }

    #[test]
    fn process_test() {
        let mut g: BasicGame = Default::default();

        assert_eq!(g.get_state(), "n");
        assert_eq!(
            g.process(vec!["0"]).err().unwrap(),
            GameError::CommandError(String::from("command length should be 2, actual: 1"))
        );
        assert_eq!(
            g.process(vec!["0", "s"]).err().unwrap(),
            GameError::CommandError(String::from(
                "game state is not same. expected: 'n', actual: 's'"
            ))
        );
        g.state = g.process(vec!["0", "n"]).unwrap();
        assert_eq!(g.get_state(), "e");
    }
}
