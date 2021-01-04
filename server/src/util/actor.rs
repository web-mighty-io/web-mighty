use actix::dev::ToEnvelope;
use actix::prelude::*;
use std::sync::mpsc::channel;

pub fn send<A, B, M>(actor: &mut A, ctx: &mut A::Context, to: Addr<B>, msg: M) -> Result<M::Result, MailboxError>
where
    A: Actor,
    A::Context: AsyncContext<A>,
    B: Actor,
    M: Message + Send + 'static,
    M::Result: Send,
    B: Handler<M>,
    B::Context: ToEnvelope<B, M>,
{
    let (tx, rx) = channel();
    to.send(msg)
        .into_actor(actor)
        .then(move |res, _, _| {
            let _ = tx.send(res);
            fut::ready(())
        })
        .wait(ctx);
    rx.recv().unwrap()
}
