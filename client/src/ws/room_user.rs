use crate::prelude::*;
use crate::ws::session::{Context, Session, SessionTrait};
use types::{RoomUserToClient, RoomUserToServer};

pub struct UserSession;

impl SessionTrait for UserSession {
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
pub struct User {
    session: Session<UserSession>,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<User> {
        Ok(User {
            session: UserSession.start()?,
        })
    }

    pub fn on(&self, tag: String, callback: Function) {
        self.session.on(tag, callback);
    }
}
