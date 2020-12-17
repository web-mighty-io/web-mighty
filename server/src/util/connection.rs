use crate::{ABSENT_TIME, RECONNECTION_TIME};
use actix::dev::ToEnvelope;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::SystemTime;

/// Status of connection
///
/// - Online: connection is online
/// - Absent: when not moving for ABSENT_TIME
/// - Disconnected: when disconnected
/// - Offline: when disconnected for RECONNECTION_TIME
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Status {
    Online,
    Absent,
    Disconnected,
    Offline,
}

pub struct Connection<A>
where
    A: Actor,
{
    addrs: HashSet<Addr<A>>,
    time: SystemTime,
}

impl<A> Connection<A>
where
    A: Actor,
{
    pub fn new() -> Connection<A> {
        Connection {
            addrs: HashSet::new(),
            time: SystemTime::now(),
        }
    }

    pub fn status(&self) -> Status {
        if self.addrs.is_empty() {
            if self.time.elapsed().unwrap() < ABSENT_TIME {
                Status::Online
            } else {
                Status::Absent
            }
        } else if self.time.elapsed().unwrap() < RECONNECTION_TIME {
            Status::Disconnected
        } else {
            Status::Offline
        }
    }

    pub fn connect(&mut self, addr: Addr<A>) {
        self.addrs.insert(addr);
        self.update();
    }

    pub fn disconnect(&mut self, addr: Addr<A>) {
        self.addrs.remove(&addr);
        self.update();
    }

    pub fn do_send<M>(&self, msg: M)
    where
        M: Message + Clone + Send,
        M::Result: Send,
        A: Handler<M>,
        A::Context: ToEnvelope<A, M>,
    {
        for i in self.addrs.iter() {
            i.do_send(msg.clone());
        }
    }

    pub fn update(&mut self) {
        self.time = SystemTime::now();
    }
}
