use crate::prelude::*;
use actix::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

pub struct Mail {
    pub smtp: SmtpTransport,
}

impl Actor for Mail {
    type Context = Context<Self>;
}

#[derive(Clone, Message)]
#[rtype(result = "Result<()>")]
pub struct Send(lettre::Message);

impl Handler<Send> for Mail {
    type Result = Result<()>;

    fn handle(&mut self, msg: Send, _: &mut Self::Context) -> Self::Result {
        self.smtp.send(&msg.0)?;
        Ok(())
    }
}

impl Mail {
    pub fn new(username: String, password: String, host: String) -> Mail {
        let cred = Credentials::new(username, password);
        Mail {
            smtp: SmtpTransport::relay(&*host).unwrap().credentials(cred).build(),
        }
    }
}
