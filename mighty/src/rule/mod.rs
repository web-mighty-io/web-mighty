pub mod card_policy;
pub mod dealer;
pub mod deck;
pub mod election;
pub mod friend;
pub mod joker_call;
pub mod missed_deal;
pub mod pledge;
pub mod timing;
pub mod visibility;

pub mod prelude {
    pub use crate::rule::card_policy::{CardPolicy, Policy};
    pub use crate::rule::dealer::Dealer;
    pub use crate::rule::deck::{Deck, Preset as DeckPreset};
    pub use crate::rule::election::Election;
    pub use crate::rule::friend::Friend;
    pub use crate::rule::joker_call::JokerCall;
    pub use crate::rule::missed_deal::MissedDeal;
    pub use crate::rule::pledge::Pledge;
    pub use crate::rule::visibility::Visibility;

    pub use crate::rule::{Preset, Rule};
}

use crate::card::{Card, Pattern};
use crate::rule::card_policy::{CardPolicy, Policy};
use crate::rule::dealer::Dealer;
use crate::rule::election::Election;
use crate::rule::friend::Friend;
use crate::rule::joker_call::JokerCall;
use crate::rule::missed_deal::MissedDeal;
use crate::rule::pledge::Pledge;
use crate::rule::timing::Timing;
use crate::rule::visibility::Visibility;
use config::Config;
use serde::{Deserialize, Serialize};

/// Temporary Presets
///
/// After complete implementing server, this goes to database.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Preset {
    // 기본 5마
    Default5,
    // 대구동신과학고등학교
    Ddshs5,
    // 대구과학고등학교 5마
    Dhsh5,
    // 민족사관고등학교 5마
    Kmla5,
    // 광주과학고등학교 5마
    Gsa5,
    // 경기과학고등학교 5마
    Gshs5,
    // 성균관대학교 5마
    Skku5,
    // 서울과학고등학교 5마
    Sshs5,
    // 연세대학교 5마
    Yu5,
}

/// Rule in mighty game
///
/// Can make custom rule for regional mighty rules.
/// Mighty Game is implemented based on this rule.
#[derive(Debug, Clone, Serialize, Deserialize, Config, Hash, Eq, PartialEq)]
pub struct Rule {
    pub user_cnt: u8,
    pub card_cnt_per_user: u8,
    pub deck: Vec<Card>,
    pub missed_deal: MissedDeal,
    pub election: Election,
    pub pledge: Pledge,
    pub friend: Friend,
    pub friend_cnt: u8,
    pub card_policy: Policy,
    pub joker_call: JokerCall,
    pub pattern_order: Vec<Pattern>,
    pub visibility: Visibility,
    pub next_dealer: Dealer,
    pub timing: Timing,
}

