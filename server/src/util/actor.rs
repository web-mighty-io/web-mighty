use crate::prelude::*;
use actix::dev::ToEnvelope;
use actix::prelude::*;

pub fn send<A, B, M>(actor: &A, ctx: &mut A::Context, to: Addr<B>, msg: M) -> Result<M::Result, MailboxError>
where
    A: Actor,
    A::Context: AsyncContext<A>,
    B: Actor,
    M: Message + Send + 'static,
    M::Result: Send,
    B: Handler<M>,
    B::Context: ToEnvelope<B, M>,
{
    let mut x = Err(MailboxError::Closed);
    let r = &mut x as *const Result<M::Result, MailboxError> as *mut Result<M::Result, MailboxError>;
    // SAFETY: referencing `x` is finished inside unsafe code block
    unsafe {
        to.send(msg)
            .into_actor(actor)
            .then(move |res, _, _| {
                *r = res;
                fut::ready(())
            })
            .wait(ctx);
    }
    x
}
