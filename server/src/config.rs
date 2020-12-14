use deadpool_postgres::Pool;
#[cfg(feature = "https")]
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use slog::Drain;
use slog_scope::GlobalLoggerGuard;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub postgres: deadpool_postgres::Config,
    pub server: Server,
    pub mail: Mail,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    #[cfg(feature = "https")]
    pub https: Https,
    pub log: Option<Log>,
    pub port: u16,
    pub public: String,
    pub secret: String,
}

#[cfg(feature = "https")]
#[derive(Deserialize)]
pub struct Https {
    pub cert: String,
    pub key: String,
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct Log {
    pub path: Option<String>,
    pub verbose: Option<usize>,
}

#[derive(Deserialize)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
}

impl Config {
    pub fn from_path(path: PathBuf) -> Config {
        toml::from_str(&*fs::read_to_string(&path).expect(&*format!("failed to load {:?}", path)))
            .expect(&*format!("failed to parse {:?}", path))
    }

    pub fn private_key(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.server.secret.clone());
        let res = hasher.finalize();
        Vec::from(&res[..])
    }

    pub fn logger(&self) -> GlobalLoggerGuard {
        let log = self.server.log.clone().unwrap_or(Log {
            path: None,
            verbose: Some(4),
        });
        let drain = if let Some(log_path) = &log.path {
            let decorator = slog_term::PlainDecorator::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(log_path)
                    .unwrap(),
            );
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            slog_async::Async::new(drain).build().fuse()
        } else {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            slog_async::Async::new(drain).build().fuse()
        };

        let drain = drain
            .filter_level(slog::Level::from_usize(log.verbose.unwrap_or(4)).unwrap_or(slog::Level::Info))
            .fuse();
        let logger = slog::Logger::root(drain, slog::o!());
        let guard = slog_scope::set_global_logger(logger);
        slog_stdlog::init().unwrap();
        guard
    }

    pub fn db_pool(&self) -> Pool {
        self.postgres.create_pool(tokio_postgres::NoTls).unwrap()
    }

    #[cfg(feature = "https")]
    pub fn ssl_builder(&self) -> SslAcceptorBuilder {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(self.server.https.key.clone(), SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(self.server.https.cert.clone())
            .unwrap();
        builder
    }
}
