use crate::base::*;
use crate::error::{Error, Result};
use parse_display::{Display, FromStr, ParseError};
use rand::seq::SliceRandom;
use rand::Rng;

/// type of friend making
#[derive(PartialEq, Clone, Debug, Display, FromStr)]
pub enum BasicFriendFunc {
    #[display("n")]
    None,
    #[display("c{0}")]
    ByCard(Card),
    #[display("u{0}")]
    ByUser(usize),
    #[display("w{0}")]
    ByWinning(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BasicCommand {
    // user-id
    StartGame(usize),
    // user-id, giruda, pledge (0 for done)
    Pledge(usize, Option<CardType>, u8),
    // user-id, friend function type, dropped cards
    SelectFriend(usize, BasicFriendFunc, Vec<Card>),
    // user-id, card to place, type to rush (if joker & first of turn), joker called (if right card)
    Go(usize, Card, RushType, bool),
    // user-id
    Random(usize),
}

impl std::str::FromStr for BasicCommand {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // todo

        let p_num = s
            .get(1..2)
            .ok_or_else(ParseError::new)?
            .parse::<usize>()
            .unwrap();

        match s.get(0..1).ok_or_else(ParseError::new)? {
            "h" => Ok(Self::StartGame(p_num)),
            "p" => match s.get(2..3).ok_or_else(ParseError::new)? {
                "n" => {
                    let num = s
                        .get(3..4)
                        .ok_or_else(ParseError::new)?
                        .parse::<u8>()
                        .unwrap();
                    Ok(Self::Pledge(p_num, None, num + 12))
                }
                "s" | "d" | "h" | "c" => {
                    let num = s
                        .get(3..4)
                        .ok_or_else(ParseError::new)?
                        .parse::<u8>()
                        .unwrap();
                    Ok(Self::Pledge(
                        p_num,
                        Some(
                            s.get(2..3)
                                .ok_or_else(ParseError::new)?
                                .parse::<CardType>()
                                .unwrap(),
                        ),
                        num + 13,
                    ))
                }
                _ => Err(ParseError::new()),
            },

            "s" => {
                let dropped: Vec<Card> = vec![
                    s.get(2..4)
                        .ok_or_else(ParseError::new)?
                        .parse::<Card>()
                        .unwrap(),
                    s.get(4..6)
                        .ok_or_else(ParseError::new)?
                        .parse::<Card>()
                        .unwrap(),
                    s.get(6..8)
                        .ok_or_else(ParseError::new)?
                        .parse::<Card>()
                        .unwrap(),
                    s.get(8..10)
                        .ok_or_else(ParseError::new)?
                        .parse::<Card>()
                        .unwrap(),
                ];

                match s.get(10..11).ok_or_else(ParseError::new)? {
                    "n" => Ok(Self::SelectFriend(p_num, BasicFriendFunc::None, dropped)),
                    "c" => Ok(Self::SelectFriend(
                        p_num,
                        BasicFriendFunc::ByCard(
                            s.get(11..13)
                                .ok_or_else(ParseError::new)?
                                .parse::<Card>()
                                .unwrap(),
                        ),
                        dropped,
                    )),
                    "u" => {
                        let num = s
                            .get(11..12)
                            .ok_or_else(ParseError::new)?
                            .parse::<usize>()
                            .unwrap();
                        Ok(Self::SelectFriend(
                            p_num,
                            BasicFriendFunc::ByUser(num),
                            dropped,
                        ))
                    }
                    "w" => {
                        let num = s
                            .get(11..12)
                            .ok_or_else(ParseError::new)?
                            .parse::<u8>()
                            .unwrap();
                        Ok(Self::SelectFriend(
                            p_num,
                            BasicFriendFunc::ByWinning(num),
                            dropped,
                        ))
                    }
                    _ => Err(ParseError::new()),
                }
            }

            "g" => {
                let p_card = s
                    .get(2..4)
                    .ok_or_else(ParseError::new)?
                    .parse::<Card>()
                    .unwrap();
                let p_rushtype = s
                    .get(4..5)
                    .ok_or_else(ParseError::new)?
                    .parse::<RushType>()
                    .unwrap();
                let joker_called = s.get(5..6).ok_or_else(ParseError::new)? == "1";
                Ok(Self::Go(p_num, p_card, p_rushtype, joker_called))
            }

            "r" => {
                let num = s
                    .get(1..2)
                    .ok_or_else(ParseError::new)?
                    .parse::<usize>()
                    .unwrap();
                Ok(Self::Random(num))
            }

            _ => Err(ParseError::new()),
        }
    }
}

impl std::fmt::Display for BasicCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo
        match &*self {
            BasicCommand::StartGame(p_num) => write!(f, "h{}", p_num),
            BasicCommand::Pledge(p_num, ctype, num) => match ctype {
                None => {
                    if *num < 12 {
                        panic!("Invalid Pledge: {}", num);
                    }
                    write!(f, "p{}n{}", p_num, num - 12)
                }
                Some(n) => {
                    if *num < 13 {
                        panic!("Invalid Pledge: {}", num);
                    }
                    write!(f, "p{}{}{}", p_num, n.to_string(), num - 13)
                }
            },
            BasicCommand::SelectFriend(p_num, f_function, cvec) => {
                if cvec.len() != 4 {
                    panic!("Invalid Vector Size");
                }

                match f_function {
                    BasicFriendFunc::ByCard(card) => write!(
                        f,
                        "s{}{}{}{}{}c{}",
                        p_num,
                        cvec[0].to_string(),
                        cvec[1].to_string(),
                        cvec[2].to_string(),
                        cvec[3].to_string(),
                        card.to_string()
                    ),
                    BasicFriendFunc::ByUser(num) => write!(
                        f,
                        "s{}{}{}{}{}u{}",
                        p_num,
                        cvec[0].to_string(),
                        cvec[1].to_string(),
                        cvec[2].to_string(),
                        cvec[3].to_string(),
                        num
                    ),
                    BasicFriendFunc::ByWinning(num) => write!(
                        f,
                        "s{}{}{}{}{}w{}",
                        p_num,
                        cvec[0].to_string(),
                        cvec[1].to_string(),
                        cvec[2].to_string(),
                        cvec[3].to_string(),
                        num
                    ),
                    BasicFriendFunc::None => write!(
                        f,
                        "s{}{}{}{}{}n",
                        p_num,
                        cvec[0].to_string(),
                        cvec[1].to_string(),
                        cvec[2].to_string(),
                        cvec[3].to_string()
                    ),
                }
            }
            BasicCommand::Go(p_num, card, rush_type, joker_called) => {
                let num = match joker_called {
                    true => 1,
                    false => 0,
                };
                write!(
                    f,
                    "g{}{}{}{}",
                    p_num,
                    card.to_string(),
                    rush_type.to_string(),
                    num
                )
            }
            BasicCommand::Random(p_num) => write!(f, "r{}", p_num),
        }
    }
}

