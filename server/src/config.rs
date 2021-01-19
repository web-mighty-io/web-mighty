use crate::actor;
use crate::dev::*;
use config::{Environment, File};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use rand::distributions::Standard;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use slog::{Drain, Level, Logger};
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard};
use slog_term::{FullFormat, PlainDecorator, TermDecorator};
use std::fs::OpenOptions;
use std::path::PathBuf;

type DpConfig = deadpool_postgres::Config;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // postgres configuration
    pub postgres: Option<DpConfig>,
    // host of server (default: localhost)
    pub host: Option<String>,
    // port of server (default: 80)
    pub port: Option<u16>,
    // https configuration (default: http only)
    pub https: Option<Https>,
    // path to log (default: stdout)
    pub log_path: Option<String>,
    // log verbose (default: 4)
    pub verbose: Option<usize>,
    // path to static files (default: static)
    pub serve_path: Option<String>,
    // secret key for auth (default: random)
    // note: generate through `openssl rand -hex 16`
    pub secret: Option<String>,
    // mail configuration
    pub mail: Option<Mail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Https {
    // port for serving https (default: 443)
    pub port: Option<u16>,
    // key file for https
    pub key: String,
    // cert file for https
    pub cert: String,
    // enable redirect http to https (default: false)
    pub redirect: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    // username to mail server (default: admin)
    pub username: Option<String>,
    // password to mail server (default: main)
    pub password: Option<String>,
    // host to mail server (default: localhost:587)
    pub host: Option<String>,
}

impl Config {
    pub fn generate(path: PathBuf) -> Config {
        let mut s = ::config::Config::new();
        s.merge(File::from(path).required(false)).unwrap();
        s.merge(Environment::new().separator("__")).unwrap();
        s.try_into().unwrap()
    }

    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or_else(|| "localhost".to_owned())
    }

    pub fn get_port(&self) -> u16 {
        self.port.clone().unwrap_or(80)
    }

    pub fn get_private_key(&self) -> Vec<u8> {
        if let Some(secret) = &self.secret {
            let mut hasher = Sha256::new();
            hasher.update(secret);
            let res = hasher.finalize();
            Vec::from(&res[..])
        } else {
            (&mut rand::thread_rng()).sample_iter(Standard).take(32).collect()
        }
    }

    pub fn get_serve_path(&self) -> String {
        self.serve_path.clone().unwrap_or_else(|| "static".to_owned())
    }

    pub fn get_pg_config(&self) -> PgConfig {
        self.postgres.as_ref().map_or_else(
            || {
                let mut c = PgConfig::new();
                c.host("localhost");
                c
            },
            |c| c.get_pg_config().unwrap().into(),
        )
    }

    pub fn get_logger(&self) -> GlobalLoggerGuard {
        let drain = if let Some(log_path) = &self.log_path {
            let decorator = PlainDecorator::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(log_path)
                    .unwrap(),
            );
            let drain = FullFormat::new(decorator).build().fuse();
            Async::new(drain).build().fuse()
        } else {
            let decorator = TermDecorator::new().build();
            let drain = FullFormat::new(decorator).build().fuse();
            Async::new(drain).build().fuse()
        };

        let drain = drain
            .filter_level(Level::from_usize(self.verbose.unwrap_or(4)).unwrap_or(slog::Level::Info))
            .fuse();
        let logger = Logger::root(drain, slog::o!());
        let guard = set_global_logger(logger);
        slog_stdlog::init().unwrap();
        guard
    }

    pub fn get_mail(&self) -> actor::Mail {
        let mail = self.mail.clone().unwrap_or(Mail {
            username: None,
            password: None,
            host: None,
        });
        actor::Mail::new(
            mail.username.unwrap_or_else(|| "admin".to_owned()),
            mail.password.unwrap_or_else(|| "admin".to_owned()),
            mail.host.unwrap_or_else(|| "localhost:587".to_owned()),
        )
    }

    // # Https part
    //
    // from this part, is would assert that self.https is not none

    pub fn get_https_port(&self) -> u16 {
        self.https.as_ref().unwrap().port.unwrap_or(443)
    }

    pub fn get_redirect(&self) -> bool {
        self.https.as_ref().unwrap().redirect.unwrap_or(false)
    }

    pub fn get_ssl_builder(&self) -> SslAcceptorBuilder {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(self.https.as_ref().unwrap().key.clone(), SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(self.https.as_ref().unwrap().cert.clone())
            .unwrap();
        builder
    }
}
