use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use types::{RoomUserToClient, RoomUserToServer};

pub struct User;

impl SessionTrait for User {
    type Sender = RoomUserToServer;

    fn tag() -> &'static str {
        "room"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: RoomUserToClient = serde_json::from_str(&*msg).unwrap();
        match msg {
            RoomUserToClient::Room(info) => ("room_info", JsValue::from_serde(&info).unwrap()),
            RoomUserToClient::Game(state) => ("game_state", JsValue::from_serde(&state).unwrap()),
        }
    }
}

#[wasm_bindgen]
pub fn room_on(tag: String, callback: Function) {
    USER.with(move |user| user.borrow().on(tag, callback));
}

#[wasm_bindgen]
pub fn room_start_game() {
    USER.with(move |user| user.borrow().send(RoomUserToServer::Start));
}

#[wasm_bindgen]
pub fn room_change_name(name: String) {
    USER.with(move |user| user.borrow().send(RoomUserToServer::ChangeName(name)));
}

#[wasm_bindgen]
pub fn room_change_rule(rule: &JsValue) {
    USER.with(move |user| {
        user.borrow()
            .send(RoomUserToServer::ChangeRule(rule.into_serde().unwrap()))
    });
}

#[wasm_bindgen]
pub fn room_send_command(command: &JsValue) {
    USER.with(move |user| {
        user.borrow()
            .send(RoomUserToServer::Command(command.into_serde().unwrap()))
    });
}
