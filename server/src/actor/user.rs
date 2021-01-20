use crate::actor::hub::GetRoom;
use crate::actor::room::{ChangeName, ChangeRule, Go, RoomJoin, RoomLeave, StartGame};
use crate::actor::{Hub, Main, Room, RoomUser};
use crate::dev::*;
use actix::clock::Duration;
use actix::prelude::*;
use mighty::prelude::State;
use std::collections::HashSet;
use std::time::SystemTime;

pub struct JoinedRoom {
    addr: Addr<Room>,
    info: RoomInfo,
    group: HashSet<Addr<Session<RoomUser>>>,
    disconn: u8,
}

pub struct User {
    info: UserInfo,
    status: UserStatus,
    conn: u8,
    disconn: u8,
    last_update: SystemTime,
    room: Option<JoinedRoom>,
    subscribers: HashSet<Addr<Session<Main>>>,
    hub: Addr<Hub>,
}

impl Actor for User {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum UserConnect {
    Room(Addr<Session<RoomUser>>),
    Subscribe(Addr<Session<Main>>),
    Main,
}

impl Handler<UserConnect> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserConnect, _: &mut Self::Context) -> Self::Result {
        match msg {
            UserConnect::Room(addr) => {
                ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "no joined room");
                self.room.as_mut().unwrap().group.insert(addr);
            }
            UserConnect::Subscribe(addr) => {
                addr.do_send(MainToClient::UserStatus(self.info.no, self.status));
                self.subscribers.insert(addr);
            }
            UserConnect::Main => {
                self.conn += 1;
                self.update_status();
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum UserDisconnect {
    Room(Addr<Session<RoomUser>>),
    Unsubscribe(Addr<Session<Main>>),
    Main,
}

impl Handler<UserDisconnect> for User {
    type Result = ();

    fn handle(&mut self, msg: UserDisconnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserDisconnect::Room(addr) => {
                if self.room.is_none() {
                    return;
                }
                let room = self.room.as_mut().unwrap();
                room.group.remove(&addr);
                room.disconn += 1;
                self.update_status();
                ctx.run_later(RECONNECTION_TIME, |act, _| {
                    act.room.as_mut().unwrap().disconn -= 1;
                    act.update_status();
                });
            }
            UserDisconnect::Unsubscribe(addr) => {
                self.subscribers.remove(&addr);
            }
            UserDisconnect::Main => {
                self.conn -= 1;
                self.disconn += 1;
                self.update_status();
                ctx.run_later(RECONNECTION_TIME, |act, _| {
                    act.disconn -= 1;
                    act.update_status();
                });
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserJoin(pub RoomId);

impl Handler<UserJoin> for User {
    type Result = ();

    fn handle(&mut self, msg: UserJoin, ctx: &mut Self::Context) -> Self::Result {
        if self.room.is_some() {
            return;
        }
        self.hub
            .send(GetRoom(msg.0))
            .into_actor(self)
            .then(|res, act, ctx| {
                if let Ok(Ok(room)) = res {
                    let room_addr = room.clone();
                    room.send(RoomJoin::User(act.info.no, ctx.address()))
                        .into_actor(act)
                        .then(move |res, act, _| {
                            if let Ok(info) = res {
                                if info.user.contains(&act.info.no) {
                                    act.room = Some(JoinedRoom {
                                        addr: room_addr,
                                        info,
                                        group: HashSet::new(),
                                        disconn: 0,
                                    });
                                } else {
                                    // todo
                                }
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                }

                fut::ready(())
            })
            .wait(ctx);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserLeave;

impl Handler<UserLeave> for User {
    type Result = ();

    fn handle(&mut self, _: UserLeave, _: &mut Self::Context) -> Self::Result {
        if self.room.is_none() {
            return;
        }
        self.room.as_ref().unwrap().addr.do_send(RoomLeave::User(self.info.no));
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct UserCommand(pub RoomUserToServer);

impl Handler<UserCommand> for User {
    type Result = ();

    fn handle(&mut self, msg: UserCommand, _: &mut Self::Context) -> Self::Result {
        if self.room.is_none() {
            return;
        }
        let room = self.room.as_ref().unwrap();
        let user_no = self.info.no;
        match msg.0 {
            RoomUserToServer::Start => {
                room.addr.do_send(StartGame(user_no));
            }
            RoomUserToServer::ChangeName(name) => {
                room.addr.do_send(ChangeName(user_no, name));
            }
            RoomUserToServer::ChangeRule(rule) => {
                room.addr.do_send(ChangeRule(user_no, rule));
            }
            RoomUserToServer::Command(cmd) => {
                room.addr.do_send(Go(user_no, cmd));
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct GotRoomInfo(pub RoomInfo);

impl Handler<GotRoomInfo> for User {
    type Result = ();

    fn handle(&mut self, msg: GotRoomInfo, _: &mut Self::Context) -> Self::Result {
        if self.room.is_none() {
            return;
        }
        let room = self.room.as_mut().unwrap();
        for i in room.group.iter() {
            i.do_send(RoomUserToClient::Room(msg.0.clone()));
        }
        room.info = msg.0;
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct GotGameState(pub State);

impl Handler<GotGameState> for User {
    type Result = ();

    fn handle(&mut self, msg: GotGameState, _: &mut Self::Context) -> Self::Result {
        if self.room.is_none() {
            return;
        }
        for i in self.room.as_ref().unwrap().group.iter() {
            i.do_send(RoomUserToClient::Game(msg.0.clone()));
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Update;

impl Handler<Update> for User {
    type Result = ();

    fn handle(&mut self, _: Update, ctx: &mut Self::Context) -> Self::Result {
        self.last_update = SystemTime::now();
        self.update_status();
        ctx.run_later(ABSENT_TIME + Duration::from_millis(1), |act, _| {
            act.update_status();
        });
    }
}

impl User {
    pub fn new(info: UserInfo, hub: Addr<Hub>) -> User {
        User {
            info,
            status: UserStatus::OFFLINE,
            conn: 0,
            disconn: 0,
            last_update: SystemTime::now(),
            room: None,
            subscribers: HashSet::new(),
            hub,
        }
    }

    fn get_status(&mut self) -> UserStatus {
        let mut status = UserStatus::empty();
        let mut leave_room = false;

        if let Some(room) = &self.room {
            status |= UserStatus::IN_ROOM;

            if room.info.is_game {
                status |= UserStatus::IN_GAME;
            }

            if room.group.is_empty() {
                if room.disconn > 0 {
                    status |= UserStatus::ROOM_DISCONN;
                } else if room.info.is_game {
                    // todo: ghost
                } else {
                    leave_room = true;
                }
            }
        }

        if leave_room {
            self.room.as_ref().unwrap().addr.do_send(RoomLeave::User(self.info.no));
        }

        if self.conn == 0 {
            if self.disconn == 0 {
                status |= UserStatus::OFFLINE;
            } else {
                status |= UserStatus::DISCONNECTED;
            }
        } else if self.last_update.elapsed().unwrap_or_else(|_| Duration::from_secs(0)) >= ABSENT_TIME {
            status |= UserStatus::ABSENT;
        } else {
            status |= UserStatus::ONLINE;
        }

        status
    }

    fn update_status(&mut self) {
        let new_state = self.get_status();
        if self.status != new_state {
            self.status = new_state;

            for addr in self.subscribers.iter() {
                addr.do_send(MainToClient::UserStatus(self.info.no, self.status));
            }
        }
    }
}
