use crate::user::User;
use crate::user::Card;

pub enum GameState {
    Election,
    InGame(i32, i32), // Ingame(round starter, current player)
    Finished(i32), // Finished(winner uid)
}

pub enum FriendCondition {
    Card(Vec<String>), // Specific card set
    Person(u64), // Uid of a specific person
}

pub enum Friend {
    Known(User),
    Unknown(FriendCondition),
    None,
}

pub struct InGameUser {
    pub uid: u64,
    pub name: String,
    in_hand: Vec<Card>,
    gained: Vec<Card>,
    pub score: i32,
}

impl InGameUser {
    fn from(user: &User) -> InGameUser {
        InGameUser {
            uid: user.uid,
            name: user.name.clone(),
            in_hand: Vec::new(),
            gained: Vec::new(),
            score: 0,
        }
    }
}

pub struct Game {
    users: Vec<InGameUser>,
    pub state: GameState,
    pub friend : Friend,
}

impl Game {
    pub fn new() -> Game {
        Game{
            users: Vec::new(),
            state: GameState::Election,
            friend: Friend::None,
        }
    }

    pub fn add_user(&mut self, user: &User) {
        self.users.push(InGameUser::from(user));
    }

    pub fn delete_user(&mut self, user: &User) {
        self.users.retain(|x| x.uid != user.uid);
    }

    pub fn place_card(&mut self, user: &User, card: &Card) {
        // TODO
    }
}