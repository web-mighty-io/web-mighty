use crate::ws::session::{Context, SessionTrait};
use types::{ObserveToClient, ObserveToServer};

pub struct Observe;

impl SessionTrait for Observe {
    type Sender = ObserveToServer;

    fn tag() -> &'static str {
        "observe"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) {
        let _: ObserveToClient = serde_json::from_str(&*msg).unwrap();
    }
}
