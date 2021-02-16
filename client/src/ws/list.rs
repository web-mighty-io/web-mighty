use crate::prelude::*;
use crate::ws::session::{Context, Session, SessionTrait};
use types::{ListToClient, ListToServer, RoomId};

pub struct ListSession;

impl SessionTrait for ListSession {
    type Sender = ListToServer;

    fn tag() -> &'static str {
        "list"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ListToClient = serde_json::from_str(&*msg).unwrap();
        match msg {
            ListToClient::Room(room_info) => ("", JsValue::from_serde(&room_info).unwrap()),
            ListToClient::RoomList(room_list) => ("", JsValue::from_serde(&room_list).unwrap()),
        }
    }
}

#[wasm_bindgen]
pub struct List {
    session: Session<ListSession>,
}

#[wasm_bindgen]
impl List {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<List> {
        Ok(List {
            session: ListSession.start()?,
        })
    }

    pub fn on(&self, tag: String, callback: Function) {
        self.session.on(tag, callback);
    }

    pub fn subscribe(&self, room_id: RoomId) {
        self.session.send(ListToServer::Subscribe(room_id));
    }

    pub fn unsubscribe(&self, room_id: RoomId) {
        self.session.send(ListToServer::Unsubscribe(room_id));
    }

    pub fn get_room_list(&self, user_min: u32, user_max: u32) {
        self.session.send(ListToServer::GetRoomList {
            user_num: (user_min, user_max),
        });
    }
}
