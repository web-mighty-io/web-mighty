use actix::dev::ToEnvelope;
use actix::prelude::*;

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
        *self = ExAddr::Addr(addr);
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
}
