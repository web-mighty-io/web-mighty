use crate::actor::db::UserInfo;
use crate::actor::hub::GetRoom;
use crate::actor::main::MainSend;
use crate::actor::room::{ChangeName, ChangeRule, GetInfo, Go, RoomInfo, RoomJoin, RoomLeave, StartGame};
use crate::actor::room_user::{RoomUserReceive, RoomUserSend};
use crate::actor::{Hub, Main, Room, RoomId, RoomUser};
use crate::dev::*;
use actix::prelude::*;
use bitflags::bitflags;
use mighty::prelude::State;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct UserStatus: u8 {
        const ROOM_MASK    = 0b1100;
        const IN_GAME      = 0b1100;
        const IN_ROOM      = 0b0100;
        const ONLINE       = 0b0011;
        const ABSENT       = 0b0010;
        const DISCONNECTED = 0b0001;
        const OFFLINE      = 0b0000;
    }
}

impl From<Status> for UserStatus {
    fn from(s: Status) -> Self {
        match s {
            Status::Online => UserStatus::ONLINE,
            Status::Absent => UserStatus::ABSENT,
            Status::Disconnected => UserStatus::DISCONNECTED,
            Status::Offline => UserStatus::OFFLINE,
        }
    }
}

pub struct JoinedRoom {
    addr: Addr<Room>,
    info: RoomInfo,
    group: HashSet<Addr<Session<RoomUser>>>,
}

pub struct User {
    info: UserInfo,
    status: UserStatus,
    room: Option<JoinedRoom>,
    main: Addr<Connection<Session<Main>>>,
    subscribers: HashSet<Addr<Session<Main>>>,
    hub: Addr<Hub>,
}

impl Actor for User {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.main
            .do_send(AddListener(move |status| addr.do_send(ChangeStatus(status))));
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum UserConnect {
    Room(Addr<Session<RoomUser>>),
    Main(Addr<Session<Main>>),
    Subscribe(Addr<Session<Main>>),
}

impl Handler<UserConnect> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserConnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserConnect::Room(addr) => {
                ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "no joined room");
                self.room.as_mut().unwrap().group.insert(addr);
            }
            UserConnect::Main(addr) => {
                send(self, ctx, self.main.clone(), Connect(addr))?;
            }
            UserConnect::Subscribe(addr) => {
                self.subscribers.insert(addr);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum UserDisconnect {
    Room(Addr<Session<RoomUser>>),
    Main(Addr<Session<Main>>),
    Unsubscribe(Addr<Session<Main>>),
}

impl Handler<UserDisconnect> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserDisconnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserDisconnect::Room(addr) => {
                ensure!(self.room.is_some(), StatusCode::NOT_FOUND, "not joined in room");
                ensure!(
                    self.room.as_mut().unwrap().group.remove(&addr),
                    StatusCode::BAD_REQUEST,
                    "you are not joined"
                );
            }
            UserDisconnect::Main(addr) => {
                send(self, ctx, self.main.clone(), Disconnect(addr))??;
            }
            UserDisconnect::Unsubscribe(addr) => {
                ensure!(self.subscribers.remove(&addr), StatusCode::NOT_FOUND, "not subscribed");
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct UserJoin(pub RoomId);

impl Handler<UserJoin> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserJoin, ctx: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_none(), StatusCode::BAD_REQUEST, "already joined to room");
        let room = send(self, ctx, self.hub.clone(), GetRoom(msg.0))??;
        send(
            self,
            ctx,
            room.clone(),
            RoomJoin::User(self.info.user_no.into(), ctx.address()),
        )??;
        let info = send(self, ctx, room.clone(), GetInfo)?;
        self.room = Some(JoinedRoom {
            addr: room,
            info,
            group: HashSet::new(),
        });
        self.status |= UserStatus::IN_ROOM;
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct UserLeave;

impl Handler<UserLeave> for User {
    type Result = Result<()>;

    fn handle(&mut self, _: UserLeave, ctx: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), StatusCode::NOT_FOUND, "not joined in room");
        let to = self.room.as_ref().unwrap().addr.clone();
        send(self, ctx, to, RoomLeave::User(self.info.user_no.into()))??;
        self.status ^= self.status & UserStatus::ROOM_MASK;
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct UserCommand(pub RoomUserReceive);

impl Handler<UserCommand> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserCommand, ctx: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), "not joined in room");
        let room = self.room.as_ref().unwrap();
        let user_no = self.info.user_no.into();
        match msg.0 {
            RoomUserReceive::Start => {
                send(self, ctx, room.addr.clone(), StartGame(user_no))??;
            }
            RoomUserReceive::ChangeName(name) => {
                send(self, ctx, room.addr.clone(), ChangeName(user_no, name))??;
            }
            RoomUserReceive::ChangeRule(rule) => {
                send(self, ctx, room.addr.clone(), ChangeRule(user_no, rule))??;
            }
            RoomUserReceive::Command(cmd) => {
                send(self, ctx, room.addr.clone(), Go(user_no, cmd))??;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeStatus(pub Status);

impl Handler<ChangeStatus> for User {
    type Result = ();

    fn handle(&mut self, msg: ChangeStatus, _: &mut Self::Context) -> Self::Result {
        self.status = (self.status & UserStatus::ROOM_MASK) | msg.0.into();
        for i in self.subscribers.iter() {
            i.do_send(MainSend {
                user_no: self.info.user_no.into(),
                status: self.status,
            });
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct GotRoomInfo(pub RoomInfo);

impl Handler<GotRoomInfo> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: GotRoomInfo, _: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "not joined in room");
        let room = self.room.as_mut().unwrap();
        for i in room.group.iter() {
            i.do_send(RoomUserSend::Room(msg.0.clone()));
        }
        room.info = msg.0;
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct GotGameState(pub State);

impl Handler<GotGameState> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: GotGameState, _: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "not joined in room");
        for i in self.room.as_ref().unwrap().group.iter() {
            i.do_send(RoomUserSend::Game(msg.0.clone()));
        }
        Ok(())
    }
}

impl User {
    pub fn new(info: UserInfo, hub: Addr<Hub>) -> User {
        User {
            info,
            status: UserStatus::OFFLINE,
            room: None,
            main: Connection::start_default(),
            subscribers: HashSet::new(),
            hub,
        }
    }
}
