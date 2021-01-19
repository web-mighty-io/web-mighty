use crate::actor::hub::GetUser;
use crate::actor::user::{Update, UserConnect, UserDisconnect};
use crate::actor::{Hub, User};
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use types::{MainToClient, MainToServer};

pub struct Main {
    user: Addr<User>,
    hub: Addr<Hub>,
}

impl SessionTrait for Main {
    type Sender = MainToClient;

    fn started(act: &mut Session<Self>, _: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserConnect::Main);
    }

    fn stopped(act: &mut Session<Self>, _: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserDisconnect::Main);
    }

    fn receive(act: &mut Session<Self>, msg: String, ctx: &mut WebsocketContext<Session<Self>>) {
        let msg: MainToServer = serde_json::from_str(&*msg).unwrap();
        match msg {
            MainToServer::Subscribe(no) => {
                act.inner
                    .hub
                    .send(GetUser(no))
                    .into_actor(act)
                    .then(|res, _, ctx| {
                        if let Ok(Ok(user)) = res {
                            user.do_send(UserConnect::Subscribe(ctx.address()));
                        }

                        fut::ready(())
                    })
                    .wait(ctx);
            }
            MainToServer::Unsubscribe(no) => {
                act.inner
                    .hub
                    .send(GetUser(no))
                    .into_actor(act)
                    .then(|res, _, ctx| {
                        if let Ok(Ok(user)) = res {
                            user.do_send(UserDisconnect::Unsubscribe(ctx.address()));
                        }

                        fut::ready(())
                    })
                    .wait(ctx);
            }
            MainToServer::Update => {
                act.inner.user.do_send(Update);
            }
        }
    }
}

impl Main {
    pub fn new(user: Addr<User>, hub: Addr<Hub>) -> Main {
        Main { user, hub }
    }
}
