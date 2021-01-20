use crate::ws::list::List;
use crate::ws::main::Main;
use crate::ws::observe::Observe;
use crate::ws::room_user::User;
use crate::ws::session::{Session, SessionTrait};
use std::cell::RefCell;

thread_local! {
    pub static MAIN: RefCell<Session<Main>> = RefCell::new(Main.start().unwrap());
    pub static LIST: RefCell<Session<List>> = RefCell::new(List.start().unwrap());
    pub static OBSERVE: RefCell<Session<Observe>> = RefCell::new(Observe.start().unwrap());
    pub static USER: RefCell<Session<User>> = RefCell::new(User.start().unwrap());
}
