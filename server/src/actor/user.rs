use crate::actor::db::UserInfo;
use crate::actor::hub::GetRoom;
use crate::actor::room::{self, GetInfo, RoomInfo, RoomJoin, RoomLeave};
use crate::actor::{Hub, Room, RoomId};
use crate::prelude::*;
use crate::session::{MainSession, RoomSession};
use actix::prelude::*;
use bitflags::bitflags;
use mighty::{Command, State};
use serde::{Deserialize, Serialize};

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
    group: Addr<Group<RoomSession>>,
}

pub struct User {
    info: UserInfo,
    status: UserStatus,
    room: Option<JoinedRoom>,
    main: Addr<Connection<MainSession>>,
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
    Room(Addr<RoomSession>),
    Main(Addr<MainSession>),
}

impl Handler<UserConnect> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserConnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserConnect::Room(addr) => {
                ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "no joined room");
                send(self, ctx, self.room.as_ref().unwrap().group.clone(), Connect(addr))?;
            }
            UserConnect::Main(addr) => {
                send(self, ctx, self.main.clone(), Connect(addr))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub enum UserDisconnect {
    Room(Addr<RoomSession>),
    Main(Addr<MainSession>),
}

impl Handler<UserDisconnect> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: UserDisconnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            UserDisconnect::Room(addr) => {
                ensure!(self.room.is_some(), StatusCode::NOT_FOUND, "not joined in room");
                send(self, ctx, self.room.as_ref().unwrap().group.clone(), Disconnect(addr))??;
            }
            UserDisconnect::Main(addr) => {
                send(self, ctx, self.main.clone(), Disconnect(addr))??;
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
            group: Group::start_default(),
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
#[rtype(result = "()")]
pub struct ChangeStatus(pub Status);

impl Handler<ChangeStatus> for User {
    type Result = ();

    fn handle(&mut self, msg: ChangeStatus, _: &mut Self::Context) -> Self::Result {
        self.status = (self.status & UserStatus::ROOM_MASK) | msg.0.into();
        // todo
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct Go(pub Command);

impl Handler<Go> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: Go, ctx: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), StatusCode::NOT_FOUND, "not joined in room");
        let to = self.room.as_ref().unwrap().addr.clone();
        send(self, ctx, to, room::Go(self.info.user_no.into(), msg.0))??;
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct GotRoomInfo(pub RoomInfo);

impl Handler<GotRoomInfo> for User {
    type Result = Result<()>;

    fn handle(&mut self, msg: GotRoomInfo, _: &mut Self::Context) -> Self::Result {
        ensure!(self.room.is_some(), StatusCode::BAD_REQUEST, "not joined in room");
        self.room.as_mut().unwrap().info = msg.0;
        // todo
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct GotGameState(pub State);

impl Handler<GotGameState> for User {
    type Result = ();

    fn handle(&mut self, _: GotGameState, _: &mut Self::Context) -> Self::Result {
        // todo
    }
}

impl User {
    pub fn new(info: UserInfo, hub: Addr<Hub>) -> User {
        User {
            info,
            status: UserStatus::OFFLINE,
            room: None,
            main: Connection::start_default(),
            hub,
        }
    }
}
