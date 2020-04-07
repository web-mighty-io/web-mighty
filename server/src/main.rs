use actix_web::{get, App, HttpServer, Responder};
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "1.0.0-dev", author = "Jaeyong Sung")]
struct Opts {
    #[clap(short = "i", long = "host", default_value = "127.0.0.1")]
    host: String,
    #[clap(long = "http-port", default_value = "80")]
    http_port: u16,
    #[clap(long = "https-port", default_value = "443")]
    https_port: u16,
    #[clap(long = "https")]
    https: bool,
    #[clap(short = "l", long = "log", parse(from_os_str))]
    log: Option<PathBuf>,
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    HttpServer::new(|| App::new().service(index))
        .bind(format!("{}:{}", opts.host, opts.http_port))?
        .run()
        .await
}
