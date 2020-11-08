use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChangeName {
    pub name: String,
}

pub struct Room {
    name: String,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Handler<ChangeName> for Room {
    type Result = ();

    fn handle(&mut self, msg: ChangeName, _: &mut Self::Context) -> Self::Result {
        self.name = msg.name;
    }
}

impl Room {
    fn new() -> Room {
        Room { name: "".to_owned() }
    }
}
