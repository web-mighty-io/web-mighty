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
    pub postgres: DpConfig,
    // host of server (default: localhost)
    pub host: Option<String>,
    // port of server
    pub port: u16,
    // https configuration (default: http only)
    pub https: Option<Https>,
    // path to log (default: stdout)
    pub log_path: Option<String>,
    // log verbose (default: 4)
    pub verbose: Option<usize>,
    // path to static files
    pub serve_path: String,
    // secret key for auth (default: random)
    // note: generate through `openssl rand -hex 16`
    pub secret: Option<String>,
    // mail configuration
    pub mail: Mail,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Https {
    // port for serving https
    pub port: u16,
    // key file for https
    pub key: String,
    // cert file for https
    pub cert: String,
    // enable redirect http to https (default: false)
    pub redirect: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
}

impl Config {
    pub fn generate(path: PathBuf) -> Config {
        let mut s = ::config::Config::new();
        s.merge(File::from(path).required(false)).unwrap();
        s.merge(Environment::new()).unwrap();
        s.try_into().unwrap()
    }

    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or_else(|| "localhost".to_owned())
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

    pub fn get_pg_config(&self) -> PgConfig {
        self.postgres.get_pg_config().unwrap().into()
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
        actor::Mail::new(
            self.mail.username.clone(),
            self.mail.password.clone(),
            self.mail.host.clone(),
        )
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
