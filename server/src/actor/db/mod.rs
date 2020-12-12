use crate::actor::db::game::{change_rating, get_rating, make_game, save_state};
pub use crate::actor::db::game::{ChangeRatingForm, GetRatingForm, MakeGameForm, Rating, SaveStateForm};
use crate::actor::db::user::{
    add_user, change_info, check_email, check_id, delete, get_email, get_info, login, regenerate_token, register,
};
pub use crate::actor::db::user::{
    AddUserForm, ChangeInfoForm, CheckEmailForm, CheckIdForm, DeleteForm, GetEmailForm, GetInfoForm, LoginForm,
    RegenerateTokenForm, RegisterForm, UserInfo,
};
use actix::prelude::*;
use deadpool_postgres::Pool;
use error::Result;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use uuid::Uuid;

pub mod error;
mod game;
mod user;

const TOKEN_VALID_DURATION: Duration = Duration::from_secs(24 * 60 * 60);

pub struct Database {
    pub pool: Pool,
}

impl Actor for Database {
    type Context = Context<Self>;
}

macro_rules! impl_handler {
    ($msg:ty, $res:ty, $func:ident) => {
        impl Handler<$msg> for Database {
            type Result = $res;

            fn handle(&mut self, msg: $msg, ctx: &mut Self::Context) -> Self::Result {
                let (tx, rx): (Sender<Self::Result>, Receiver<Self::Result>) = mpsc::channel();
                $func(msg, self.pool.clone())
                    .into_actor(self)
                    .then(move |res, _, _| {
                        let _ = tx.send(res);
                        fut::ready(())
                    })
                    .wait(ctx);
                rx.recv().unwrap()
            }
        }
    };
}

impl_handler!(AddUserForm, Result<()>, add_user);
impl_handler!(ChangeInfoForm, Result<()>, change_info);
impl_handler!(CheckIdForm, Result<bool>, check_id);
impl_handler!(CheckEmailForm, Result<bool>, check_email);
// impl_handler!(DeleteForm, Result<()>, delete);
impl_handler!(LoginForm, Result<u32>, login);
impl_handler!(GetEmailForm, Result<String>, get_email);
impl_handler!(GetInfoForm, Result<UserInfo>, get_info);
impl_handler!(RegenerateTokenForm, Result<Uuid>, regenerate_token);
impl_handler!(RegisterForm, Result<()>, register);
impl_handler!(ChangeRatingForm, Result<()>, change_rating);
impl_handler!(GetRatingForm, Result<Vec<Rating>>, get_rating);
impl_handler!(MakeGameForm, Result<()>, make_game);
impl_handler!(SaveStateForm, Result<()>, save_state);

#[derive(Deserialize, Serialize, Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct Delete(pub u32, pub DeleteForm);

impl Handler<Delete> for Database {
    type Result = Result<()>;

    fn handle(&mut self, msg: Delete, ctx: &mut Self::Context) -> Self::Result {
        let (tx, rx): (Sender<Self::Result>, Receiver<Self::Result>) = mpsc::channel();
        delete(msg.0, msg.1, self.pool.clone())
            .into_actor(self)
            .then(move |res, _, _| {
                let _ = tx.send(res);
                fut::ready(())
            })
            .wait(ctx);
        rx.recv().unwrap()
    }
}

impl Database {
    pub fn new(pool: Pool) -> Database {
        Database { pool }
    }
}
