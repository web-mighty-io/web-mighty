use crate::dev::*;
use actix::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

pub struct Mail {
    smtp: SmtpTransport,
    from: Mailbox,
    secret: String,
}

impl Actor for Mail {
    type Context = Context<Self>;
}

#[derive(Debug, Clone, Message, Serialize, Deserialize)]
#[rtype(result = "Result<()>")]
pub struct SendVerification {
    email: String,
    id: String,
    token: String,
}

impl Handler<SendVerification> for Mail {
    type Result = Result<()>;

    fn handle(&mut self, msg: SendVerification, _: &mut Self::Context) -> Self::Result {
        let token = encode(
            &Header::new(Algorithm::ES256),
            &msg,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        let msg = lettre::Message::builder()
            .from(self.from.clone())
            .to(msg.email.parse().unwrap())
            .subject("Finish signing up to web-mighty.io")
            .body(format!("{}{}", msg.id, token))?;
        // todo
        self.smtp.send(&msg)?;
        Ok(())
    }
}

impl Mail {
    pub fn new(from: String, username: String, password: String, host: String, secret: String) -> Mail {
        let cred = Credentials::new(username, password);
        Mail {
            smtp: SmtpTransport::relay(&*host).unwrap().credentials(cred).build(),
            from: from.parse().unwrap(),
            secret,
        }
    }
}
