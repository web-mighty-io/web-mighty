use crate::actor::db::GetInfoForm;
use crate::actor::room::Room;
use crate::actor::user::User;
use crate::actor::{Database, GameId, RoomId, UserNo};
use crate::dev::*;
use actix::prelude::*;
use mighty::rule::Rule;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Hub {
    room: HashMap<RoomId, Addr<Room>>,
    counter: u64,
    users: HashMap<UserNo, Addr<User>>,
    db: Addr<Database>,
}

impl Actor for Hub {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "Result<Addr<Room>>")]
pub struct GetRoom(pub RoomId);

impl Handler<GetRoom> for Hub {
    type Result = Result<Addr<Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.room.get(&msg.0).cloned().ok_or_else(|| err!("no room"))
    }
}

#[derive(Clone, Message)]
#[rtype(result = "RoomId")]
pub struct MakeRoom(pub String, pub Rule, pub bool);

impl Handler<MakeRoom> for Hub {
    type Result = RoomId;

    fn handle(&mut self, msg: MakeRoom, ctx: &mut Self::Context) -> Self::Result {
        let room_id = RoomId(self.generate_uuid("room"));
        self.room.insert(
            room_id,
            Room::new(room_id, msg.0, msg.1, msg.2, ctx.address(), self.db.clone()).start(),
        );
        room_id
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveRoom(pub RoomId);

impl Handler<RemoveRoom> for Hub {
    type Result = ();

    fn handle(&mut self, msg: RemoveRoom, _: &mut Self::Context) -> Self::Result {
        self.room.remove(&msg.0);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<Addr<User>>")]
pub struct HubConnect(pub UserNo);

impl Handler<HubConnect> for Hub {
    type Result = Result<Addr<User>>;

    fn handle(&mut self, msg: HubConnect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.users.get(&msg.0) {
            Ok(addr.clone())
        } else {
            let info = send(self, ctx, self.db.clone(), GetInfoForm::UserNo(msg.0 .0))??;
            let user = User::new(info, ctx.address()).start();
            self.users.insert(msg.0, user.clone());
            Ok(user)
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<Addr<User>>")]
pub struct GetUser(pub UserNo);

impl Handler<GetUser> for Hub {
    type Result = Result<Addr<User>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.0).cloned().ok_or_else(|| err!("no user"))
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct HubDisconnect(pub UserNo);

impl Handler<HubDisconnect> for Hub {
    type Result = ();

    fn handle(&mut self, msg: HubDisconnect, _: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.0);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "GameId")]
pub struct MakeGameId;

impl Handler<MakeGameId> for Hub {
    type Result = GameId;

    fn handle(&mut self, _: MakeGameId, _: &mut Self::Context) -> Self::Result {
        GameId(self.generate_uuid("game"))
    }
}

impl Hub {
    pub fn new(db: Addr<Database>) -> Hub {
        Hub {
            room: HashMap::new(),
            counter: 0,
            users: HashMap::new(),
            db,
        }
    }

    pub fn generate_uuid(&mut self, tag: &str) -> Uuid {
        self.counter += 1;
        Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            format!(
                "{}-{}-{}",
                tag,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                self.counter,
            )
            .as_ref(),
        )
    }
}
