use crate::actor::room::Room;
use crate::actor::user::User;
use crate::actor::{GameId, RoomId, UserNo};
use crate::db::user::{get_info, GetInfoForm};
use actix::prelude::*;
use deadpool_postgres::Pool;
use mighty::rule::Rule;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Hub {
    room: HashMap<RoomId, Addr<Room>>,
    counter: u64,
    users: HashMap<UserNo, Addr<User>>,
    pool: Pool,
}

impl Actor for Hub {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "Option<Addr<Room>>")]
pub struct GetRoom(pub RoomId);

impl Handler<GetRoom> for Hub {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.room.get(&msg.0).cloned()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "RoomId")]
pub struct MakeRoom(pub String, pub Rule);

impl Handler<MakeRoom> for Hub {
    type Result = RoomId;

    fn handle(&mut self, msg: MakeRoom, ctx: &mut Self::Context) -> Self::Result {
        let room_id = RoomId(self.generate_uuid("room"));
        self.room.insert(
            room_id,
            Room::new(room_id, msg.0, msg.1, ctx.address(), self.pool.clone()).start(),
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
#[rtype(result = "Addr<User>")]
pub struct Connect(pub UserNo);

impl Handler<Connect> for Hub {
    type Result = Addr<User>;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.users.get(&msg.0) {
            addr.clone()
        } else {
            let user_no = msg.0;
            get_info(GetInfoForm::UserNo(msg.0 .0), self.pool.clone())
                .into_actor(self)
                .then(move |res, act, ctx| {
                    let user = User::new(res.unwrap(), ctx.address(), act.pool.clone()).start();
                    act.users.insert(user_no, user);

                    fut::ready(())
                })
                .wait(ctx);

            self.users.get(&user_no).unwrap().clone()
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Option<Addr<User>>")]
pub struct GetUser(pub UserNo);

impl Handler<GetUser> for Hub {
    type Result = Option<Addr<User>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.0).cloned()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub UserNo);

impl Handler<Disconnect> for Hub {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
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
    pub fn new(pool: Pool) -> Hub {
        Hub {
            room: HashMap::new(),
            counter: 0,
            users: HashMap::new(),
            pool,
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
