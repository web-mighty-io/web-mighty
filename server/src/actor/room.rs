use crate::actor::server::{MakeGameId, RemoveRoom};
use crate::actor::{list_ss, observe_ss, server, user, GameId, RoomId, UserNo};
use crate::db;
use actix::prelude::*;
use deadpool_postgres::Pool;
use mighty::rule::Rule;
use mighty::Game;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct Room {
    id: RoomId,
    name: String,
    rule: Rule,
    is_rank: bool,
    game_id: GameId,
    game_no: u32,
    game: Option<Game>,
    head: UserNo,
    user: Vec<UserNo>,
    user_addr: HashMap<UserNo, Addr<user::User>>,
    observe_addr: HashSet<Addr<observe_ss::ObserveSession>>,
    list_addr: HashSet<Addr<list_ss::ListSession>>,
    server: Addr<server::Server>,
    pool: Pool,
}

impl Actor for Room {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "bool")]
pub enum Join {
    User(UserNo, Addr<user::User>),
    Observe(Addr<observe_ss::ObserveSession>),
    List(Addr<list_ss::ListSession>),
}

impl Handler<Join> for Room {
    type Result = bool;

    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        match msg {
            Join::User(user_id, addr) => {
                if self.user.len() < self.rule.user_cnt as usize {
                    self.user.push(user_id);
                    self.user_addr.insert(user_id, addr);
                    true
                } else {
                    false
                }
            }
            Join::Observe(addr) => {
                self.observe_addr.insert(addr);
                true
            }
            Join::List(addr) => {
                self.list_addr.insert(addr);
                true
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum Leave {
    User(UserNo),
    Observe(Addr<observe_ss::ObserveSession>),
    List(Addr<list_ss::ListSession>),
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Leave::User(user_id) => {
                if self.game.is_some() {
                    return;
                }

                if self.user_addr.remove(&user_id).is_some() {
                    let mut idx = 0;
                    for (i, v) in self.user.iter().enumerate() {
                        if *v == user_id {
                            idx = i;
                            break;
                        }
                    }
                    self.user.remove(idx);

                    if user_id == self.head {
                        if let Some(id) = self.user.first() {
                            self.head = *id;
                        }
                    }
                    self.try_remove(ctx);
                }
            }
            Leave::Observe(addr) => {
                self.observe_addr.remove(&addr);
                self.try_remove(ctx);
            }
            Leave::List(addr) => {
                self.list_addr.remove(&addr);
                self.try_remove(ctx);
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeName(UserNo, String);

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
pub struct StartGame(UserNo);

impl Handler<StartGame> for Room {
    type Result = ();

    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        if self.game.is_none() && msg.0 == self.head {
            self.server
                .send(MakeGameId)
                .into_actor(self)
                .then(|res, act, ctx| {
                    if let Ok(res) = res {
                        act.game = Some(Game::new(act.rule.clone()));
                        db::game::make_game(
                            db::game::SaveGameForm {
                                game_id: res.0,
                                room_id: act.id.0,
                                room_name: act.name.clone(),
                                users: act.user.iter().map(|u| u.0).collect(),
                                is_rank: act.is_rank,
                                rule: act.rule.clone(),
                            },
                            act.pool.clone(),
                        )
                        .into_actor(act)
                        .then(|_, _, _| fut::ready(()))
                        .wait(ctx);
                    }

                    fut::ready(())
                })
                .wait(ctx);
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "mighty::Result<()>")]
pub struct Go(UserNo, mighty::Command);

impl Handler<Go> for Room {
    type Result = mighty::Result<()>;

    fn handle(&mut self, msg: Go, _: &mut Self::Context) -> Self::Result {
        if let Some(user_id) = self
            .user
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x == msg.0 { Some(i) } else { None })
            .next()
        {
            if self.next(user_id, msg.1)? {
                self.game = None;
            }
            Ok(())
        } else {
            Err(mighty::Error::InvalidUser)
        }
    }
}

impl Room {
    pub fn new(id: RoomId, name: String, rule: Rule, server: Addr<server::Server>, pool: Pool) -> Room {
        Room {
            id,
            name,
            rule,
            is_rank: false,
            game_id: GameId(Uuid::default()),
            game_no: 0,
            game: None,
            head: UserNo(0),
            user: Vec::new(),
            user_addr: HashMap::new(),
            observe_addr: HashSet::new(),
            list_addr: HashSet::new(),
            server,
            pool,
        }
    }

    fn try_remove(&mut self, ctx: &mut Context<Self>) {
        if self.user_addr.is_empty() && self.observe_addr.is_empty() && self.list_addr.is_empty() {
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

    async fn save_state(&self) -> db::Result<()> {
        if let Some(game) = &self.game {
            db::game::save_state(
                db::game::SaveStateForm {
                    game_id: self.game_id.0,
                    room_id: self.id.0,
                    number: self.game_no,
                    state: game.get_state(),
                },
                self.pool.clone(),
            )
            .await
        } else {
            Ok(())
        }
    }

    fn next(&mut self, user_id: usize, cmd: mighty::Command) -> mighty::Result<bool> {
        if let Some(game) = &mut self.game {
            game.next(user_id, cmd)
        } else {
            Err(mighty::Error::Internal("Game not started"))
        }
    }
}
