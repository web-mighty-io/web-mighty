use crate::ws::session::{Context, SessionTrait};
use types::{RoomUserToClient, RoomUserToServer};

pub struct Room;

impl SessionTrait for Room {
    type Sender = RoomUserToServer;

    fn tag() -> &'static str {
        "room"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) {
        let _: RoomUserToClient = serde_json::from_str(&*msg).unwrap();
    }
}