impl From<Preset> for Rule {
    fn from(p: Preset) -> Self {
        match p {
            Preset::Default5 => Rule::new(),
            Preset::Ddshs5 => Rule::new()
                .set_election(Election::all() - Election::NO_GIRUDA_EXIST)
                .map_pledge(|p| p.set_change_cost(1))
                .set_friend(Friend::CARD | Friend::FAKE | Friend::NONE)
                .map_joker_call(|j| {
                    j.set_cards(vec![(
                        Card::Normal(Pattern::Clover, 2),
                        Card::Normal(Pattern::Clover, 2),
                    )])
                }),
            Preset::Dhsh5 => Rule::new()
                .map_pledge(|p| p.set_min(12).set_max(23))
                .set_election(Election::all() - Election::PASS_FIRST)
                .map_card_policy(|p| p.set_mighty((CardPolicy::NoEffect, CardPolicy::Valid))),
            Preset::Kmla5 => Rule::new()
                .map_missed_deal(|m| m.set_score(1).set_joker(-1).set_limit(1))
                .map_joker_call(|j| j.set_mighty_defense(false)),
            Preset::Gsa5 => Rule::new().map_pledge(|p| p.set_min(12)).map_card_policy(|p| {
                p.set_mighty((CardPolicy::NoEffect, CardPolicy::Valid))
                    .set_joker((CardPolicy::Valid, CardPolicy::Valid))
            }),
            Preset::Gshs5 => Rule::new()
                .set_deck(deck::Preset::FullDeck.to_vec())
                .set_election(Election::NO_GIRUDA_EXIST | Election::PASS_FIRST)
                .map_missed_deal(|m| {
                    m.set_score(2)
                        .set_joker(-1)
                        .mut_card(|m| {
                            m.insert(Card::Normal(Pattern::Spade, 0), -2);
                        })
                        .set_limit(1)
                })
                .map_pledge(|p| p.set_min(14))
                .map_joker_call(|j| {
                    j.mut_cards(|v| {
                        v.push((Card::Normal(Pattern::Heart, 2), Card::Normal(Pattern::Diamond, 2)));
                    })
                }),
            // implement friend known time
            Preset::Skku5 => Rule::new()
                .map_pledge(|p| p.set_min(12).set_change_cost(0))
                .map_card_policy(|p| {
                    p.set_joker((CardPolicy::Valid, CardPolicy::Valid))
                        .set_giruda((CardPolicy::Valid, CardPolicy::Valid))
                })
                .map_joker_call(|j| j.set_has_power(true)),
            Preset::Sshs5 => Rule::new()
                .map_missed_deal(|m| {
                    m.set_score(2)
                        .set_joker(-1)
                        .mut_card(|m| {
                            m.insert(Card::Normal(Pattern::Spade, 10), 1);
                            m.insert(Card::Normal(Pattern::Diamond, 10), 1);
                            m.insert(Card::Normal(Pattern::Heart, 10), 1);
                            m.insert(Card::Normal(Pattern::Clover, 10), 1);
                            m.insert(Card::Normal(Pattern::Spade, 0), 1);
                        })
                        .set_limit(1)
                })
                .set_friend(Friend::all() - Friend::PICK)
                .map_card_policy(|p| p.set_joker_call((CardPolicy::NoEffect, CardPolicy::Valid))),
            Preset::Yu5 => Rule::new()
                .map_missed_deal(|m| {
                    m.set_score(2)
                        .mut_card(|m| {
                            m.insert(Card::Normal(Pattern::Spade, 10), 1);
                            m.insert(Card::Normal(Pattern::Heart, 10), 1);
                            m.insert(Card::Normal(Pattern::Spade, 0), 1);
                        })
                        .set_limit(1)
                })
                .set_election(Election::INCREASING | Election::ORDERED)
                .map_pledge(|p| p.set_min(14).set_max(23))
                .map_card_policy(|p| p.set_joker_call((CardPolicy::NoEffect, CardPolicy::Valid))),
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            user_cnt: 5,
            card_cnt_per_user: 10,
            deck: deck::Preset::SingleJoker.to_vec(),
            missed_deal: MissedDeal::new(),
            election: Election::all(),
            pledge: Pledge::new(),
            friend: Friend::all(),
            friend_cnt: 1,
            card_policy: Policy::new(),
            joker_call: JokerCall::new(),
            pattern_order: vec![Pattern::Spade, Pattern::Diamond, Pattern::Heart, Pattern::Clover],
            visibility: Visibility::FRIEND,
            next_dealer: Dealer::Friend,
            timing: Timing::new(),
        }
    }

    pub fn valid(&self) -> bool {
        self.user_cnt > 0
            && self.user_cnt <= 8
            && self.card_cnt_per_user > 0
            && self.user_cnt * self.card_cnt_per_user <= self.deck.len() as u8
            && self.pledge.valid()
            && self.deck.iter().filter(|c| c.is_joker()).count() == self.joker_call.len()
            && {
                let mut v = self.pattern_order.clone();
                v.sort();
                v == vec![Pattern::Spade, Pattern::Diamond, Pattern::Heart, Pattern::Clover]
            }
    }
}
