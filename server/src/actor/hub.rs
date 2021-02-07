use crate::actor::room::Room;
use crate::actor::user::User;
use crate::db::game::{save_rule, SaveRuleForm};
use crate::db::user::{get_user_info, GetInfoForm};
use crate::dev::*;
use actix::prelude::*;
use mighty::prelude::Rule;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashMap;

/// Hub Actor
///
/// This contains addresses of user and room.
/// It will give or make addresses of user & room.
#[derive(Debug)]
pub struct Hub {
    room: HashMap<RoomId, Addr<Room>>,
    counter: u64,
    users: HashMap<UserNo, Addr<User>>,
    pool: Pool,
}

impl Actor for Hub {
    type Context = Context<Self>;
}

/// This would request the address of room.
/// If the room doesn't exists, it would respond error.
#[derive(Debug, Clone, Message)]
#[rtype(result = "Result<Addr<Room>>")]
pub struct GetRoom(pub RoomId);

impl Handler<GetRoom> for Hub {
    type Result = Result<Addr<Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.room.get(&msg.0).cloned().ok_or_else(|| err!("no room"))
    }
}

/// This would make room with `room_name`, `rule`, and `is_rank`.
/// The `room_id` would generated with random value.
#[derive(Debug, Clone, Message)]
#[rtype(result = "RoomId")]
pub struct MakeRoom(pub String, pub Rule, pub bool);

impl Handler<MakeRoom> for Hub {
    type Result = RoomId;

    fn handle(&mut self, msg: MakeRoom, ctx: &mut Self::Context) -> Self::Result {
        let room_uuid = RoomUid::generate_random();
        let room_id = self.generate_room_id();
        let user_cnt = msg.1.user_cnt as usize;
        let rule = RuleHash::generate(&msg.1);
        let _ = save_rule(SaveRuleForm { rule: msg.1.clone() }, self.pool.clone());
        let room = Room::new(
            RoomInfo {
                uid: room_uuid,
                id: room_id,
                name: msg.0,
                rule,
                is_rank: msg.2,
                head: UserNo(0),
                user: vec![UserNo(0); user_cnt],
                observer_cnt: 0,
                is_game: false,
            },
            ctx.address(),
            self.pool.clone(),
        )
        .start();
        self.room.insert(room_id, room);
        room_id
    }
}

/// Removes the room if exists
#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveRoom(pub RoomId);

impl Handler<RemoveRoom> for Hub {
    type Result = ();

    fn handle(&mut self, msg: RemoveRoom, _: &mut Self::Context) -> Self::Result {
        self.room.remove(&msg.0);
    }
}

/// Returns the existing user or make new one.
/// Returns error when `user_no` doesn't exists.
#[derive(Debug, Clone, Message)]
#[rtype(result = "Result<Addr<User>>")]
pub struct HubConnect(pub UserNo);

impl Handler<HubConnect> for Hub {
    type Result = Result<Addr<User>>;

    fn handle(&mut self, msg: HubConnect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.users.get(&msg.0) {
            Ok(addr.clone())
        } else {
            let user_info = get_user_info(GetInfoForm::UserNo(msg.0 .0), self.pool.clone())?;
            let user = User::new(user_info, ctx.address(), self.pool.clone()).start();
            self.users.insert(msg.0, user.clone());
            Ok(user)
        }
    }
}

/// Returns address of user if user is present or returns error
#[derive(Debug, Clone, Message)]
#[rtype(result = "Result<Addr<User>>")]
pub struct GetUser(pub UserNo);

impl Handler<GetUser> for Hub {
    type Result = Result<Addr<User>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.0).cloned().ok_or_else(|| err!("no user"))
    }
}

/// When user gets offline, this would remove user.
#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct HubDisconnect(pub UserNo);

impl Handler<HubDisconnect> for Hub {
    type Result = ();

    fn handle(&mut self, msg: HubDisconnect, _: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.0);
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

    /// Generate random 6-digit `room_id`
    pub fn generate_room_id(&mut self) -> RoomId {
        loop {
            let id = RoomId(Uniform::new(0, 999999).sample(&mut thread_rng()));
            if !self.room.contains_key(&id) {
                break id;
            }
        }
    }
}
