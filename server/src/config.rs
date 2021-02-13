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
//! - `outer_host`: host from outside (if proxy, it could be different) (defaults to `host`)
//! - `port`: port to use for http connection (defaults to `80`)
//! - `https`: https configuration
//!   * `port`: port to use for https connection (defaults to `443`)
//!   * `key`: key file for ssl (defaults to `key.pem`)
//!   * `cert`: cert file for ssl (defaults to `cert.pem`)
//!   * `redirect`: redirect http connection to https (defaults to `false`)
//! - `log_path`: path to write log to (stdout if not present)
//! - `verbose`: verbose level of log (1 ~ 6) (defaults to `4`)
//! - `serve_path`: path to `public` directory (defaults `public`) **This wouldn't be necessary
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
//! outer_host = "example.com"
//! port = "8080"
//! verbose = "2"
//! secret = "a093c76bd2c5f4e7dff6360c78bcb57a"
//! log_path = "server.log"
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
//! OUTER_HOST="example.com"
//! PORT="8080"
//! VERBOSE="2"
//! SECRET="a093c76bd2c5f4e7dff6360c78bcb57a"
//! LOG_PATH="server.log"
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
use crate::path::{join, to_absolute_path};
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
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

/// Using `deadpoool_postgres::Config` just for serde
type DpConfig = deadpool_postgres::Config;

#[derive(Debug, Clone)]
pub struct Builder {
    builders: Vec<(PathBuf, ConfigBuilder)>,
}

impl Builder {
    pub fn add_file<P: AsRef<Path>>(mut self, path: P) -> Builder {
        let path = to_absolute_path(path.as_ref().to_path_buf());
        self.builders.push((join(&path, ".."), ConfigBuilder::from_file(path)));
        self
    }

    pub fn add_env(mut self) -> Builder {
        let path = to_absolute_path(env::current_dir().unwrap());
        self.builders.push((path, ConfigBuilder::from_env()));
        self
    }

