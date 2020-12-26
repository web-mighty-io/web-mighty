use crate::actor::db::UserInfo;
use crate::actor::{Database, Hub, Room, RoomId};
use crate::session::{ListSession, MainSession, ObserveSession, RoomSession};
use crate::util::{self, AddListener, Connection, ExAddr, Status};
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    InGame,
    InRoom,
    Online,
    Absent,
    Disconnected,
    Offline,
}

pub struct RoomInfo {
    room_id: RoomId,
    room: ExAddr<Room>,
    session: Addr<RoomSession>,
    time: SystemTime,
}

pub struct User {
    info: UserInfo,
    status: UserStatus,
    room: Option<RoomInfo>,
    list_session: Addr<Connection<ListSession>>,
    list_status: Status,
    main_session: Addr<Connection<MainSession>>,
    main_status: Status,
    observe_session: Addr<Connection<ObserveSession>>,
    observe_status: Status,
    hub: Addr<Hub>,
    db: Addr<Database>,
}

impl Actor for User {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.list_session
            .do_send(AddListener(move |status| addr.do_send(ChangeStatus::List(status))));
        let addr = ctx.address();
        self.main_session
            .do_send(AddListener(move |status| addr.do_send(ChangeStatus::Main(status))));
        let addr = ctx.address();
        self.observe_session
            .do_send(AddListener(move |status| addr.do_send(ChangeStatus::Observe(status))));
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum Connect {
    Game(Addr<RoomSession>, RoomId),
    List(Addr<ListSession>),
    Main(Addr<MainSession>),
    Observe(Addr<ObserveSession>),
}

impl Handler<Connect> for User {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        match msg {
            Connect::Game(addr, room_id) => {}
            Connect::List(addr) => {
                self.list_session.do_send(util::Connect(addr));
            }
            Connect::Main(addr) => {
                self.main_session.do_send(util::Connect(addr));
            }
            Connect::Observe(addr) => {
                self.observe_session.do_send(util::Connect(addr));
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum Disconnect {
    Game,
    List(Addr<ListSession>),
    Main(Addr<MainSession>),
    Observe(Addr<ObserveSession>),
}

impl Handler<Disconnect> for User {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Disconnect::Game => {}
            Disconnect::List(addr) => {
                self.list_session.do_send(util::Disconnect(addr));
            }
            Disconnect::Main(addr) => {
                self.main_session.do_send(util::Disconnect(addr));
            }
            Disconnect::Observe(addr) => {
                self.observe_session.do_send(util::Disconnect(addr));
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Leave;

impl Handler<Leave> for User {
    type Result = ();

    fn handle(&mut self, msg: Leave, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub enum ChangeStatus {
    List(Status),
    Main(Status),
    Observe(Status),
}

impl Handler<ChangeStatus> for User {
    type Result = ();

    fn handle(&mut self, msg: ChangeStatus, _: &mut Self::Context) -> Self::Result {
        match msg {
            ChangeStatus::List(s) => {
                self.list_status = s;
            }
            ChangeStatus::Main(s) => {
                self.main_status = s;
            }
            ChangeStatus::Observe(s) => {
                self.observe_status = s;
            }
        }
    }
}

impl User {
    pub fn new(info: UserInfo, hub: Addr<Hub>, db: Addr<Database>) -> User {
        User {
            info,
            status: UserStatus::Online,
            room: None,
            list_session: Connection::start_default(),
            list_status: Status::Offline,
            main_session: Connection::start_default(),
            main_status: Status::Offline,
            observe_session: Connection::start_default(),
            observe_status: Status::Offline,
            hub,
            db,
        }
    }
}
