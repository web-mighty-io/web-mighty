use crate::ws::session::{Context, SessionTrait};
use types::{MainToClient, MainToServer};

pub struct Main;

impl SessionTrait for Main {
    type Sender = MainToServer;

    fn tag() -> &'static str {
        "main"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) {
        let _: MainToClient = serde_json::from_str(&*msg).unwrap();
    }
}
