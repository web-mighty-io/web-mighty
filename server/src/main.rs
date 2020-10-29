use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, middleware, App, HttpServer, Responder};
use clap::Clap;
use rand::Rng;
use slog::Drain;
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "1.0.0-dev", author = "Jaeyong Sung")]
struct Opts {
    #[clap(short = 'i', long = "host", default_value = "127.0.0.1")]
    host: String,
    #[clap(short = 'p', long = "http-port", default_value = "80")]
    http_port: u16,
    #[clap(long = "https-port", default_value = "443")]
    https_port: u16,
    #[clap(long = "https")]
    https: bool,
    #[clap(short = 'l', long = "log", parse(from_os_str))]
    log: Option<PathBuf>,
    #[clap(
        short = 'v',
        long = "verbose",
        parse(from_occurrences),
        default_value = "4"
    )]
    verbose: usize,
}

#[get("/")]
async fn index(id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        format!("Hello {}!", id)
    } else {
        format!("Hello Visitor!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let drain = if let Some(log_path) = opts.log {
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
    let _log = slog::Logger::root(drain, slog::o!());
    let _guard = slog_scope::set_global_logger(_log);
    slog_stdlog::init().unwrap();

    // todo: change the way of generating private key
    //       reason: the key changes every time server starts
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("web-mighty-auth")
                    .secure(true),
            ))
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .run()
    .await
}
