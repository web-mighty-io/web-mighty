use crate::dev::*;
use actix::dev::ToEnvelope;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

/// Status of connection
///
/// - Online: connection is online
/// - Absent: when not moving for ABSENT_TIME
/// - Disconnected: when disconnected
/// - Offline: when disconnected for RECONNECTION_TIME
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Debug, MessageResponse)]
pub enum Status {
    Online,
    Absent,
    Disconnected,
    Offline,
}

/// Connection is handler for multiple actor connections with status
///
/// # Example
///
/// ```no_run
/// # use actix::prelude::*;
/// # use server::prelude::{AddListener, Connect, Connection, Spread};
///
/// # #[derive(Clone, Default)]
/// # pub struct A {
/// #     counter: usize,
/// # }
///
/// # impl Actor for A {
/// #     type Context = Context<Self>;
/// # }
///
/// # #[derive(Clone, Message)]
/// # #[rtype(result = "()")]
/// # pub struct AddOne;
///
/// # impl Handler<AddOne> for A {
/// #     type Result = ();
///
/// #     fn handle(&mut self, _: AddOne, _: &mut Self::Context) -> Self::Result {
/// #         self.counter += 1;
/// #         println!("{}", self.counter);
/// #     }
/// # }
///
/// let connection: Addr<Connection<A>> = Connection::start_default();
/// connection.do_send(AddListener(|status| {
///     println!("status changed: {:?}", status);
/// }));
/// connection.do_send(Connect(A::start_default()));
/// connection.do_send(Connect(A::start_default()));
/// connection.do_send(Spread(AddOne));
/// ```
pub struct Connection<A>
where
    A: Actor,
{
    addrs: HashSet<Addr<A>>,
    time: SystemTime,
    status: Status,
    status_listener: HashMap<usize, Box<dyn FnMut(Status)>>,
    counter: usize,
}

impl<A> Actor for Connection<A>
where
    A: Actor,
{
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Spread<M>(pub M)
where
    M: Message + Clone,
    M::Result: Send;

impl<A, M> Handler<Spread<M>> for Connection<A>
where
    A: Actor + Handler<M>,
    M: Message + Clone + Send,
    M::Result: Send,
    A::Context: ToEnvelope<A, M>,
{
    type Result = ();

    fn handle(&mut self, msg: Spread<M>, ctx: &mut Self::Context) -> Self::Result {
        self.update(ctx);
        for i in self.addrs.iter() {
            i.do_send(msg.0.clone());
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Connect<A>(pub Addr<A>)
where
    A: Actor;

impl<A> Handler<Connect<A>> for Connection<A>
where
    A: Actor,
{
    type Result = ();

    fn handle(&mut self, msg: Connect<A>, ctx: &mut Self::Context) -> Self::Result {
        self.update(ctx);
        self.addrs.insert(msg.0);
        if self.addrs.len() == 1 {
            ctx.notify(Update);
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct Disconnect<A>(pub Addr<A>)
where
    A: Actor;

impl<A> Handler<Disconnect<A>> for Connection<A>
where
    A: Actor,
{
    type Result = Result<()>;

    fn handle(&mut self, msg: Disconnect<A>, ctx: &mut Self::Context) -> Self::Result {
        self.update(ctx);
        ensure!(self.addrs.remove(&msg.0), StatusCode::NOT_FOUND, "no user");
        if self.addrs.is_empty() {
            ctx.notify(Update);
            ctx.notify_later(Update, RECONNECTION_TIME + Duration::from_millis(1));
        }
        Ok(())
    }
}

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct AddListener<F: 'static + FnMut(Status)>(pub F);

impl<A, F> Handler<AddListener<F>> for Connection<A>
where
    A: Actor,
    F: 'static + FnMut(Status),
{
    type Result = usize;

    fn handle(&mut self, msg: AddListener<F>, _: &mut Self::Context) -> Self::Result {
        self.add_listener(msg.0)
    }
}

#[derive(Clone, Message)]
#[rtype(result = "")]
pub struct RemoveListener(pub usize);

impl<A> Handler<RemoveListener> for Connection<A>
where
    A: Actor,
{
    type Result = ();

    fn handle(&mut self, msg: RemoveListener, _: &mut Self::Context) -> Self::Result {
        self.remove_listener(msg.0);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Update;

impl<A> Handler<Update> for Connection<A>
where
    A: Actor,
{
    type Result = ();

    fn handle(&mut self, _: Update, _: &mut Self::Context) -> Self::Result {
        if self.update_status() {
            for (_, f) in self.status_listener.iter_mut() {
                f(self.status);
            }
        }
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Status")]
pub struct GetStatus;

impl<A> Handler<GetStatus> for Connection<A>
where
    A: Actor,
{
    type Result = Status;

    fn handle(&mut self, _: GetStatus, _: &mut Self::Context) -> Self::Result {
        self.status
    }
}

impl<A> Default for Connection<A>
where
    A: Actor,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Connection<A>
where
    A: Actor,
{
    pub fn new() -> Connection<A> {
        Connection {
            addrs: HashSet::new(),
            time: SystemTime::now() - RECONNECTION_TIME,
            status: Status::Offline,
            status_listener: HashMap::new(),
            counter: 0,
        }
    }

    fn update_status(&mut self) -> bool {
        let status = if self.addrs.is_empty() {
            if self.time.elapsed().unwrap() < ABSENT_TIME {
                Status::Online
            } else {
                Status::Absent
            }
        } else if self.time.elapsed().unwrap() < RECONNECTION_TIME {
            Status::Disconnected
        } else {
            Status::Offline
        };

        if self.status != status {
            self.status = status;
            true
        } else {
            false
        }
    }

    fn next_id(&mut self) -> usize {
        loop {
            if !self.status_listener.contains_key(&self.counter) {
                break self.counter;
            }
            // overflow can happen: overflow is expected & does not throw error on release builds.
            self.counter += 1;
        }
    }

    fn add_listener<F: 'static + FnMut(Status)>(&mut self, func: F) -> usize {
        let id = self.next_id();
        self.status_listener.insert(id, Box::new(func));
        id
    }

    fn remove_listener(&mut self, id: usize) {
        self.status_listener.remove(&id);
    }

    fn update(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.time = SystemTime::now();
        ctx.notify_later(Update, ABSENT_TIME + Duration::from_millis(1));
    }
}
