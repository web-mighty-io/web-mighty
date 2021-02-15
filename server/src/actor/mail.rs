use crate::app_state::AppState;
use crate::dev::*;
use actix::prelude::*;
use actix_web::web;
use hyperx::header;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::SystemTime;

pub struct Mail {
    smtp: SmtpTransport,
    from: Mailbox,
    secret: String,
    host: String,
    app_state: Option<web::Data<AppState>>,
}

impl Actor for Mail {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetAppState(pub web::Data<AppState>);

impl Handler<SetAppState> for Mail {
    type Result = ();

    fn handle(&mut self, msg: SetAppState, _: &mut Self::Context) -> Self::Result {
        self.app_state = Some(msg.0);
    }
}

#[derive(Debug, Clone, Message, Serialize, Deserialize)]
#[rtype(result = "Result<()>")]
pub struct SendVerification {
    pub email: String,
    pub user_id: String,
    pub token: String,
    pub exp: usize,
}

impl Handler<SendVerification> for Mail {
    type Result = Result<()>;

    fn handle(&mut self, msg: SendVerification, _: &mut Self::Context) -> Self::Result {
        if let Some(app_state) = &self.app_state {
            let token = encode(
                &Header::new(Algorithm::HS256),
                &msg,
                &EncodingKey::from_secret(self.secret.as_ref()),
            )?;

            let body = app_state
                .render(
                    "mail.hbs",
                    &json!({
                        "host": self.host,
                        "token": token,
                        "user_id": msg.user_id,
                        "expire": SystemTime::now() + TOKEN_VALID_DURATION
                    }),
                )
                .unwrap();

            let msg = lettre::Message::builder()
                .from(self.from.clone())
                .to(msg.email.parse().unwrap())
                .subject("Finish your registration to Web Mighty")
                .header(header::ContentType::html())
                .body(body)
                .unwrap();

            self.smtp.send(&msg)?;
            Ok(())
        } else {
            bail!("mail is not initialized");
        }
    }
}

impl Mail {
    pub fn new(
        from: String,
        username: String,
        password: String,
        host: String,
        server_host: String,
        secret: String,
    ) -> Mail {
        let cred = Credentials::new(username, password);
        Mail {
            smtp: SmtpTransport::relay(&*host).unwrap().credentials(cred).build(),
            from: from.parse().unwrap(),
            secret,
            host: server_host,
            app_state: None,
        }
    }
}
