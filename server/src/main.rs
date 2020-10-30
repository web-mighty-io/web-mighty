use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, middleware, web, App, Either, HttpResponse, HttpServer, Responder};
use clap::Clap;
use rand::Rng;
use serde_json::json;
use server::app_state::AppState;
use server::util;
use slog::Drain;
use std::env;
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
    #[clap(long = "https-port", default_value = "443")]
    https_port: u16,
    #[clap(long = "https")]
    https: bool,
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
        Either::A(format!("Hello {}!", id))
    } else {
        let handlebars = data.get_handlebars();
        let body = handlebars.render("index.hbs", &json!({})).unwrap();
        Either::B(HttpResponse::Ok().body(body))
    }
}

#[get("/res/{file:.*}")]
async fn resource(data: web::Data<AppState>, web::Path(file): web::Path<String>) -> impl Responder {
    log::info!("{}", file);
    let resources = data.get_resources();
    if let Some(body) = resources.get(&file) {
        Either::A(HttpResponse::Ok().body(body))
    } else {
        Either::B(HttpResponse::NotFound())
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

    let static_files = if opts.static_files.is_relative() {
        util::compress(env::current_dir().unwrap().join(opts.static_files))
    } else {
        opts.static_files
    };
    let state = AppState::new(&static_files);

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
            .app_data(state.clone())
            .service(index)
            .service(resource)
    })
    .bind(format!("{}:{}", opts.host, opts.http_port))?
    .run()
    .await
}
