use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, middleware, web, App, Either, HttpResponse, HttpServer, Responder};
use clap::Clap;
#[cfg(feature = "https")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use rand::Rng;
use serde_json::json;
use server::app_state::AppState;
#[cfg(feature = "https")]
use server::https::RedirectHttps;
use server::util;
use slog::Drain;
use slog_scope::GlobalLoggerGuard;
use std::fs::OpenOptions;
use std::path::PathBuf;

// todo: write help
#[derive(Clap)]
#[clap(version = "1.0.0-dev", author = "Jaeyong Sung")]
struct Opts {
    #[clap(short = 'i', long = "host", default_value = "0.0.0.0")]
    host: String,
    #[clap(short = 'p', long = "http-port", default_value = "80")]
    http_port: u16,
    #[cfg(feature = "https")]
    #[clap(long = "https-port", default_value = "443")]
    https_port: u16,
    #[cfg(feature = "https")]
    #[clap(long = "https-key", default_value = "key.pem", parse(from_os_str))]
    https_key: PathBuf,
    #[cfg(feature = "https")]
    #[clap(long = "https-cert", default_value = "cert.pem", parse(from_os_str))]
    https_cert: PathBuf,
    #[clap(short = 'l', long = "log", parse(from_os_str))]
    log: Option<PathBuf>,
    #[clap(
        short = 's',
        long = "static-files",
        default_value = "static",
        parse(from_os_str)
    )]
    static_files: PathBuf,
    #[clap(
        short = 'v',
        long = "verbose",
        default_value = "4",
        parse(from_occurrences)
    )]
    verbose: usize,
}

// todo: move to other file
#[get("/")]
async fn index(id: Identity, data: web::Data<AppState>) -> impl Responder {
    if let Some(id) = id.identity() {
        let handlebars = data.get_handlebars();
        let body = handlebars
            .render("main.hbs", &json!({ "user_id": id }))
            .unwrap();
        HttpResponse::Ok().body(body)
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("index.hbs", &json!({})).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/res/{file:.*}")]
async fn resource(data: web::Data<AppState>, web::Path(file): web::Path<String>) -> impl Responder {
    let resources = data.get_resources();
    if let Some(body) = resources.get(&file) {
        Either::A(HttpResponse::Ok().body(body))
    } else {
        Either::B(HttpResponse::NotFound())
    }
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

#[cfg(feature = "https")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
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
            .service(index)
            .service(resource)
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .bind_openssl(format!("{}:{}", opts.host, opts.https_port), builder)?
    .run()
    .await
}

#[cfg(not(feature = "https"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
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
            .service(index)
            .service(resource)
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .run()
    .await
}
