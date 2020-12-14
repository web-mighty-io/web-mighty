use actix::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub struct Mail {
    pub smtp: SmtpTransport,
}

impl Actor for Mail {
    type Context = Context<Self>;
}

impl Mail {
    pub fn new(username: String, password: String, host: String) -> Mail {
        let cred = Credentials::new(username, password);
        Mail {
            smtp: SmtpTransport::relay(&*host).unwrap().credentials(cred).build(),
        }
    }
}
