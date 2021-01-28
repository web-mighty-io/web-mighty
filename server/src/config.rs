//! # Main Configuration
//!
//! - `postgres`: postgres connection configuration
//!   * `user`: username (defaults to `admin`)
//!   * `password`: password (defaults to `admin`)
//!   * `dbname`: database name (defaults to `web_mighty`)
//!   * `host`: host (defaults to `127.0.0.1`)
//!   * `port`: port (defaults to `5432`)
//!   * **see `deadpool_postgres::Config` for more configuration**
//! - `host`: hostname of server (defaults to `localhost`)
//! - `port`: port to use for http connection (defaults to `80`)
//! - `https`: https configuration
//!   * `port`: port to use for https connection (defaults to `443`)
//!   * `key`: key file for ssl
//!   * `cert`: cert file for ssl
//!   * `redirect`: redirect http connection to https (defaults to `false`)
//! - `log_path`: path to write log to (stdout if not present)
//! - `verbose`: verbose level of log (1 ~ 6) (defaults to `4`)
//! - `serve_path`: path to `static` directory (defaults `static`) **This wouldn't be necessary
//!                 if you use docker of the way in `README.md`**
//! - `secret`: random key for token (defaults to random) **note: generate through
//!             `openssl rand -hex 16`**
//! - `mail`: mail configuration
//!   * `from`: email to send from (defaults to `{username}@{host}` or `{username}`)
//!   * `username`: username to mail server (defaults to `admin`)
//!   * `password`: password to mail server (defaults to `admin`)
//!   * `host`: host of mail server (defaults to `localhost:587`)
//!
//! # Examples
//!
//! ## `.toml` example
//!
//! ```toml
//! host = "0.0.0.0"
//! port = "8080"
//! verbose = "2"
//! secret = "a093c76bd2c5f4e7dff6360c78bcb57a"
//!
//! [postgres]
//! user = "postgres"
//! password = "secret"
//! host = "0.0.0.0"
//! dbname = "postgres"
//!
//! [https]
//! port = "8443"
//! key = "./key.pem"
//! cert = "./cert.pem"
//! redirect = true
//!
//! [mail]
//! from = "noreply@example.com"
//! username = "admin"
//! password = "secret"
//! host = "0.0.0.0"
//! ```
//!
//! ## Environment example
//!
//! ```env
//! HOST="0.0.0.0"
//! PORT="8080"
//! VERBOSE="2"
//! SECRET="a093c76bd2c5f4e7dff6360c78bcb57a"
//!
//! POSTGRES__USER="postgres"
//! POSTGRES__PASSWORD="secret"
//! POSTGRES__HOST="0.0.0.0"
//! POSTGRES__DBNAME="postgres"
//!
//! HTTPS__PORT="8443"
//! HTTPS__KEY="./key.pem"
//! HTTPS__CERT="./cert.pem"
//! HTTPS__REDIRECT="true"
//!
//! MAIL__FROM="noreply@example.com"
//! MAIL__USERNAME="admin"
//! MAIL__PASSWORD="secret"
//! MAIL__HOST="0.0.0.0"
//! ```

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

/// Using `deadpoool_postgres::Config` just for serde
type DpConfig = deadpool_postgres::Config;

/// Main configuration struct
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub postgres: Option<DpConfig>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub https: Option<Https>,
    pub log_path: Option<String>,
    pub verbose: Option<usize>,
    pub serve_path: Option<String>,
    pub secret: Option<String>,
    pub mail: Option<Mail>,
}

/// Https configuration struct
#[derive(Debug, Clone, Deserialize)]
pub struct Https {
    pub port: Option<u16>,
    pub key: String,
    pub cert: String,
    pub redirect: Option<bool>,
}

/// Mail configuration struct
#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    pub from: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
}

impl Config {
    /// Generate `Config` from the given path.
    ///
    /// If `server.toml` doesn't exists, it gets from env variables.
    /// If env variable doesn't exists, it uses default values.
    pub fn generate(path: PathBuf) -> Config {
        let mut s = ::config::Config::new();
        s.merge(File::from(path).required(false)).unwrap();
        s.merge(Environment::new().separator("__")).unwrap();
        s.try_into().unwrap()
    }

    /// Function to get host
    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or_else(|| "localhost".to_owned())
    }

    /// Function to get port
    pub fn get_port(&self) -> u16 {
        self.port.clone().unwrap_or(80)
    }

    /// Function to get private key
    /// If not present, it would generate random value.
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

    /// Function to get serve path
    pub fn get_serve_path(&self) -> String {
        self.serve_path.clone().unwrap_or_else(|| "static".to_owned())
    }

    /// Function to get postgres configuration
    pub fn get_pg_config(&self) -> PgConfig {
        let mut conf = self
            .postgres
            .as_ref()
            .map_or_else(PgConfig::new, |c| c.get_pg_config().unwrap().into());

        if conf.get_hosts().is_empty() {
            conf.host("127.0.0.1");
        }

        if conf.get_ports().is_empty() {
            conf.port(5432);
        }

        if conf.get_dbname().is_none() {
            conf.dbname("web_mighty");
        }

        if conf.get_user().is_none() {
            conf.user("admin");
        }

        if conf.get_password().is_none() {
            conf.password("admin");
        }

        conf
    }

    /// Function to get logger (using `slog`)
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

    /// Function to get mail configuration
    pub fn get_mail(&self) -> actor::Mail {
        let mail = self.mail.clone().unwrap_or(Mail {
            from: None,
            username: None,
            password: None,
            host: None,
        });

        let host = mail.host.unwrap_or_else(|| "localhost".to_owned());
        let username = mail.username.unwrap_or_else(|| "admin".to_owned());
        let password = mail.password.unwrap_or_else(|| "admin".to_owned());
        let from = mail.from.unwrap_or_else(|| {
            if username.contains('@') {
                username.clone()
            } else {
                format!("{}@{}", username, host)
            }
        });

        actor::Mail::new(from, username, password, host, hex::encode(self.get_private_key()))
    }

    /// Function to get https port (assuming https is enabled)
    pub fn get_https_port(&self) -> u16 {
        self.https.as_ref().unwrap().port.unwrap_or(443)
    }

    /// Function to get rediect option (assuming https is enabled)
    pub fn get_redirect(&self) -> bool {
        self.https.as_ref().unwrap().redirect.unwrap_or(false)
    }

    /// Function to get ssl builder (assuming https is enabled)
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
