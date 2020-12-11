use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use clap::Clap;
use server::app_state::AppState;
use server::config::Config;
#[cfg(feature = "https")]
use server::https::RedirectHttps;
use server::service::{config, p404};
use server::util;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "1.0.0-dev", about = "The Mighty Mighty Card Game Server")]
struct Opts {
    #[clap(
        short = 'c',
        long = "config",
        default_value = "server.toml",
        parse(from_os_str),
        about = ".toml configuration file path"
    )]
    config: PathBuf,
}

#[cfg(feature = "https")]
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let conf = Config::from_path(opts.config);
    let private_key = conf.private_key();
    let _guard = conf.logger();
    let pool = conf.db_pool();
    let public = conf.server.public.clone();
    let host = conf.server.host.clone();
    let http_port = conf.server.port;
    let https_port = conf.server.https.port;
    let builder = conf.ssl_builder();

    let state = AppState::new(util::to_absolute_path(public), pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("web-mighty-auth")
                    .secure(true),
            ))
            .wrap(RedirectHttps::new(http_port, https_port))
            .wrap(Logger::default())
            .app_data(state.clone())
            .data(pool.clone())
            .configure(config)
            .default_service(web::to(p404))
    })
    .bind(format!("{}:{}", host, http_port))?
    .bind_openssl(format!("{}:{}", host, https_port), builder)?
    .run()
    .await
}

#[cfg(not(feature = "https"))]
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let conf = Config::from_path(opts.config);
    let private_key = conf.private_key();
    let _guard = conf.logger();
    let pool = conf.db_pool();
    let public = conf.server.public.clone();
    let host = conf.server.host.clone();
    let http_port = conf.server.port;

    let state = AppState::new(util::to_absolute_path(public), pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("web-mighty-auth")
                    .secure(true),
            ))
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .data(pool.clone())
            .configure(config)
            .default_service(web::to(p404))
    })
    .bind(format!("{}:{}", host, http_port))?
    .run()
    .await
}
