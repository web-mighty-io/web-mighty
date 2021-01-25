use crate::card::{Card, Pattern, Rush};
use crate::rule::{card_policy::CardPolicy, election, Rule};
#[cfg(feature = "server")]
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "client", feature = "server"))]
use {
    crate::card::Color,
    crate::command::Command,
    crate::error::{Error, Result},
    crate::rule::friend,
};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum FriendFunc {
    None,
    ByCard(Card),
    ByUser(usize),
    First,
    Last,
}

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
        giruda: Option<Pattern>,
        // pledge score of ruling party
        pledge: u8,
        // deck for each user (len of 5)
        deck: Vec<Vec<Card>>,
        // score cards
        score_deck: Vec<Vec<Card>>,
        // turn count 0 to 9
        turn_count: u8,
        // placed cards in front of users
        placed_cards: Vec<(Card, CardPolicy)>,
        // start user of this turn
        start_user: usize,
        // current user of this turn
        current_user: usize,
        // current pattern of this turn
        current_pattern: Rush,
        // is joker called (user can decide)
        joker_call_card: Option<Card>,
        joker_call_effect: bool,
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
        giruda: Option<Pattern>,
    },
}

impl State {
    #[cfg(feature = "server")]
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
                        true
                    }
                })
                .all(|s| s);
            if is_not_missed_deal {
                break deck;
            }
        }
    }

    #[cfg(feature = "server")]
    fn is_joker_called(&self) -> bool {
        if let State::InGame { joker_call_card, .. } = self {
            *joker_call_card != None
        } else {
            false
        }
    }

    #[cfg(feature = "server")]
    fn get_current_pattern(&self) -> Rush {
        match self {
            State::InGame { current_pattern, .. } => *current_pattern,
            _ => Rush::SPADE,
        }
    }

    #[cfg(feature = "server")]
    fn get_giruda(&self) -> Option<Pattern> {
        match self {
            State::InGame { giruda, .. } => *giruda,
            // don't need this value
            _ => None,
        }
    }

    #[cfg(any(feature = "client", feature = "server"))]
    fn get_mighty(&self) -> Card {
        match self {
            State::InGame {
                giruda: Some(Pattern::Spade),
                ..
            } => Card::Normal(Pattern::Diamond, 0),
            // don't need this value
            _ => Card::Normal(Pattern::Spade, 0),
        }
    }

    #[cfg(any(feature = "client", feature = "server"))]
    fn check_card_valid(&self, c: (CardPolicy, CardPolicy)) -> bool {
        match self {
            State::InGame {
                turn_count,
                start_user,
                current_user,
                ..
            } => {
                if *turn_count == 0 {
                    c.0 == CardPolicy::Invalid || (c.0 == CardPolicy::InvalidForFirst && current_user == start_user)
                } else {
                    c.1 == CardPolicy::Invalid || (c.1 == CardPolicy::InvalidForFirst && current_user == start_user)
                }
            }
            // don't need this value
            _ => false,
        }
    }

    #[cfg(feature = "server")]
    fn check_card_effect(&self, c: (CardPolicy, CardPolicy)) -> bool {
        match self {
            State::InGame { turn_count, .. } => {
                (*turn_count == 0 && c.0 == CardPolicy::NoEffect) || (*turn_count == 9 && c.1 == CardPolicy::NoEffect)
            }
            // don't need this value
            _ => false,
        }
    }

    #[cfg(feature = "server")]
    fn compare_cards(&self, lhs: &Card, rhs: &Card) -> bool {
        let mighty = self.get_mighty();
        if *lhs == mighty {
            return false;
        }
        if *rhs == mighty {
            return true;
        }

        let cur_pat = self.get_current_pattern();
        let cur_color = Color::from(cur_pat);
        let giruda = self.get_giruda();
        let giruda_color = giruda.clone().map(Color::from);

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

                    if cur_pat.contains(Rush::from(*c1)) && cur_pat.contains(Rush::from(*c2)) {
                        n1 > n2
                    } else if cur_pat.contains(Rush::from(*c1)) || cur_pat.contains(Rush::from(*c2)) {
                        cur_pat.contains(Rush::from(*c2))
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

impl State {
    #[cfg(feature = "server")]
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

    #[cfg(feature = "server")]
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
                        return Err(Error::InvalidOrder);
                    }

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
                            let offset = if c == None { rule.pledge.no_giruda_offset } else { 0 };
                            let max_pledge = if start_user == user_id {
                                (max_pledge as i8 + offset + rule.pledge.first_offset) as u8
                            } else {
                                (max_pledge as i8 + offset) as u8
                            };
                            if p < std::cmp::max(max_pledge, rule.pledge.min) {
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
                            } else if !is_ordered && not_done.is_empty() {
                                is_election_done = true;
                                for (i, p) in pledge.iter().enumerate() {
                                    if let Some((_, c)) = p {
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
                                }
                            }
                            if is_election_done {
                                let mut deck = deck.clone();
                                let mut left = left.clone();
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
                                deck[president].append(&mut left);
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
                _ => Err(Error::InvalidCommand("Command::Pledge")),
            },
            State::SelectFriend {
                president,
                giruda,
                pledge,
                deck,
            } => match cmd {
                Command::SelectFriend(drop_card, friend_func) => {
                    let mut deck = deck.clone();
                    for card in drop_card.iter() {
                        let idx = deck[user_id].iter().position(|x| *x == *card).ok_or(Error::NotInDeck)?;
                        deck[user_id].remove(idx);
                    }
                    let friend = match friend_func {
                        FriendFunc::ByCard(c) => {
                            if !rule.friend.contains(friend::Friend::CARD) {
                                return Err(Error::InvalidFriendFunc);
                            }
                            let temp = deck
                                .iter()
                                .enumerate()
                                .filter(|(_, d)| d.contains(&c))
                                .map(|(i, _)| i)
                                .next();
                            if temp.unwrap() == *president && !rule.friend.contains(friend::Friend::FAKE) {
                                return Err(Error::InvalidFriendFunc);
                            }
                            temp
                        }
                        FriendFunc::ByUser(u) => Some(u).filter(|_| u != *president),
                        FriendFunc::None => {
                            if !rule.friend.contains(friend::Friend::NONE) {
                                return Err(Error::InvalidFriendFunc);
                            }
                            None
                        }
                        _ => None,
                    };
                    let is_friend_known = matches!(&friend_func, FriendFunc::None | FriendFunc::ByUser(_));
                    Ok(State::InGame {
                        president: *president,
                        friend_func,
                        friend,
                        is_friend_known,
                        giruda: *giruda,
                        pledge: *pledge,
                        deck,
                        score_deck: Vec::new(),
                        turn_count: 0,
                        placed_cards: vec![(Card::Normal(Pattern::Spade, 0), CardPolicy::Valid); 5],
                        start_user: *president,
                        current_user: *president,
                        current_pattern: Rush::from(Pattern::Spade),
                        joker_call_card: None,
                        joker_call_effect: false,
                    })
                }
                Command::ChangePledge(new_giruda) => {
                    if *giruda == new_giruda {
                        return Err(Error::SameGiruda);
                    }

                    let new_pledge = if matches!(giruda, None) {
                        ((*pledge) as i8 - rule.pledge.no_giruda_offset + rule.pledge.change_cost as i8) as u8
                    } else if matches!(new_giruda, None) {
                        ((*pledge) as i8 + rule.pledge.no_giruda_offset + rule.pledge.change_cost as i8) as u8
                    } else {
                        ((*pledge) as i8 + rule.pledge.change_cost as i8) as u8
                    };

                    if new_pledge > rule.pledge.max {
                        return Err(Error::InvalidPledge(true, rule.pledge.max));
                    }

                    Ok(State::SelectFriend {
                        president: *president,
                        giruda: new_giruda,
                        pledge: new_pledge,
                        deck: deck.clone(),
                    })
                }
                Command::Random => self.next(
                    user_id,
                    Command::SelectFriend(
                        deck[user_id]
                            .choose_multiple(&mut rand::thread_rng(), 4)
                            .cloned()
                            .collect(),
                        FriendFunc::None,
                    ),
                    rule,
                ),
                _ => Err(Error::InvalidCommand("Command::Pledge")),
            },
            State::InGame {
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
                joker_call_card,
                joker_call_effect,
            } => match cmd {
                Command::Go(card, rush_type, user_joker_call) => {
                    let mut friend = *friend;
                    let mut is_friend_known = *is_friend_known;
                    let mut deck = deck.clone();
                    let mut score_deck = score_deck.clone();
                    let mut turn_count = *turn_count;
                    let mut placed_cards = placed_cards.clone();
                    let mut start_user = *start_user;
                    let mut current_pattern = *current_pattern;
                    let mut joker_call_card = *joker_call_card;
                    let mut joker_call_effect = *joker_call_effect;

                    placed_cards[user_id] = (card, CardPolicy::Valid);

                    is_friend_known = match friend_func {
                        FriendFunc::ByCard(c) => *c == card,
                        _ => is_friend_known,
                    };

                    let mut joker_calls = Vec::new();
                    let mut is_noeffect = false;
                    joker_calls.push(
                        if matches!(*giruda, Some(y) if Rush::from(y) == Rush::from(rule.joker_call.cards[0].0)) {
                            rule.joker_call.cards[0].0
                        } else {
                            rule.joker_call.cards[0].1
                        },
                    );

                    joker_calls.push(
                        if matches!(*giruda, Some(y) if Rush::from(y) == Rush::from(rule.joker_call.cards[1].0)) {
                            rule.joker_call.cards[1].0
                        } else {
                            rule.joker_call.cards[1].1
                        },
                    );

                    if joker_call_card != None {
                        if !deck[user_id]
                            .iter()
                            .all(|x| matches!(joker_call_card, Some(y) if y == *x) || card == *x)
                        {
                            return Err(Error::JokerCall);
                        } else if joker_call_effect {
                            is_noeffect = true;
                        }
                    }

                    let idx = deck[user_id].iter().position(|x| *x == card).ok_or(Error::NotInDeck)?;
                    if turn_count == 0 || turn_count == 9 {
                        if card == self.get_mighty() {
                            if self.check_card_valid(rule.card_policy.mighty) {
                                return Err(Error::WrongCard);
                            }
                            if self.check_card_effect(rule.card_policy.mighty) {
                                is_noeffect = true;
                            }
                        } else if matches!(rule.card_policy.card.get(&card), Some(y) if self.check_card_valid(*y)) {
                            return Err(Error::WrongCard);
                        } else if matches!(rule.card_policy.card.get(&card), Some(y) if self.check_card_effect(*y)) {
                            is_noeffect = true;
                        } else {
                            match card {
                                Card::Normal(t, _) => {
                                    if Some(t) == *giruda {
                                        if self.check_card_valid(rule.card_policy.giruda) {
                                            return Err(Error::WrongCard);
                                        }
                                        if self.check_card_effect(rule.card_policy.giruda) {
                                            is_noeffect = true;
                                        }
                                    }
                                    if joker_calls.contains(&card)
                                        && user_joker_call
                                        && self.check_card_valid(rule.card_policy.joker_call)
                                    {
                                        return Err(Error::WrongCard);
                                    }
                                }
                                Card::Joker(_) => {
                                    if self.check_card_valid(rule.card_policy.joker) {
                                        return Err(Error::WrongCard);
                                    }
                                    if self.check_card_effect(rule.card_policy.joker) {
                                        is_noeffect = true;
                                    }
                                }
                            }
                        }
                        if is_noeffect {
                            placed_cards[user_id].1 = CardPolicy::NoEffect;
                        }
                    }

                    if *current_user == start_user {
                        current_pattern = Rush::from(card);
                        joker_call_card = None;
                        joker_call_effect = false;

                        if !deck[user_id].iter().all(|x| match *x {
                            Card::Normal(t, _) => *x == self.get_mighty() || matches!(giruda, Some(y) if t == *y),
                            Card::Joker(_) => true,
                        }) && matches!(giruda, Some(y) if Rush::from(*y) == current_pattern)
                            && rule.card_policy.giruda.0 == CardPolicy::InvalidForFirst
                        {
                            return Err(Error::WrongCard);
                        }

                        match card {
                            Card::Normal(..) => {
                                if joker_calls.contains(&card) && user_joker_call {
                                    joker_call_card = Some(card);
                                    if !(rule.card_policy.joker_call.0 == CardPolicy::NoEffect && turn_count == 0
                                        || rule.card_policy.joker_call.1 == CardPolicy::NoEffect && turn_count == 9)
                                    {
                                        joker_call_effect = true;
                                    }
                                }
                            }

                            Card::Joker(t) => {
                                current_pattern = rush_type;
                                let containing = match t {
                                    Color::Black => Rush::black().contains(current_pattern),
                                    Color::Red => Rush::red().contains(current_pattern),
                                };
                                if !containing {
                                    return Err(Error::WrongPattern);
                                }
                            }
                        }
                        deck[user_id].remove(idx);
                    } else if self.get_mighty() == card {
                        deck[user_id].remove(idx);
                    } else if !deck[user_id].iter().all(|x| !current_pattern == Rush::from(*x))
                        && !current_pattern == Rush::from(card)
                    {
                        return Err(Error::WrongCard);
                    } else {
                        deck[user_id].remove(idx);
                    }

                    let mut next_user = (*current_user + 1) % 5;

                    if next_user == start_user {
                        let mut winner = Option::<usize>::None;

                        for i in 0..5 {
                            let (_, p) = placed_cards[i];

                            if p == CardPolicy::NoEffect {
                                continue;
                            }

                            winner = match winner {
                                Some(j) => {
                                    if placed_cards[j].1 != CardPolicy::NoEffect
                                        && self.compare_cards(&placed_cards[i].0, &placed_cards[j].0)
                                    {
                                        Some(j)
                                    } else {
                                        Some(i)
                                    }
                                }
                                None => Some(i),
                            };
                        }
                        if winner == None {
                            for i in 0..5 {
                                winner = match winner {
                                    Some(j) => {
                                        if self.compare_cards(&placed_cards[i].0, &placed_cards[j].0) {
                                            Some(j)
                                        } else {
                                            Some(i)
                                        }
                                    }
                                    None => Some(i),
                                };
                            }
                        }
                        let winner = winner.ok_or(Error::Internal("internal error occurred when calculating score"))?;

                        if let FriendFunc::First = friend_func {
                            friend =
                                friend.or_else(|| Some(winner).filter(|_| turn_count == 0 && winner != *president));
                            is_friend_known |= turn_count == 0;
                        }

                        if let FriendFunc::Last = friend_func {
                            friend =
                                friend.or_else(|| Some(winner).filter(|_| turn_count == 9 && winner != *president));
                            is_friend_known |= turn_count == 9;
                        }

                        {
                            let mut score_cards = placed_cards
                                .iter()
                                .filter_map(|(c, _)| if c.is_score() { Some(*c) } else { None })
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
                            if matches!(friend_func, FriendFunc::None) {
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
                            if score == 20 {
                                mul *= 2;
                            }
                            if score <= 10 {
                                mul *= 2;
                            }

                            if score >= pledge {
                                score = mul * (score - 10);
                            } else {
                                score = pledge + score - 20;
                                winner = (1 << 5) - winner;
                            }

                            return Ok(State::GameEnded {
                                winner,
                                president,
                                friend,
                                score,
                                pledge,
                                giruda: *giruda,
                            });
                        }
                    }

                    Ok(State::InGame {
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
                        joker_call_card,
                        joker_call_effect,
                    })
                }
                Command::Random => {
                    let rand_card = deck[user_id].choose(&mut rand::thread_rng()).unwrap();
                    self.next(user_id, Command::Go(*rand_card, Rush::from(*rand_card), false), rule)
                }
                _ => Err(Error::InvalidCommand("BasicCommand::Go")),
            },
            _ => Ok(self.clone()),
        }
    }

    #[cfg(feature = "client")]
    pub fn is_valid_command(&self, user_id: usize, cmd: Command, rule: &Rule) -> Result<()> {
        match self {
            State::Election {
                pledge,
                done,
                curr_user: _,
                start_user,
                deck: _,
                left: _,
            } => match cmd {
                Command::Pledge(x) => {
                    let done = done.clone();
                    let pledge = pledge.clone();

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

                            Ok(())
                        }
                        _ => {
                            if !rule.election.contains(election::Election::PASS_FIRST) && *start_user == None {
                                return Err(Error::PassFirst);
                            }

                            Ok(())
                        }
                    }
                }
                _ => Err(Error::InvalidCommand("Command::Pledge")),
            },
            State::SelectFriend {
                president,
                giruda,
                pledge,
                deck,
            } => match cmd {
                Command::SelectFriend(drop_card, friend_func) => {
                    let mut deck = deck.clone();
                    for card in drop_card.iter() {
                        let idx = deck[user_id].iter().position(|x| *x == *card).ok_or(Error::NotInDeck)?;
                        deck[user_id].remove(idx);
                    }
                    match friend_func {
                        FriendFunc::ByCard(c) => {
                            if !rule.friend.contains(friend::Friend::CARD) {
                                return Err(Error::InvalidFriendFunc);
                            }
                            let temp = deck
                                .iter()
                                .enumerate()
                                .filter(|(_, d)| d.contains(&c))
                                .map(|(i, _)| i)
                                .next();
                            if temp.unwrap() == *president && !rule.friend.contains(friend::Friend::FAKE) {
                                return Err(Error::InvalidFriendFunc);
                            }
                        }
                        FriendFunc::None => {
                            if !rule.friend.contains(friend::Friend::NONE) {
                                return Err(Error::InvalidFriendFunc);
                            }
                        }
                        _ => {}
                    };
                    Ok(())
                }
                Command::ChangePledge(new_giruda) => {
                    if *giruda == new_giruda {
                        return Err(Error::SameGiruda);
                    }

                    let new_pledge = if matches!(giruda, None) {
                        ((*pledge) as i8 - rule.pledge.no_giruda_offset + rule.pledge.change_cost as i8) as u8
                    } else if matches!(new_giruda, None) {
                        ((*pledge) as i8 + rule.pledge.no_giruda_offset + rule.pledge.change_cost as i8) as u8
                    } else {
                        ((*pledge) as i8 + rule.pledge.change_cost as i8) as u8
                    };

                    if new_pledge > rule.pledge.max {
                        return Err(Error::InvalidPledge(true, rule.pledge.max));
                    }

                    Ok(())
                }
                _ => Err(Error::InvalidCommand("Command::Pledge")),
            },
            State::InGame {
                president: _,
                friend_func: _,
                friend: _,
                is_friend_known: _,
                giruda,
                pledge: _,
                deck,
                score_deck: _,
                turn_count,
                placed_cards: _,
                start_user,
                current_user,
                current_pattern,
                joker_call_card,
                joker_call_effect: _,
            } => match cmd {
                Command::Go(card, rush_type, user_joker_call) => {
                    let deck = deck.clone();
                    let turn_count = *turn_count;
                    let start_user = *start_user;
                    let mut current_pattern = *current_pattern;
                    let joker_call_card = *joker_call_card;

                    let joker_calls = vec![
                        if matches!(*giruda, Some(y) if Rush::from(y) == Rush::from(rule.joker_call.cards[0].0)) {
                            rule.joker_call.cards[0].0
                        } else {
                            rule.joker_call.cards[0].1
                        },
                        if matches!(*giruda, Some(y) if Rush::from(y) == Rush::from(rule.joker_call.cards[1].0)) {
                            rule.joker_call.cards[1].0
                        } else {
                            rule.joker_call.cards[1].1
                        },
                    ];

                    if joker_call_card != None
                        && !deck[user_id]
                            .iter()
                            .all(|x| matches!(joker_call_card, Some(y) if y == *x) || card == *x)
                    {
                        return Err(Error::JokerCall);
                    }

                    deck[user_id].iter().position(|x| *x == card).ok_or(Error::NotInDeck)?;
                    if turn_count == 0 || turn_count == 9 {
                        if card == self.get_mighty() && self.check_card_valid(rule.card_policy.mighty)
                            || matches!(rule.card_policy.card.get(&card), Some(y) if self.check_card_valid(*y))
                        {
                            return Err(Error::WrongCard);
                        } else {
                            match card {
                                Card::Normal(t, _) => {
                                    if Some(t) == *giruda && self.check_card_valid(rule.card_policy.giruda) {
                                        return Err(Error::WrongCard);
                                    }
                                    if joker_calls.contains(&card)
                                        && user_joker_call
                                        && self.check_card_valid(rule.card_policy.joker_call)
                                    {
                                        return Err(Error::WrongCard);
                                    }
                                }
                                Card::Joker(_) => {
                                    if self.check_card_valid(rule.card_policy.joker) {
                                        return Err(Error::WrongCard);
                                    }
                                }
                            }
                        }
                    }

                    if *current_user == start_user {
                        current_pattern = Rush::from(card);

                        if !deck[user_id].iter().all(|x| match *x {
                            Card::Normal(t, _) => *x == self.get_mighty() || matches!(giruda, Some(y) if t == *y),
                            Card::Joker(_) => true,
                        }) && matches!(giruda, Some(y) if Rush::from(*y) == current_pattern)
                            && rule.card_policy.giruda.0 == CardPolicy::InvalidForFirst
                        {
                            return Err(Error::WrongCard);
                        }

                        if let Card::Joker(t) = card {
                            current_pattern = rush_type;
                            let containing = match t {
                                Color::Black => Rush::black().contains(current_pattern),
                                Color::Red => Rush::red().contains(current_pattern),
                            };
                            if !containing {
                                return Err(Error::WrongPattern);
                            }
                        }
                    } else if !deck[user_id].iter().all(|x| !current_pattern == Rush::from(*x))
                        && !current_pattern == Rush::from(card)
                    {
                        return Err(Error::WrongCard);
                    }

                    Ok(())
                }
                _ => Err(Error::InvalidCommand("BasicCommand::Go")),
            },
            _ => Ok(()),
        }
    }

    /// Valid users to action next time.
    /// Result is 8-bit integer which contains 0 or 1 for each user.
    /// If all users all valid to action, the result would be `(1 << N) - 1`
    pub fn valid_users(&self, rule: &Rule) -> u8 {
        match self {
            State::Election { curr_user, .. } => {
                if rule.election.contains(election::Election::ORDERED) {
                    1 << *curr_user
                } else {
                    (1 << rule.user_cnt) - 1
                }
            }
            State::SelectFriend { president, .. } => 1 << *president,
            State::InGame { current_user, .. } => 1 << *current_user,
            _ => 0,
        }
    }
} //나중에 테스트 다 하고 바꾸기>

