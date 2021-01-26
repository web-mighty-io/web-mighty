use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use types::{ObserveToClient, ObserveToServer};

pub struct Observe;

impl SessionTrait for Observe {
    type Sender = ObserveToServer;

    fn tag() -> &'static str {
        "observe"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ObserveToClient = serde_json::from_str(&*msg).unwrap();
        match msg {
            ObserveToClient::Room(info) => ("room_info", JsValue::from_serde(&info).unwrap()),
            ObserveToClient::Game(state) => ("game_state", JsValue::from_serde(&state).unwrap()),
        }
    }
}

#[wasm_bindgen]
pub fn observe_on(tag: String, callback: Function) {
    OBSERVE.with(move |observe| observe.borrow().on(tag, callback));
}
