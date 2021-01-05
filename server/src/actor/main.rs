use crate::actor::hub::GetUser;
use crate::actor::user::{UserConnect, UserDisconnect, UserStatus};
use crate::actor::{Hub, User, UserNo};
use crate::dev::*;
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use serde::{Deserialize, Serialize};

pub struct Main {
    user: Addr<User>,
    hub: Addr<Hub>,
    connection: Option<Addr<Connection<Session<Main>>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Message)]
#[rtype(result = "()")]
pub struct MainSend {
    pub user_no: UserNo,
    pub status: UserStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MainReceive {
    Subscribe(UserNo),
    Unsubscribe(UserNo),
    Update,
}

impl SessionTrait for Main {
    type Receiver = MainSend;

    fn started(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserConnect::Main(ctx.address()));
    }

    fn stopped(act: &mut Session<Self>, ctx: &mut WebsocketContext<Session<Self>>) {
        act.inner.user.do_send(UserDisconnect::Main(ctx.address()));
    }

    fn receive(act: &mut Session<Self>, msg: String, ctx: &mut WebsocketContext<Session<Self>>) {
        let msg: MainReceive = serde_json::from_str(&*msg).unwrap();
        match msg {
            MainReceive::Subscribe(no) => {
                if let Ok(Some(user)) = send(act, ctx, act.inner.hub.clone(), GetUser(no)) {
                    user.do_send(UserConnect::Subscribe(ctx.address()));
                }
            }
            MainReceive::Unsubscribe(no) => {
                if let Ok(Some(user)) = send(act, ctx, act.inner.hub.clone(), GetUser(no)) {
                    user.do_send(UserDisconnect::Unsubscribe(ctx.address()));
                }
            }
            _ => {}
        }
        if let Some(connection) = &act.inner.connection {
            connection.do_send(Update);
        }
    }
}

impl Main {
    pub fn new(user: Addr<User>, hub: Addr<Hub>) -> Main {
        Main {
            user,
            hub,
            connection: None,
        }
    }
}
