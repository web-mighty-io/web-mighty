use crate::actor::db::SaveStateForm;
use crate::actor::hub::{MakeGameId, RemoveRoom};
use crate::actor::{hub, Database, GameId, Hub, List, Observe, RoomId, User, UserNo};
use crate::dev::*;
use actix::prelude::*;
use mighty::rule::Rule;
use mighty::{Command, Game};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Message, MessageResponse, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct RoomInfo {
    id: RoomId,
    name: String,
    rule: Rule,
    is_rank: bool,
    head: UserNo,
    user: Vec<UserNo>,
    observer_cnt: usize,
    is_game: bool,
}

#[derive(Clone)]
pub struct GameInfo {
    id: GameId,
    no: u32,
    game: Game,
}

pub struct Room {
    info: RoomInfo,
    game: Option<GameInfo>,
    user_addr: HashMap<UserNo, Addr<User>>,
    observe: Addr<Group<Session<Observe>>>,
    list: Addr<Group<Session<List>>>,
    hub: Addr<Hub>,
    db: Addr<Database>,
}

impl Actor for Room {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum RoomJoin {
    User(UserNo, Addr<User>),
    Observe(Addr<Session<Observe>>),
    List(Addr<Session<List>>),
}

impl Handler<RoomJoin> for Room {
    type Result = Result<()>;

    fn handle(&mut self, msg: RoomJoin, _: &mut Self::Context) -> Self::Result {
        match msg {
            RoomJoin::User(user_id, addr) => {
                let mut is_full = true;
                for i in self.info.user.iter_mut() {
                    if i.0 == 0 {
                        *i = user_id;
                        is_full = false;
                    }
                }
                ensure!(!is_full, StatusCode::BAD_REQUEST, "room is full");
                self.user_addr.insert(user_id, addr);
                self.set_head();
                self.spread_info();
            }
            RoomJoin::Observe(addr) => {
                self.observe.do_send(Connect(addr));
                self.info.observer_cnt += 1;
                self.spread_info();
            }
            RoomJoin::List(addr) => {
                self.list.do_send(Connect(addr));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum RoomLeave {
    User(UserNo),
    Observe(Addr<Session<Observe>>),
    List(Addr<Session<List>>),
}

impl Handler<RoomLeave> for Room {
    type Result = Result<()>;

    fn handle(&mut self, msg: RoomLeave, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            RoomLeave::User(user_no) => {
                ensure!(
                    self.user_addr.remove(&user_no).is_some(),
                    StatusCode::NOT_FOUND,
                    "no user"
                );
                for i in self.info.user.iter_mut() {
                    if *i == user_no {
                        i.0 = 0;
                    }
                }
                self.set_head();
                self.spread_info();

                if self.user_addr.is_empty() {
                    self.hub.do_send(RemoveRoom(self.info.id));
                    ctx.stop();
                }
            }
            RoomLeave::Observe(addr) => {
                send(self, ctx, self.observe.clone(), Disconnect(addr))??;
                self.spread_info();
            }
            RoomLeave::List(addr) => {
                self.list.do_send(Disconnect(addr));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct ChangeName(pub UserNo, pub String);

impl Handler<ChangeName> for Room {
    type Result = Result<()>;

    fn handle(&mut self, msg: ChangeName, _: &mut Self::Context) -> Self::Result {
        ensure!(
            msg.0 == self.info.head,
            StatusCode::UNAUTHORIZED,
            "you are not head of room"
        );
        self.info.name = msg.1;
        self.spread_info();
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct StartGame(pub UserNo);

impl Handler<StartGame> for Room {
    type Result = Result<()>;

    fn handle(&mut self, msg: StartGame, ctx: &mut Self::Context) -> Self::Result {
        ensure!(
            msg.0 == self.info.head,
            StatusCode::UNAUTHORIZED,
            "you are not head of room"
        );
        let id = send(self, ctx, self.hub.clone(), MakeGameId)?;
        self.game = Some(GameInfo {
            id,
            no: 0,
            game: Game::new(self.info.rule.clone()),
        });
        self.info.is_game = true;
        self.spread_info();
        self.spread_game();
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct Go(pub UserNo, pub Command);

impl Handler<Go> for Room {
    type Result = Result<()>;

    fn handle(&mut self, msg: Go, ctx: &mut Self::Context) -> Self::Result {
        let mut user_id = self.info.user.len();
        for (i, x) in self.info.user.iter().enumerate() {
            if *x == msg.0 {
                user_id = i;
                break;
            }
        }

        ensure!(
            user_id != self.info.user.len(),
            StatusCode::UNAUTHORIZED,
            "you are not the player"
        );
        let finished = self.next(user_id, msg.1)?;
        self.save_state(ctx)?;

        if finished {
            // todo: apply rating system
            self.info.is_game = false;
            self.game = None;
            self.spread_info();
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "RoomInfo")]
pub struct GetInfo;

impl Handler<GetInfo> for Room {
    type Result = RoomInfo;

    fn handle(&mut self, _: GetInfo, _: &mut Self::Context) -> Self::Result {
        self.info.clone()
    }
}

impl Room {
    pub fn new(
        id: RoomId,
        name: String,
        rule: Rule,
        is_rank: bool,
        server: Addr<hub::Hub>,
        db: Addr<Database>,
    ) -> Room {
        let user_cnt = rule.user_cnt as usize;
        Room {
            info: RoomInfo {
                id,
                name,
                rule,
                is_rank,
                head: UserNo(0),
                user: vec![UserNo(0); user_cnt],
                observer_cnt: 0,
                is_game: false,
            },
            game: None,
            user_addr: HashMap::new(),
            observe: Group::start_default(),
            list: Group::start_default(),
            hub: server,
            db,
        }
    }

    fn set_head(&mut self) {
        if self.info.head.0 == 0 {
            for i in self.info.user.iter() {
                if i.0 != 0 {
                    self.info.head.0 = 0;
                }
            }
        } else if !self.user_addr.contains_key(&self.info.head) {
            self.info.head.0 = 0;
            // recursion: only go down once
            self.set_head();
        }
    }

    fn save_state(&mut self, ctx: &mut <Self as Actor>::Context) -> Result<()> {
        if let Some(game) = &self.game {
            let form = SaveStateForm {
                game_id: game.id.0,
                room_id: self.info.id.0,
                number: game.no,
                state: game.game.get_state(),
            };
            send(self, ctx, self.db.clone(), form)?
        } else {
            Ok(())
        }
    }

    fn next(&mut self, user_id: usize, cmd: mighty::Command) -> Result<bool> {
        if let Some(game) = &mut self.game {
            let res = game.game.next(user_id, cmd)?;
            self.spread_game();
            Ok(res)
        } else {
            bail!("game not started")
        }
    }

    fn spread_info(&self) {
        // todo
    }

    fn spread_game(&self) {
        // todo
    }
}
