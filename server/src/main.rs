use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use clap::Clap;
use deadpool_postgres::Pool;
use rand::Rng;
use serde::Deserialize;
use server::app_state::AppState;
use server::handlers::{config, p404};
use server::util;
use slog::Drain;
use slog_scope::GlobalLoggerGuard;
use std::fs::OpenOptions;
use std::path::PathBuf;
#[cfg(feature = "https")]
use {
    openssl::ssl::{SslAcceptor, SslFiletype, SslMethod},
    server::https::RedirectHttps,
};

#[derive(Clap)]
#[clap(version = "1.0.0-dev", about = "The Mighty Mighty Card Game Server")]
struct Opts {
    #[clap(
        short = 'i',
        long = "host",
        default_value = "0.0.0.0",
        about = "host of this server"
    )]
    host: String,
    #[clap(
        short = 'p',
        long = "http-port",
        default_value = "80",
        about = "port to run http server"
    )]
    http_port: u16,
    #[cfg(feature = "https")]
    #[clap(
        long = "https-port",
        default_value = "443",
        about = "port to run https server"
    )]
    https_port: u16,
    #[cfg(feature = "https")]
    #[clap(
        long = "https-key",
        default_value = "key.pem",
        parse(from_os_str),
        about = "private key file to run https"
    )]
    https_key: PathBuf,
    #[cfg(feature = "https")]
    #[clap(
        long = "https-cert",
        default_value = "cert.pem",
        parse(from_os_str),
        about = "certification file to run https"
    )]
    https_cert: PathBuf,
    #[clap(
        short = 'l',
        long = "log",
        parse(from_os_str),
        about = "file to log (stdout when not used)"
    )]
    log: Option<PathBuf>,
    #[clap(
        short = 's',
        long = "static-files",
        default_value = "static",
        parse(from_os_str),
        about = "location to static files"
    )]
    static_files: PathBuf,
    #[clap(
        short = 'v',
        long = "verbose",
        default_value = "4",
        parse(from_occurrences),
        about = "verbose output"
    )]
    verbose: usize,
    #[clap(
        short = 'e',
        long = "env",
        parse(from_os_str),
        about = ".env file path for postgres db connection (./.env for default)"
    )]
    dotenv: Option<PathBuf>,
}

fn set_log(opts: &Opts) -> GlobalLoggerGuard {
    let drain = if let Some(log_path) = &opts.log {
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
        .filter_level(slog::Level::from_usize(opts.verbose).unwrap_or(slog::Level::Info))
        .fuse();
    let logger = slog::Logger::root(drain, slog::o!());
    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    guard
}

// todo: change the way of generating private key
//       reason: the key changes every time server starts
fn generate_private_key() -> [u8; 32] {
    rand::thread_rng().gen::<[u8; 32]>()
}

#[derive(Deserialize)]
struct Config {
    addr: String,
    pg: deadpool_postgres::Config,
}

impl Config {
    fn new() -> Config {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))
            .unwrap();
        cfg.try_into().unwrap()
    }
}

// todo: make configurable (dotenv)
fn make_db_pool() -> Pool {
    let cfg = Config::new();
    cfg.pg.create_pool(tokio_postgres::NoTls).unwrap()
}

fn load_dotenv(opts: &Opts) {
    match &opts.dotenv {
        Some(path) => {
            dotenv::from_path(path).ok();
        }
        None => {
            dotenv::dotenv().ok();
        }
    }
}

#[cfg(feature = "https")]
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    load_dotenv(&opts);
    let _guard = set_log(&opts);
    let state = AppState::new(util::to_absolute_path(opts.static_files));
    let private_key = generate_private_key();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(opts.https_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(opts.https_cert).unwrap();
    let http_port = opts.http_port;
    let https_port = opts.https_port;

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("web-mighty-auth")
                    .secure(true),
            ))
            .wrap(RedirectHttps::new(http_port, https_port))
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .data(make_db_pool())
            .configure(config)
            .default_service(web::to(p404))
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .bind_openssl(format!("{}:{}", opts.host, opts.https_port), builder)?
    .run()
    .await
}

#[cfg(not(feature = "https"))]
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    load_dotenv(&opts);
    let _guard = set_log(&opts);
    let state = AppState::new(util::to_absolute_path(opts.static_files));
    let private_key = generate_private_key();

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("web-mighty-auth")
                    .secure(true),
            ))
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .data(make_db_pool())
            .configure(config)
            .default_service(web::to(p404))
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .run()
    .await
}
