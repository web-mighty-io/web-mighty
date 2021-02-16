use crate::card::{Card, Pattern, Rush};
use crate::rule::{card_policy::CardPolicy, election, Rule};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use {
    crate::card::Color,
    crate::command::Command,
    crate::error::{Error, Result},
    crate::rule::friend,
    rand::seq::SliceRandom,
    std::cmp::Ordering,
};
/*#[cfg(any(feature = "client", feature = "server"))]
use {
    crate::card::Color,
    crate::command::Command,
    crate::error::{Error, Result},
    crate::rule::friend,
};*/

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
            let mut deck = rule.deck.0.clone();
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

    /*
    #[cfg(feature = "server")]
    fn is_joker_called(&self) -> bool {
        if let State::InGame { joker_call_card, .. } = self {
            *joker_call_card != None
        } else {
            false
        }
    }
    */
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

    #[cfg(feature = "server")]
    //#[cfg(any(feature = "client", feature = "server"))]
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

    #[cfg(feature = "server")]
    //#[cfg(any(feature = "client", feature = "server"))]
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
    fn minor_card_winner(&self, rule: &Rule, card_vec: &[Card]) -> Card {
        // no mighty, giruda, joker
        // only normal card
        let cur_pat = self.get_current_pattern();

        let mut it = card_vec
            .iter()
            .filter_map(|i| if let Card::Normal(p, n) = i { Some((p, n)) } else { None })
            .filter(|(p, _)| cur_pat.contains(Rush::from(**p)));

        let fold_fn = |(p_max, n_max): (Pattern, u8), (p, n): (&Pattern, &u8)| match n_max.cmp(n) {
            Ordering::Less => (*p, *n),
            Ordering::Equal
                if rule.pattern_order.iter().fold(0, |mut x, r| {
                    if x == 0 {
                        if *r == *p {
                            x = 1;
                        } else if *r == p_max {
                            x = 2;
                        }
                    }
                    x
                }) == 1 =>
            {
                (*p, *n)
            }
            _ => (p_max, n_max),
        };

        let (p, n) = if let Some((p, n)) = it.next() {
            it.fold((*p, *n), fold_fn)
        } else {
            card_vec
                .iter()
                .filter_map(|i| if let Card::Normal(p, n) = i { Some((p, n)) } else { None })
                .fold((Pattern::Spade, 0), fold_fn)
        };

        Card::Normal(p, n)
    }

    #[cfg(feature = "server")]
    fn get_max_card_with_pattern(card_vec: &[Card], pat: Pattern) -> Option<Card> {
        card_vec
            .iter()
            .filter_map(|c| match c {
                Card::Normal(p, n) if *p == pat => Some(n),
                _ => None,
            })
            .max()
            .map(|i| Card::Normal(pat, *i))
    }

    #[cfg(feature = "server")]
    fn calculate_winner(&self, rule: &Rule, card_vec: &[Card]) -> Card {
        let mighty = self.get_mighty();

        if card_vec.contains(&mighty) {
            return mighty;
        }

        if rule.deck.1 == 0b01 || rule.deck.1 == 0b10 {
            // one types of joker
            let joker = if rule.deck.1 == 0b01 {
                Card::Joker(Color::Red)
            } else {
                Card::Joker(Color::Black)
            };

            if card_vec.contains(&joker) {
                return joker;
            }
            //no more joker

            let giruda = self.get_giruda();
            if let Some(giruda) = giruda {
                State::get_max_card_with_pattern(card_vec, giruda)
                    .unwrap_or_else(|| self.minor_card_winner(rule, card_vec))
            } else {
                //노기루
                //마이티, 기루다, 조커 무시하고 일반 카드들간의 경쟁
                self.minor_card_winner(&rule, card_vec)
            }
        } else {
            //two types of joker

            if let Some(giruda) = self.get_giruda() {
                //giruda & two joker
                let (joker1, joker2) = if Pattern::Spade == giruda || Pattern::Clover == giruda {
                    (Card::Joker(Color::Black), Card::Joker(Color::Red))
                } else {
                    (Card::Joker(Color::Red), Card::Joker(Color::Black))
                };

                if card_vec.contains(&joker1) {
                    return joker1;
                }

                //no more joker1
                let max_card = State::get_max_card_with_pattern(card_vec, giruda);

                if let Some(max_card) = max_card {
                    max_card
                } else {
                    //기루다가 없다
                    //현재상황 = 마이티X조커1X기루다X
                    // 조커2와 먼저돌린 문양의 경쟁

                    let cur_pat = self.get_current_pattern();
                    if Color::from(cur_pat) != Color::from(giruda) && card_vec.contains(&joker2) {
                        joker2
                    } else {
                        self.minor_card_winner(&rule, card_vec)
                    }
                }
            } else {
                //노기루 & two joker
                //rush에 해당하는 조커가 가장 강력함.
                let cur_pat = self.get_current_pattern();
                let joker1 = if Rush::black().contains(cur_pat) {
                    Card::Joker(Color::Black)
                } else {
                    Card::Joker(Color::Red)
                };

                if card_vec.contains(&joker1) {
                    return joker1;
                }

                self.minor_card_winner(&rule, &card_vec)
            }
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
                    if drop_card.len() != deck[user_id].len() - deck[(user_id + 1) % (rule.user_cnt as usize)].len() {
                        return Err(Error::DropCard);
                    }
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
                        score_deck: vec![Vec::new(); rule.user_cnt as usize],
                        turn_count: 0,
                        placed_cards: vec![
                            (Card::Normal(Pattern::Spade, 0), CardPolicy::Valid);
                            rule.user_cnt as usize
                        ],
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
                            .choose_multiple(
                                &mut rand::thread_rng(),
                                deck[user_id].len() - deck[(user_id + 1) % (rule.user_cnt as usize)].len(),
                            )
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
                    for cards in &rule.joker_call.cards {
                        joker_calls.push(if matches!(*giruda, Some(y) if Rush::from(y) == Rush::from(cards.0)) {
                            cards.1
                        } else {
                            cards.0
                        });
                    }
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
                        let mut card_vec = Vec::new();

                        for i in 0..(rule.user_cnt as usize) {
                            let (c, p) = placed_cards[(i + start_user) % (rule.user_cnt as usize)];

                            if p == CardPolicy::NoEffect {
                                continue;
                            }
                            card_vec.push(c);
                        }
                        if card_vec.is_empty() {
                            for i in 0..(rule.user_cnt as usize) {
                                let (c, _) = placed_cards[(i + start_user) % (rule.user_cnt as usize)];
                                card_vec.push(c);
                            }
                        }
                        // winner is fucked
                        let winner = self.calculate_winner(&rule, &card_vec);

                        // instance winner
                        // todo
                        let winner = card_vec.iter().position(|c| *c == winner).unwrap();

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

                            let mut score = score_deck.iter().map(|x| x.len() as u8).sum();
                            let mut winner = 1 << president;
                            if let Some(f) = friend {
                                score -= score_deck[f].len() as u8;
                                winner += 1 << f;
                            }
                            score = 20 - score + score_deck[president].len() as u8;
                            if score == 20 {
                                mul *= 2;
                            }

                            if score >= pledge {
                                score = mul * (score - 10);
                            } else {
                                score = if score <= 10 {
                                    2 * (pledge - score)
                                } else {
                                    pledge - score
                                };
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

    /*
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
    }*/

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
}

/*
#[cfg(test)]
mod test {
    #[cfg(feature = "server")]
    use {super::*, crate::prelude::Command, crate::rule::Preset, rand::prelude::IteratorRandom};

    #[cfg(feature = "server")]
    #[test]
    fn compare_cards_test_clover() {
        let mut new_deck: Vec<Vec<Card>> = Vec::new();

        let mut dec1: Vec<Card> = Vec::new();
        let mut dec2: Vec<Card> = Vec::new();
        let mut dec3: Vec<Card> = Vec::new();
        let mut dec4: Vec<Card> = Vec::new();
        let mut dec5: Vec<Card> = Vec::new();
        let mut trash: Vec<Card> = Vec::new();

        for i in 0..10 {
            dec2.push(Card::Normal(Pattern::Spade, i));
            dec3.push(Card::Normal(Pattern::Clover, i));
            dec4.push(Card::Normal(Pattern::Heart, i));
            dec5.push(Card::Normal(Pattern::Diamond, i));
        }
        for i in 10..12 {
            dec1.push(Card::Normal(Pattern::Spade, i));
            dec1.push(Card::Normal(Pattern::Clover, i));
            dec1.push(Card::Normal(Pattern::Heart, i));
            dec1.push(Card::Normal(Pattern::Diamond, i));
        }
        dec1.push(Card::Normal(Pattern::Spade, 12));
        dec1.push(Card::Joker(Color::Black));
        trash.push(Card::Normal(Pattern::Clover, 12));
        trash.push(Card::Normal(Pattern::Heart, 12));
        trash.push(Card::Normal(Pattern::Diamond, 12));
        new_deck.push(dec1);
        new_deck.push(dec2);
        new_deck.push(dec3);
        new_deck.push(dec4);
        new_deck.push(dec5);

        let rule = Rule::from(Preset::Default5);

        let mut state = State::new(&rule);

        if let State::Election { deck, left, .. } = &mut state {
            *deck = new_deck;
            *left = trash;
        }

        state = state
            .next(0, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule)
            .unwrap();

        //pre-test
        assert_eq!(state.is_joker_called(), false);
        assert_eq!(state.get_current_pattern(), Rush::SPADE);
        assert_eq!(state.get_giruda(), None);
        assert_eq!(state.check_card_valid(rule.card_policy.mighty), false);
        assert_eq!(state.check_card_effect(rule.card_policy.mighty), false);

        state = state.next(1, Command::Pledge(None), &rule).unwrap();
        state = state.next(2, Command::Pledge(None), &rule).unwrap();
        state = state.next(3, Command::Pledge(None), &rule).unwrap();
        state = state.next(4, Command::Pledge(None), &rule).unwrap();

        let mut drop_card = Vec::new();
        if let State::SelectFriend { president, deck, .. } = state.clone() {
            drop_card = deck[president]
                .choose_multiple(&mut rand::thread_rng(), 3)
                .cloned()
                .collect();
        }
        state = state
            .next(0, Command::SelectFriend(drop_card, FriendFunc::ByUser(1)), &rule)
            .unwrap();
        assert!(state.compare_cards(&Card::Normal(Pattern::Clover, 0), &Card::Normal(Pattern::Spade, 0)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Clover, 1), &Card::Normal(Pattern::Clover, 0)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Clover, 1), &Card::Normal(Pattern::Clover, 2)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Heart, 2), &Card::Normal(Pattern::Clover, 1)));

        if let State::InGame { deck, current_user, .. } = state.clone() {
            let card = deck[current_user]
                .iter()
                .filter(|c| matches!(c, Card::Normal(Pattern::Diamond, _)))
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();
            state = state
                .next(current_user, Command::Go(card, Rush::empty(), false), &rule)
                .unwrap();
        }

        assert!(state.compare_cards(&Card::Normal(Pattern::Spade, 12), &Card::Normal(Pattern::Diamond, 3)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Diamond, 5), &Card::Normal(Pattern::Clover, 3)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Clover, 5) , &Card::Joker(Color::Black)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Spade, 5) , &Card::Joker(Color::Black)));
        //in-game test
        assert_eq!(state.is_joker_called(), false);
        assert_eq!(state.get_current_pattern(), Rush::DIAMOND);
        assert_eq!(state.get_giruda().unwrap(), Pattern::Clover);
    }

    #[cfg(feature = "server")]
    #[test]
    fn compare_cards_test_spade() {
        let rule = Rule::from(Preset::Default5);
        let mut state = State::new(&rule);
        state = state
            .next(0, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule)
            .unwrap();
        state = state
            .next(1, Command::Pledge(Some((Some(Pattern::Spade), 14))), &rule)
            .unwrap();
        state = state.next(2, Command::Pledge(None), &rule).unwrap();
        state = state.next(3, Command::Pledge(None), &rule).unwrap();
        state = state.next(4, Command::Pledge(None), &rule).unwrap();
        state = state.next(0, Command::Pledge(None), &rule).unwrap();
        let mut drop_card = Vec::new();
        if let State::SelectFriend { president, deck, .. } = state.clone() {
            drop_card = deck[president]
                .choose_multiple(&mut rand::thread_rng(), 3)
                .cloned()
                .collect();
        }

        state = state
            .next(1, Command::SelectFriend(drop_card, FriendFunc::ByUser(0)), &rule)
            .unwrap();
        assert!(state.compare_cards(&Card::Normal(Pattern::Spade, 0), &Card::Normal(Pattern::Diamond, 0)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Spade, 1), &Card::Normal(Pattern::Spade, 0)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Spade, 1), &Card::Normal(Pattern::Spade, 5)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Heart, 10), &Card::Normal(Pattern::Spade, 5)));
    }

    #[cfg(feature = "server")]
    #[test]
    fn joker_call_test_spade() {
        //joker call with spade giruda

        let mut new_deck: Vec<Vec<Card>> = Vec::new();

        let mut dec1: Vec<Card> = Vec::new();
        let mut dec2: Vec<Card> = Vec::new();
        let mut dec3: Vec<Card> = Vec::new();
        let mut dec4: Vec<Card> = Vec::new();
        let mut dec5: Vec<Card> = Vec::new();
        let mut trash: Vec<Card> = Vec::new();

        for i in 0..10 {
            dec2.push(Card::Normal(Pattern::Spade, i));
            dec3.push(Card::Normal(Pattern::Clover, i));
            dec4.push(Card::Normal(Pattern::Heart, i));
            dec5.push(Card::Normal(Pattern::Diamond, i));
        }
        for i in 10..12 {
            dec1.push(Card::Normal(Pattern::Spade, i));
            dec1.push(Card::Normal(Pattern::Clover, i));
            dec1.push(Card::Normal(Pattern::Heart, i));
            dec1.push(Card::Normal(Pattern::Diamond, i));
        }
        dec1.push(Card::Normal(Pattern::Spade, 12));
        dec1.push(Card::Joker(Color::Black));
        trash.push(Card::Normal(Pattern::Clover, 12));
        trash.push(Card::Normal(Pattern::Heart, 12));
        trash.push(Card::Normal(Pattern::Diamond, 12));
        new_deck.push(dec1);
        new_deck.push(dec2);
        new_deck.push(dec3);
        new_deck.push(dec4);
        new_deck.push(dec5);

        let rule = Rule::from(Preset::Default5);

        let mut state = State::new(&rule);

        if let State::Election { deck, left, .. } = &mut state {
            *deck = new_deck;
            *left = trash.clone();
        }

        state = state.next(0, Command::Pledge(None), &rule).unwrap();
        state = state.next(1, Command::Pledge(None), &rule).unwrap();
        state = state
            .next(2, Command::Pledge(Some((Some(Pattern::Spade), 13))), &rule)
            .unwrap();
        state = state.next(3, Command::Pledge(None), &rule).unwrap();
        state = state.next(4, Command::Pledge(None), &rule).unwrap();

        let mut drop_card = Vec::new();
        if let State::SelectFriend { .. } = state {
            drop_card = trash;
        }

        state = state
            .next(2, Command::SelectFriend(drop_card, FriendFunc::ByUser(1)), &rule)
            .unwrap();

        if let State::InGame { current_user, .. } = state {
            let card = Card::Normal(Pattern::Clover, 2);
            state = state
                .next(current_user, Command::Go(card, Rush::empty(), true), &rule)
                .unwrap();
        }

        //compare_card_test
        assert!(state.compare_cards(&Card::Normal(Pattern::Diamond, 12), &Card::Normal(Pattern::Spade, 3)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Heart, 5), &Card::Normal(Pattern::Clover, 3)));
        assert!(state.compare_cards(&Card::Normal(Pattern::Diamond, 5), &Card::Normal(Pattern::Heart, 6)));
        //joker_call_test
        assert_eq!(state.get_giruda().unwrap(), Pattern::Spade);
        assert_eq!(state.get_current_pattern(), Rush::CLOVER);
        assert_eq!(state.is_joker_called(), true);
        assert!(state.compare_cards(&Card::Joker(Color::Black) , &Card::Normal(Pattern::Spade, 2)));
        assert!(state.compare_cards(&Card::Joker(Color::Red) , &Card::Joker(Color::Black)));
    }

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
        let mut drop_card = Vec::new();
        if let State::SelectFriend {
            president,
            pledge,
            giruda,
            deck,
            ..
        } = state.clone()
        {
            assert_eq!(president, 1usize);
            assert_eq!(pledge, 14u8);
            assert_eq!(format!("{:?}", giruda.unwrap()), format!("{:?}", Pattern::Clover));
            drop_card = deck[president]
                .choose_multiple(&mut rand::thread_rng(), 3)
                .cloned()
                .collect();
        }
        if let Err(x) = state.next(1, Command::ChangePledge(Some(Pattern::Clover)), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::SameGiruda))
        }
        state = state
            .next(1, Command::ChangePledge(Some(Pattern::Spade)), &rule)
            .unwrap();
        state = state
            .next(1, Command::SelectFriend(drop_card, FriendFunc::ByUser(2)), &rule)
            .unwrap();
        for i in 0..50 {
            if let State::InGame {
                giruda,
                deck,
                current_user,
                ..
            } = state.clone()
            {
                let card = deck[current_user]
                    .iter()
                    .filter(|c| match c {
                        Card::Normal(t, _) => Some(*t) != giruda || i >= 5,
                        _ => true,
                    })
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .unwrap();
                state = state
                    .next(current_user, Command::Go(card, Rush::empty(), false), &rule)
                    .unwrap();
            }
        }
        if let State::GameEnded {
            winner,
            president,
            friend,
            ..
        } = state
        {
            assert!(winner == 6 || winner == 26);
            assert_eq!(president, 1);
            assert_eq!(friend, Some(2));
        }
    }

    #[cfg(feature = "server")]
    #[test]
    fn next_gshs5_test1() {
        let rule = Rule::from(Preset::Gshs5);
        let mut state = State::new(&rule);
        if let Err(x) = state.next(0, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::InvalidPledge(false, 14)))
        }
        state = state.next(0, Command::Pledge(None), &rule).unwrap();
        state = state
            .next(3, Command::Pledge(Some((Some(Pattern::Clover), 14))), &rule)
            .unwrap();
        if let Err(x) = state.next(2, Command::Pledge(Some((Some(Pattern::Clover), 13))), &rule) {
            assert_eq!(format!("{}", x), format!("{}", Error::InvalidPledge(false, 13)))
        }
        state = state
            .next(2, Command::Pledge(Some((Some(Pattern::Spade), 14))), &rule)
            .unwrap();
        state = state
            .next(2, Command::Pledge(Some((Some(Pattern::Clover), 14))), &rule)
            .unwrap();
        state = state.next(1, Command::Pledge(None), &rule).unwrap();
        state = state.next(4, Command::Pledge(None), &rule).unwrap();
        state = state.next(3, Command::Pledge(None), &rule).unwrap();
        state = state.next(2, Command::Pledge(None), &rule).unwrap();
        if let State::SelectFriend {
            president,
            giruda,
            deck,
            ..
        } = state.clone()
        {
            let drop_card = deck[president]
                .choose_multiple(&mut rand::thread_rng(), 4)
                .cloned()
                .collect();
            if let Err(x) = state.next(president, Command::ChangePledge(giruda), &rule) {
                assert_eq!(format!("{}", x), format!("{}", Error::SameGiruda))
            }
            state = state
                .next(
                    president,
                    Command::SelectFriend(drop_card, FriendFunc::ByUser(0)),
                    &rule,
                )
                .unwrap();
            for i in 0..50 {
                if let State::InGame {
                    giruda,
                    deck,
                    current_user,
                    ..
                } = state.clone()
                {
                    let card = deck[current_user]
                        .iter()
                        .filter(|c| match c {
                            Card::Normal(t, _) => Some(*t) != giruda || i >= 5,
                            _ => true,
                        })
                        .choose(&mut rand::thread_rng())
                        .cloned()
                        .unwrap();
                    state = state
                        .next(current_user, Command::Go(card, Rush::empty(), false), &rule)
                        .unwrap();
                }
            }
        }
    }
    // not random and real data test should be applied
}
*/
