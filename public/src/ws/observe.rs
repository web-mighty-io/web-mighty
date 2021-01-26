use crate::ws::session::{Context, SessionTrait};
use types::{ObserveToClient, ObserveToServer};
use wasm_bindgen::JsValue;

pub struct Observe;

impl SessionTrait for Observe {
    type Sender = ObserveToServer;

    fn tag() -> &'static str {
        "observe"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ObserveToClient = serde_json::from_str(&*msg).unwrap();
        ("todo", JsValue::from_serde(&msg).unwrap())
    }
}