impl BasicCommand {
    pub fn random(user: usize) -> BasicCommand {
        BasicCommand::Random(user)
    }
}

/// State of basic mighty game.
///
/// - `NotStarted`: When game is not started
/// - `Election`: After passing out cards,
/// - `SelectFriend`: After election, president will select friend (or not)
/// - `InGame`: After selecting friend, they will play 10 turns
#[derive(Clone, Debug, PartialEq)]
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
        friend_func: BasicFriendFunc,
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
        friend: Option<usize>,
        score: u8,
        pledge: u8,
        giruda: Option<CardType>,
    },
}

impl Default for BasicState {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicState {
    pub fn new() -> BasicState {
        BasicState::NotStarted
    }

    /// Check if joker called.
    /// **Valid output only in in-game.**
    fn is_joker_called(&self) -> bool {
        if let BasicState::InGame {
            is_joker_called, ..
        } = self
        {
            *is_joker_called
        } else {
            false
        }
    }

    /// Get the current pattern of this turn.
    /// **Valid output only in in-game.**
    fn get_current_pattern(&self) -> RushType {
        match self {
            BasicState::InGame {
                current_pattern, ..
            } => *current_pattern,
            // don't need this value
            _ => RushType::Spade,
        }
    }

    /// Get the giruda of this turn.
    /// **Valid output only in in-game.**
    fn get_giruda(&self) -> Option<CardType> {
        match self {
            BasicState::InGame { giruda, .. } => *giruda,
            // don't need this value
            _ => None,
        }
    }