#[cfg(test)]
mod test {
    #[cfg(feature = "server")]
    use {
        super::*,
        crate::prelude::{Command, Error},
        crate::rule::Preset,
    };

    #[cfg(feature = "server")]
    #[test]
    fn next_default_test1() {
        let rule = Rule::from(Preset::Default5);
        let mut state = State::new(&rule);
        if let Err(x) = state.next(0, Command::Pledge(Some((Some(Pattern::Clover), 12))), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::InvalidPledge(false, 13)))
        }
        state = state
            .next(0, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule)
            .unwrap();
        if let Err(x) = state.next(1, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::InvalidPledge(false, 13)))
        }
        state = state
            .next(1, Command::Pledge(Some((Some(Pattern::Clover), 14))), &rule)
            .unwrap();
        if let Err(x) = state.next(1, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::InvalidOrder))
        }
        state = state.next(2, Command::Pledge(None), &rule).unwrap();
        state = state.next(3, Command::Pledge(None), &rule).unwrap();
        state = state.next(4, Command::Pledge(None), &rule).unwrap();
        state = state.next(0, Command::Pledge(None), &rule).unwrap();
        if let State::SelectFriend {
            president,
            giruda,
            pledge,
            deck: _,
        } = state
        {
            assert_eq!(president, 1usize);
            assert_eq!(pledge, 14u8);
            assert_eq!(format!("{:?}", giruda.unwrap()), format!("{:?}", Pattern::Clover));
        }
    }
}
