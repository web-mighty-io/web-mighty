use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use serde::Serialize;
use types::{MainToClient, MainToServer, UserNo, UserStatus};

pub struct Main;

#[derive(Debug, Clone, Serialize)]
struct Status {
    no: UserNo,
    status: UserStatus,
}

impl SessionTrait for Main {
    type Sender = MainToServer;

    fn tag() -> &'static str {
        "main"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: MainToClient = serde_json::from_str(&*msg).unwrap();
        match msg {
            MainToClient::UserStatus(no, status) => {
                ("user_status", JsValue::from_serde(&Status { no, status }).unwrap())
            }
            MainToClient::UserInfo(info) => ("user_info", JsValue::from_serde(&info).unwrap()),
        }
    }
}

#[wasm_bindgen]
pub fn main_on(tag: String, callback: Function) {
    MAIN.with(move |main| main.borrow().on(tag, callback));
}

#[wasm_bindgen]
pub fn main_update() {
    MAIN.with(|main| main.borrow().send(MainToServer::Update));
}

#[wasm_bindgen]
pub fn main_subscribe(user_no: &JsValue) {
    MAIN.with(move |main| {
        main.borrow()
            .send(MainToServer::Subscribe(user_no.into_serde().unwrap()))
    });
}

#[wasm_bindgen]
pub fn main_unsubscribe(user_no: &JsValue) {
    MAIN.with(move |main| {
        main.borrow()
            .send(MainToServer::Unsubscribe(user_no.into_serde().unwrap()))
    });
}
