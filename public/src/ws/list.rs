use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use types::{ListToClient, ListToServer};

pub struct List;

impl SessionTrait for List {
    type Sender = ListToServer;

    fn tag() -> &'static str {
        "list"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ListToClient = serde_json::from_str(&*msg).unwrap();
        ("list", JsValue::from_serde(&msg).unwrap())
    }
}

#[wasm_bindgen]
pub fn list_on(tag: String, callback: Function) {
    LIST.with(move |list| list.borrow().on(tag, callback));
}

#[wasm_bindgen]
pub fn list_subscribe(room_id: &JsValue) {
    LIST.with(move |list| {
        list.borrow()
            .send(ListToServer::Subscribe(room_id.into_serde().unwrap()))
    });
}

#[wasm_bindgen]
pub fn list_unsubscribe(room_id: &JsValue) {
    LIST.with(move |list| {
        list.borrow()
            .send(ListToServer::Unsubscribe(room_id.into_serde().unwrap()))
    });
}