    pub fn build(&self) -> Config {
        let mut postgres = None;
        for (_, c) in self.builders.iter() {
            postgres = postgres.or_else(|| c.postgres.clone());
        }

        let mut host = None;
        for (_, c) in self.builders.iter() {
            host = host.or_else(|| c.host.clone());
        }
        let host = host.unwrap_or_else(|| "localhost".to_owned());

        let mut outer_host = None;
        for (_, c) in self.builders.iter() {
            outer_host = outer_host.or_else(|| c.outer_host.clone());
        }
        let outer_host = outer_host.unwrap_or_else(|| host.clone());

        let mut port = None;
        for (_, c) in self.builders.iter() {
            port = port.or(c.port)
        }
        let port = port.unwrap_or(80);

        let mut https_port = None;
        let mut https_key = None;
        let mut https_cert = None;
        let mut https_redirect = None;
        let mut is_https_used = false;

        for (p, c) in self.builders.iter() {
            if let Some(https) = &c.https {
                is_https_used = true;
                https_port = https_port.or(https.port);
                https_key = https_key.or_else(|| https.key.as_ref().map(|key| join(p, key)));
                https_cert = https_cert.or_else(|| https.cert.as_ref().map(|cert| join(p, cert)));
                https_redirect = https_redirect.or(https.redirect);
            }
        }

        let https = if is_https_used {
            Some(Https {
                port: https_port.unwrap_or(443),
                key: https_key.unwrap_or_else(|| to_absolute_path("key.pem")),
                cert: https_cert.unwrap_or_else(|| to_absolute_path("cert.pem")),
                redirect: https_redirect.unwrap_or(false),
            })
        } else {
            None
        };

        let log_path = (|| {
            for (p, c) in self.builders.iter() {
                if let Some(log_path) = &c.log_path {
                    return Some(join(p, log_path));
                }
            }

            None
        })();

        let verbose = (|| {
            for (_, c) in self.builders.iter() {
                if let Some(verbose) = &c.verbose {
                    return *verbose;
                }
            }

            4
        })();

        let drain = if let Some(log_path) = log_path {
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
            .filter_level(Level::from_usize(verbose).unwrap_or(slog::Level::Info))
            .fuse();
        let logger = Logger::root(drain, slog::o!());
        let guard = set_global_logger(logger);
        slog_stdlog::init().unwrap();

        let mut serve_path = None;
        for (p, c) in self.builders.iter() {
            serve_path = serve_path.or_else(|| c.serve_path.as_ref().map(|c| join(p, c)));
        }
        let serve_path = serve_path.unwrap_or_else(|| to_absolute_path("public"));

        let mut secret = None;
        for (_, c) in self.builders.iter() {
            secret = secret.or_else(|| {
                c.secret.as_ref().map(|s| {
                    let mut hasher = Sha256::new();
                    hasher.update(s);
                    let res = hasher.finalize();
                    Vec::from(&res[..])
                })
            });
        }
        let secret = secret.unwrap_or_else(|| (&mut rand::thread_rng()).sample_iter(Standard).take(32).collect());

        let mut mail_builder = MailBuilder::default();

        for (_, c) in self.builders.iter() {
            if let Some(mail) = &c.mail {
                mail_builder.from = mail_builder.from.or_else(|| mail.from.clone());
                mail_builder.username = mail_builder.username.or_else(|| mail.username.clone());
                mail_builder.password = mail_builder.password.or_else(|| mail.password.clone());
                mail_builder.host = mail_builder.host.or_else(|| mail.host.clone());
            }
        }

        let mail_host = mail_builder.host.unwrap_or_else(|| "localhost".to_owned());
        let username = mail_builder.username.unwrap_or_else(|| "admin".to_owned());
        let password = mail_builder.password.unwrap_or_else(|| "admin".to_owned());
        let from = mail_builder.from.unwrap_or_else(|| {
            if username.contains('@') {
                username.clone()
            } else {
                format!("{}@{}", username, mail_host)
            }
        });

        Config {
            postgres,
            host,
            outer_host,
            port,
            https,
            logger: guard,
            serve_path,
            secret,
            mail: Mail {
                from,
                username,
                password,
                host: mail_host,
            },
        }
    }
}

/// Main configuration builder
#[derive(Debug, Clone, Default, Deserialize)]
struct ConfigBuilder {
    postgres: Option<DpConfig>,
    host: Option<String>,
    outer_host: Option<String>,
    port: Option<u16>,
    https: Option<HttpsBuilder>,
    log_path: Option<String>,
    verbose: Option<usize>,
    serve_path: Option<String>,
    secret: Option<String>,
    mail: Option<MailBuilder>,
}

/// Https configuration builder
#[derive(Debug, Clone, Default, Deserialize)]
struct HttpsBuilder {
    port: Option<u16>,
    key: Option<String>,
    cert: Option<String>,
    redirect: Option<bool>,
}

/// Mail configuration builder
#[derive(Debug, Clone, Default, Deserialize)]
struct MailBuilder {
    from: Option<String>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
}

impl ConfigBuilder {
    /// Generate `ConfigBuilder` from the given path.
    pub fn from_file(path: PathBuf) -> ConfigBuilder {
        let mut s = ::config::Config::new();
        s.merge(File::from(path).required(false)).unwrap();
        s.try_into().unwrap()
    }

    /// Generate `ConfigBuilder` from environment variable
    pub fn from_env() -> ConfigBuilder {
        let mut s = ::config::Config::new();
        s.merge(Environment::new().separator("__")).unwrap();
        s.try_into().unwrap()
    }
}

/// Main configuration struct
pub struct Config {
    pub postgres: Option<DpConfig>,
    pub host: String,
    pub outer_host: String,
    pub port: u16,
    pub https: Option<Https>,
    pub logger: GlobalLoggerGuard,
    pub serve_path: PathBuf,
    pub secret: Vec<u8>,
    pub mail: Mail,
}

/// Https configuration struct
#[derive(Clone)]
pub struct Https {
    pub port: u16,
    pub key: PathBuf,
    pub cert: PathBuf,
    pub redirect: bool,
}

/// Mail configuration struct
#[derive(Clone)]
pub struct Mail {
    pub from: String,
    pub username: String,
    pub password: String,
    pub host: String,
}

impl Config {
    pub fn builder() -> Builder {
        Builder { builders: Vec::new() }
    }

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

    /// Function to get mail configuration
    pub fn get_mail(&self) -> actor::Mail {
        let mail = self.mail.clone();
        let host = if self.https.is_some() {
            format!("https://{}", self.outer_host)
        } else {
            format!("http://{}", self.outer_host)
        };

        actor::Mail::new(
            mail.from,
            mail.username,
            mail.password,
            mail.host,
            hex::encode(&self.secret),
            host,
        )
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
