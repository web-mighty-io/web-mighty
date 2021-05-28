use crate::prelude::*;
use crate::ws::session::{Context, Session, SessionTrait};
use serde::Serialize;
use types::{MainToClient, MainToServer, UserNo, UserStatus};

pub struct MainSession;

#[derive(Debug, Clone, Serialize)]
struct Status {
    no: UserNo,
    status: UserStatus,
}

impl SessionTrait for MainSession {
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
        }
    }
}

#[wasm_bindgen]
pub struct Main {
    session: Session<MainSession>,
}

#[wasm_bindgen]
impl Main {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Main> {
        Ok(Main {
            session: MainSession.start()?,
        })
    }

    pub fn on(&self, tag: String, callback: Function) {
        self.session.on(tag, callback);
    }

    pub fn update(&self) {
        self.session.send(MainToServer::Update);
    }

    pub fn subscribe(&self, user_no: UserNo) {
        self.session.send(MainToServer::Subscribe(user_no));
    }

    pub fn unsubscribe(&self, user_no: UserNo) {
        self.session.send(MainToServer::Unsubscribe(user_no));
    }
}