    /// Get the mighty card in game
    /// **Valid output only in in-game.**
    fn get_mighty(&self) -> Card {
        match self {
            BasicState::InGame { giruda, .. } => match giruda {
                Some(CardType::Spade) => Card::Normal(CardType::Diamond, 0),
                _ => Card::Normal(CardType::Spade, 0),
            },
            // don't need this value
            _ => Card::Normal(CardType::Spade, 0),
        }
    }

    pub fn get_state(&self) -> &str {
        match self {
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
                    if v.len() == 10 {
                        v.iter()
                            .map(|c| {
                                if Card::Normal(CardType::Spade, 0) == *c {
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
                    } else {
                        3
                    }
                })
                .all(|s| s > 2);

            if is_not_missed_deal {
                break deck;
            }
        }
    }
}

impl BasicState {
    // true if lhs < rhs
    // undefined when lhs == rhs
    fn compare_cards(&self, lhs: &Card, rhs: &Card) -> bool {
        let mighty = self.get_mighty();
        if *lhs == mighty {
            return false;
        }
        if *rhs == mighty {
            return true;
        }

        let cur_pat = self.get_current_pattern();
        let cur_color = ColorType::from(cur_pat);
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

    /// Process the given arguments and change the game state.
    /// First argument has to be the *in-game user id*
    /// who sent this command. **(always in bounds)**
    /// Second argument has to be the state of the game
    /// for checking command
    /// Third and after is different for each state.
    fn process(&self, cmd: BasicCommand) -> Result<BasicState> {
        match self {
            BasicState::NotStarted => match cmd {
                BasicCommand::StartGame(user_id) => {
                    if user_id != 0 {
                        return Err(Error::NotLeader);
                    }

                    let mut deck = BasicState::get_random_deck();
                    let left = deck.pop().unwrap();

                    Ok(BasicState::Election {
                        pledge: vec![(None, 0); 5],
                        done: vec![false; 5],
                        deck,
                        left,
                    })
                }
                _ => Err(Error::InvalidCommand("BasicCommand::StartGame")),
            },

            BasicState::Election {
                pledge,
                done,
                deck,
                left,
            } => match cmd {
                BasicCommand::Pledge(user_id, c, p) => {
                    let mut done = done.clone();
                    let mut pledge = pledge.clone();

                    if p > 20 {
                        return Err(Error::InvalidPledge(true, 20));
                    }

                    if p != 0 {
                        done[user_id] = false;
                        let max_pledge = pledge.iter().map(|(_, j)| *j).max().unwrap();
                        let max_pledge = std::cmp::max(max_pledge, 13);
                        let offset = if matches!(c, None) { 1 } else { 0 };

                        if p < max_pledge - offset {
                            return Err(Error::InvalidPledge(false, max_pledge - offset));
                        }

                        pledge[user_id] = (c, p);

                        Ok(BasicState::Election {
                            pledge,
                            done,
                            deck: deck.clone(),
                            left: left.clone(),
                        })
                    } else {
                        done[user_id] = true;

                        if done.iter().all(|x| *x) {
                            let mut candidate = Vec::new();

                            let mut last_max = 0u8;
                            for (i, p) in pledge.iter().enumerate() {
                                let (_, c) = p;
                                match c.cmp(&last_max) {
                                    std::cmp::Ordering::Greater => {
                                        candidate = vec![i];
                                        last_max = *c;
                                    }
                                    std::cmp::Ordering::Equal => {
                                        candidate.push(i);
                                    }
                                    _ => {}
                                }
                            }

                            let mut deck = deck.clone();
                            let president =
                                candidate.choose(&mut rand::thread_rng()).copied().unwrap();
                            let mut pledge = pledge[president];
                            if last_max == 0 {
                                let pledge_vec = vec![
                                    (None, 12),
                                    (Some(CardType::Spade), 13),
                                    (Some(CardType::Diamond), 13),
                                    (Some(CardType::Heart), 13),
                                    (Some(CardType::Clover), 13),
                                ];
                                pledge =
                                    pledge_vec.choose(&mut rand::thread_rng()).copied().unwrap();
                            }

                            deck[president].append(&mut left.clone());
                            Ok(BasicState::SelectFriend {
                                president,
                                pledge,
                                deck,
                            })
                        } else {
                            Ok(BasicState::Election {
                                pledge,
                                done,
                                deck: deck.clone(),
                                left: left.clone(),
                            })
                        }
                    }
                }
                BasicCommand::Random(user_id) => {
                    self.process(BasicCommand::Pledge(user_id, None, 0))
                }
                _ => Err(Error::InvalidCommand("BasicCommand::Pledge")),
            },

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
            } => match cmd {
                BasicCommand::SelectFriend(user_id, friend_func, drop_card) => {
                    if user_id != *president {
                        return Err(Error::NotPresident);
                    }

                    let mut deck = deck.clone();
                    for card in drop_card.iter() {
                        let idx = deck[user_id]
                            .iter()
                            .position(|x| *x == *card)
                            .ok_or(Error::NotInDeck)?;
                        deck[user_id].remove(idx);
                    }

                    let (giruda, pledge) = *pledge;
                    let friend = match &friend_func {
                        BasicFriendFunc::None => None,
                        BasicFriendFunc::ByCard(c) => deck
                            .iter()
                            .enumerate()
                            .filter(|(i, d)| *i != *president && d.contains(c))
                            .map(|(i, _)| i)
                            .next(),
                        BasicFriendFunc::ByUser(u) => Some(*u).filter(|_| *u != *president),
                        BasicFriendFunc::ByWinning(_) => None,
                    };

                    let is_friend_known =
                        matches!(&friend_func, BasicFriendFunc::None | BasicFriendFunc::ByUser(_));

                    Ok(BasicState::InGame {
                        president: *president,
                        friend_func,
                        friend,
                        is_friend_known,
                        giruda,
                        pledge,
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
                BasicCommand::Random(user_id) => self.process(BasicCommand::SelectFriend(
                    user_id,
                    BasicFriendFunc::ByUser(rand::thread_rng().gen_range(0, 5)),
                    deck[user_id]
                        .choose_multiple(&mut rand::thread_rng(), 4)
                        .cloned()
                        .collect(),
                )),
                _ => Err(Error::InvalidCommand("BasicCommand::SelectFriend")),
            },

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
            } => match cmd {
                BasicCommand::Go(user_id, card, rush_type, joker_call) => {
                    if user_id != *current_user {
                        return Err(Error::InvalidUser(*current_user));
                    }

                    let mut friend = *friend;
                    let mut is_friend_known = *is_friend_known;
                    let mut deck = deck.clone();
                    let mut score_deck = score_deck.clone();
                    let mut turn_count = *turn_count;
                    let mut placed_cards = placed_cards.clone();
                    let mut start_user = *start_user;
                    let mut current_pattern = *current_pattern;
                    let mut is_joker_called = *is_joker_called;

                    placed_cards[user_id] = card.clone();

                    is_friend_known = match friend_func {
                        BasicFriendFunc::ByCard(c) => *c == card,
                        _ => is_friend_known,
                    };

                    let idx = deck[user_id]
                        .iter()
                        .position(|x| *x == card)
                        .ok_or(Error::NotInDeck)?;

                    if *current_user == start_user {
                        current_pattern = RushType::from(card.clone());
                        is_joker_called = false;

                        if !deck[user_id].iter().all(|x| match *x {
                            Card::Normal(t, _) => {
                                *x == self.get_mighty() || matches!(giruda, Some(y) if t == *y)
                            }
                            Card::Joker(_) => true,
                        }) && matches!(giruda, Some(y) if RushType::from(*y) == current_pattern)
                        {
                            return Err(Error::WrongCard);
                        }
                        deck[user_id].remove(idx);

                        match card {
                            Card::Normal(t, n) => {
                                let mut joker_calls = Vec::new();

                                joker_calls.push(if Some(CardType::Clover) == *giruda {
                                    CardType::Spade
                                } else {
                                    CardType::Clover
                                });

                                joker_calls.push(if Some(CardType::Heart) == *giruda {
                                    CardType::Diamond
                                } else {
                                    CardType::Heart
                                });

                                if joker_calls.contains(&t) && n == 2 {
                                    is_joker_called = joker_call;
                                }
                            }

                            Card::Joker(t) => {
                                current_pattern = rush_type;

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
                                    return Err(Error::WrongCardType(current_pattern));
                                }
                            }
                        }
                    } else if self.get_mighty() == card {
                        deck[user_id].remove(idx);
                    } else if !deck[user_id]
                        .iter()
                        .all(|x| !current_pattern.is_same_type(x))
                        && !current_pattern.is_same_type(&card)
                    {
                        return Err(Error::WrongCard);
                    } else {
                        deck[user_id].remove(idx);
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

                        let winner = winner.ok_or(Error::Internal(
                            "internal error occurred when calculating score",
                        ))?;

                        if let BasicFriendFunc::ByWinning(j) = friend_func {
                            friend = friend.or_else(|| {
                                Some(winner).filter(|_| *j == turn_count && winner != *president)
                            });
                            is_friend_known |= *j == turn_count;
                        }

                        {
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
                            if matches!(giruda, None) {
                                mul *= 2;
                            }
                            if matches!(friend_func, BasicFriendFunc::None) {
                                mul *= 2;
                            }

                            let president = *president;
                            let pledge = *pledge;

                            let mut score = score_deck[president].len() as u8;
                            let mut winner = 1 << president;
                            if let Some(f) = friend {
                                score += score_deck[f].len() as u8;
                                winner += 1 << f;
                            }

                            if score >= pledge {
                                score = mul * (score - 10);
                            } else {
                                score = pledge + score - 20;
                                winner = (1 << 5) - winner;
                            }

                            return Ok(BasicState::GameEnded {
                                winner,
                                president,
                                friend,
                                score,
                                pledge,
                                giruda: *giruda,
                            });
                        }
                    }

                    Ok(BasicState::InGame {
                        president: *president,
                        friend_func: friend_func.clone(),
                        friend,
                        is_friend_known,
                        giruda: *giruda,
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
                BasicCommand::Random(user_id) => {
                    let rand_card = deck[user_id].choose(&mut rand::thread_rng()).unwrap();
                    self.process(BasicCommand::Go(
                        user_id,
                        rand_card.clone(),
                        RushType::from(rand_card.clone()),
                        false,
                    ))
                }
                _ => Err(Error::InvalidCommand("BasicCommand::Go")),
            },

            // command is 'd'
            BasicState::GameEnded { .. } => {
                // todo
                Ok(self.clone())
            }
        }
    }
}

impl MightyState for BasicState {
    fn next(&self, cmd: String) -> Result<Box<dyn MightyState>> {
        let cmd = cmd.parse::<BasicCommand>()?;
        Ok(Box::new(self.process(cmd)?))
    }

    // todo: fill else
    fn generate(&self, user: usize) -> Box<dyn MightyState> {
        match self {
            BasicState::NotStarted => Box::new(BasicState::NotStarted),
            BasicState::Election {
                pledge, done, deck, ..
            } => {
                let v = deck
                    .iter()
                    .enumerate()
                    .map(|(i, d)| if i == user { d.clone() } else { Vec::new() })
                    .collect::<Vec<_>>();
                Box::new(BasicState::Election {
                    pledge: pledge.clone(),
                    done: done.clone(),
                    deck: v,
                    left: Vec::new(),
                })
            }
            BasicState::SelectFriend { .. } => Box::new(BasicState::NotStarted),
            BasicState::InGame { .. } => Box::new(BasicState::NotStarted),
            BasicState::GameEnded { .. } => Box::new(BasicState::NotStarted),
        }
    }
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn str_from_command_test() {
        assert_eq!("h2", BasicCommand::StartGame(2).to_string());
        assert_eq!("p1n3", BasicCommand::Pledge(1, None, 15).to_string());
        assert_eq!(
            "p1s4",
            BasicCommand::Pledge(1, Some(CardType::Spade), 17).to_string()
        );

        let dropped: Vec<Card> = vec![
            Card::Joker(ColorType::Black),
            Card::Joker(ColorType::Red),
            Card::Normal(CardType::Spade, 0),
            Card::Normal(CardType::Clover, 12),
        ];

        assert_eq!(
            "s1jbjrs0ccn",
            BasicCommand::SelectFriend(1, BasicFriendFunc::None, dropped.clone()).to_string()
        );

        assert_eq!(
            "s1jbjrs0cccsb",
            BasicCommand::SelectFriend(
                1,
                BasicFriendFunc::ByCard(Card::Normal(CardType::Spade, 11)),
                dropped.clone()
            )
            .to_string()
        );
        assert_eq!(
            "s1jbjrs0ccu4",
            BasicCommand::SelectFriend(1, BasicFriendFunc::ByUser(4), dropped.clone()).to_string()
        );
        assert_eq!(
            "s1jbjrs0ccw6",
            BasicCommand::SelectFriend(1, BasicFriendFunc::ByWinning(6), dropped.clone())
                .to_string()
        );

        assert_eq!(
            "g3s3b1",
            BasicCommand::Go(3, Card::Normal(CardType::Spade, 3), RushType::Black, true)
                .to_string()
        );
        assert_eq!(
            "g3jrs0",
            BasicCommand::Go(3, Card::Joker(ColorType::Red), RushType::Spade, false).to_string()
        );

        assert_eq!("r0", BasicCommand::Random(0).to_string());
        assert_eq!("r3", BasicCommand::Random(3).to_string());
    }

    #[test]
    fn command_from_str_test() {
        assert_eq!("h0".parse(), Ok(BasicCommand::StartGame(0)));
        assert_eq!("h2".parse(), Ok(BasicCommand::StartGame(2)));

        assert_eq!("p2n0".parse(), Ok(BasicCommand::Pledge(2, None, 12)));
        assert_eq!("p2n8".parse(), Ok(BasicCommand::Pledge(2, None, 20)));
        assert_eq!(
            "p2h0".parse(),
            Ok(BasicCommand::Pledge(2, Some(CardType::Heart), 13))
        );
        assert_eq!(
            "p2h7".parse(),
            Ok(BasicCommand::Pledge(2, Some(CardType::Heart), 20))
        );

        let dropped: Vec<Card> = vec![
            Card::Joker(ColorType::Black),
            Card::Joker(ColorType::Red),
            Card::Normal(CardType::Spade, 0),
            Card::Normal(CardType::Clover, 12),
        ];

        assert_eq!(
            "s1jbjrs0ccn".parse(),
            Ok(BasicCommand::SelectFriend(
                1,
                BasicFriendFunc::None,
                dropped.clone()
            ))
        );
        assert_eq!(
            "s1jbjrs0cccsb".parse(),
            Ok(BasicCommand::SelectFriend(
                1,
                BasicFriendFunc::ByCard(Card::Normal(CardType::Spade, 11)),
                dropped.clone()
            ))
        );
        assert_eq!(
            "s1jbjrs0ccu4".parse(),
            Ok(BasicCommand::SelectFriend(
                1,
                BasicFriendFunc::ByUser(4),
                dropped.clone()
            ))
        );
        assert_eq!(
            "s1jbjrs0ccw6".parse(),
            Ok(BasicCommand::SelectFriend(
                1,
                BasicFriendFunc::ByWinning(6),
                dropped.clone()
            ))
        );

        assert_eq!(
            "g3s3b1".parse(),
            Ok(BasicCommand::Go(
                3,
                Card::Normal(CardType::Spade, 3),
                RushType::Black,
                true
            ))
        );
        assert_eq!(
            "g3jrs0".parse(),
            Ok(BasicCommand::Go(
                3,
                Card::Joker(ColorType::Red),
                RushType::Spade,
                false
            ))
        );

        assert_eq!("r0".parse(), Ok(BasicCommand::Random(0)));
        assert_eq!("r3".parse(), Ok(BasicCommand::Random(3)));
    }

    #[test]
    fn compare_cards_test() {
        fn make_game(giruda: &str, current_pattern: &str, is_joker_called: bool) -> BasicState {
            BasicState::InGame {
                president: 0,
                friend_func: BasicFriendFunc::None,
                friend: Option::None,
                is_friend_known: false,
                giruda: giruda.parse().ok(),
                pledge: 0,
                deck: vec![],
                score_deck: vec![],
                turn_count: 0,
                placed_cards: vec![],
                start_user: 0,
                current_user: 0,
                current_pattern: current_pattern.parse().unwrap(),
                is_joker_called,
            }
        }

        fn compare_cards(game: &BasicState, c1: &str, c2: &str) -> bool {
            game.compare_cards(&c1.parse().unwrap(), &c2.parse().unwrap())
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
    fn process_test() {
        let mut g = BasicState::new();

        assert_eq!(g.get_state(), "n");
        assert_eq!(
            g.process(BasicCommand::SelectFriend(
                0,
                BasicFriendFunc::None,
                vec![Card::Joker(ColorType::Red)]
            ))
            .err()
            .unwrap(),
            Error::InvalidCommand("BasicCommand::StartGame")
        );
        assert_eq!(
            g.process(BasicCommand::StartGame(1)).err().unwrap(),
            Error::NotLeader
        );

        g = g.process(BasicCommand::StartGame(0)).unwrap();
        assert_eq!(g.get_state(), "e");

        assert_eq!(
            g.process(BasicCommand::StartGame(0)).err().unwrap(),
            Error::InvalidCommand("BasicCommand::Pledge")
        );
        assert_eq!(
            g.process(BasicCommand::Pledge(0, None, 21)).err().unwrap(),
            Error::InvalidPledge(true, 20)
        );
        assert_eq!(
            g.process(BasicCommand::Pledge(0, None, 11)).err().unwrap(),
            Error::InvalidPledge(false, 12)
        );
        assert_eq!(
            g.process(BasicCommand::Pledge(0, Some(CardType::Spade), 12))
                .err()
                .unwrap(),
            Error::InvalidPledge(false, 13)
        );

        g = g
            .process(BasicCommand::Pledge(2, Some(CardType::Spade), 14))
            .unwrap();

        assert_eq!(
            g.process(BasicCommand::Pledge(0, Some(CardType::Spade), 13))
                .err()
                .unwrap(),
            Error::InvalidPledge(false, 14)
        );
        assert_eq!(
            g.process(BasicCommand::Pledge(0, None, 12)).err().unwrap(),
            Error::InvalidPledge(false, 13)
        );

        g = g.process(BasicCommand::Pledge(0, None, 0)).unwrap();
        g = g.process(BasicCommand::Pledge(1, None, 0)).unwrap();
        g = g.process(BasicCommand::Pledge(2, None, 0)).unwrap();
        g = g.process(BasicCommand::Pledge(3, None, 0)).unwrap();
        g = g.process(BasicCommand::Pledge(4, None, 0)).unwrap();
        assert_eq!(g.get_state(), "f");
    }
}
