use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use types::{RoomUserToClient, RoomUserToServer};

pub struct User;

impl SessionTrait for User {
    type Sender = RoomUserToServer;

    fn tag() -> &'static str {
        "room"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) {
        let _: RoomUserToClient = serde_json::from_str(&*msg).unwrap();
    }
}

#[wasm_bindgen]
pub fn room_change_name(name: String) {
    USER.with(move |user| user.borrow().send(RoomUserToServer::ChangeName(name)));
}

#[wasm_bindgen]
pub fn room_change_rule(rule: &JsValue) {
    let rule = rule.into_serde().unwrap();
    USER.with(move |user| user.borrow().send(RoomUserToServer::ChangeRule(rule)));
}
