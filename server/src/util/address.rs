use actix::dev::ToEnvelope;
use actix::prelude::*;

/// This is and address that is able not to contain address of Actor.
/// If address cannot be retrieved instantly or address have to be remove in some time,
/// this can do the job of queueing messages to send.
///
/// This supports `do_send` currently. Will be supporting if other features are needed.
///
/// # Examples
///
/// ```
/// use actix::prelude::*;
/// use server::util::ExAddr;
///
/// pub struct Counter {
///     counter: u32,
/// }
///
/// impl Actor for Counter {
///     type Context = Context<Self>;
/// }
///
/// #[derive(Clone, Message)]
/// #[rtype(result = "()")]
/// pub struct AddOne;
///
/// impl Handler<AddOne> for Counter {
///     type Result = ();
///
///     fn handle(&mut self, msg: AddOne, _: &mut Self::Context) -> Self::Result {
///         self.counter += 1;
///     }
/// }
///
/// let mut addr = ExAddr::new();
/// let counter = Counter { counter: 0 }.start();
///
/// addr.do_send(AddOne);
/// addr.set_addr(counter);
/// addr.do_send(AddOne);
/// addr.unset_addr();
/// ```
pub enum ExAddr<A>
where
    A: Actor,
{
    Addr(Addr<A>),
    Queue(Vec<Box<dyn Fn(Addr<A>)>>),
}

impl<A> Default for ExAddr<A>
where
    A: Actor,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> From<Addr<A>> for ExAddr<A>
where
    A: Actor,
{
    fn from(addr: Addr<A>) -> Self {
        ExAddr::Addr(addr)
    }
}

impl<A> From<Option<Addr<A>>> for ExAddr<A>
where
    A: Actor,
{
    fn from(addr: Option<Addr<A>>) -> Self {
        match addr {
            Some(addr) => ExAddr::Addr(addr),
            None => Self::new(),
        }
    }
}

impl<A> ExAddr<A>
where
    A: Actor,
{
    pub fn new() -> ExAddr<A> {
        ExAddr::Queue(Vec::new())
    }

    pub fn set_addr(&mut self, addr: Addr<A>) {
        match self {
            ExAddr::Addr(_) => {}
            ExAddr::Queue(v) => {
                for func in v.iter() {
                    func(addr.clone());
                }
            }
        }

        *self = ExAddr::Addr(addr)
    }

    pub fn unset_addr(&mut self) {
        if let ExAddr::Addr(_) = self {
            *self = ExAddr::Queue(Vec::new());
        }
    }

    pub fn do_send<M>(&mut self, msg: M)
    where
        M: 'static,
        M: Message + Clone + Send,
        M::Result: Send,
        A: Handler<M>,
        A::Context: ToEnvelope<A, M>,
    {
        match self {
            ExAddr::Addr(addr) => {
                addr.do_send(msg);
            }
            ExAddr::Queue(v) => {
                v.push(Box::new(move |addr| addr.do_send(msg.clone())));
            }
        }
    }

    pub fn is_set(&self) -> bool {
        matches!(self, ExAddr::Addr(_))
    }
}
