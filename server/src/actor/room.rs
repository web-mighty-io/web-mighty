use crate::actor::server::{MakeGameId, RemoveRoom};
use crate::actor::{observe_ss, server, user, GameId, RoomId, UserId};
use actix::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Game {
    id: GameId,
    game: mighty::Game,
}

impl Game {
    pub fn new(id: GameId) -> Game {
        Game {
            id,
            game: mighty::Game::new(),
        }
    }

    pub fn save(&self) {
        // todo
    }
}

pub struct Room {
    id: RoomId,
    name: String,
    game: Option<Game>,
    head: UserId,
    user: Vec<UserId>,
    user_addr: HashMap<UserId, Addr<user::User>>,
    observe_addr: HashSet<Addr<observe_ss::ObserveSession>>,
    server: Addr<server::Server>,
}

impl Actor for Room {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "bool")]
pub enum Join {
    User(UserId, Addr<user::User>),
    Observe(Addr<observe_ss::ObserveSession>),
}

impl Handler<Join> for Room {
    type Result = bool;

    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        match msg {
            Join::User(user_id, addr) => {
                for i in self.user.iter_mut() {
                    if i.0 == 0 {
                        *i = user_id;
                        self.user_addr.insert(user_id, addr);
                        return true;
                    }
                }

                false
            }
            Join::Observe(addr) => {
                self.observe_addr.insert(addr);
                true
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum Leave {
    User(UserId),
    Observe(Addr<observe_ss::ObserveSession>),
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Leave::User(user_id) => {
                if self.game.is_some() {
                    return;
                }

                self.user_addr.remove(&user_id);

                for i in self.user.iter_mut() {
                    if *i == user_id {
                        i.0 = 0;
                    }
                }

                if self.user_addr.is_empty() {
                    self.remove(ctx);
                    return;
                }

                if user_id == self.head {
                    self.set_head();
                }
            }
            Leave::Observe(addr) => {
                self.observe_addr.remove(&addr);
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeName(UserId, String);

impl Handler<ChangeName> for Room {
    type Result = ();

    fn handle(&mut self, msg: ChangeName, _: &mut Self::Context) -> Self::Result {
        if msg.0 == self.head {
            self.name = msg.1;
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct StartGame(UserId);

impl Handler<StartGame> for Room {
    type Result = ();

    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        if self.game.is_none() && msg.0 == self.head {
            self.server
                .send(MakeGameId)
                .into_actor(self)
                .then(|res, act, ctx| {
                    if let Ok(res) = res {
                        act.game = Some(Game::new(res));
                    }

                    fut::ready(())
                })
                .wait(ctx);
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "mighty::Result<()>")]
pub struct Go(UserId, mighty::Command);

impl Handler<Go> for Room {
    type Result = mighty::Result<()>;

    fn handle(&mut self, msg: Go, _: &mut Self::Context) -> Self::Result {
        if let Some(game) = &mut self.game {
            if let Some(user_id) = self
                .user
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == msg.0 { Some(i) } else { None })
                .next()
            {
                game.game.next(user_id, msg.1).map(|finished| {
                    if finished {
                        // todo
                    }
                })
            } else {
                Err(mighty::Error::InvalidUser(0))
            }
        } else {
            Err(mighty::Error::Internal("Game not started"))
        }
    }
}

impl Room {
    pub fn new(id: RoomId, server: Addr<server::Server>) -> Room {
        Room {
            id,
            name: "".to_string(),
            game: None,
            head: UserId(0),
            user: vec![UserId(0); 5],
            user_addr: HashMap::new(),
            observe_addr: HashSet::new(),
            server,
        }
    }

    fn set_head(&mut self) {
        for i in self.user.iter() {
            if i.0 != 0 {
                self.head = *i;
            }
        }
    }

    fn remove(&mut self, ctx: &mut Context<Self>) {
        self.server
            .send(RemoveRoom(self.id))
            .into_actor(self)
            .then(|_, _, ctx| {
                ctx.stop();
                fut::ready(())
            })
            .wait(ctx);
    }
}
