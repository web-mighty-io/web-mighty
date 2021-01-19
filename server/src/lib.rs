#![allow(clippy::type_complexity)]
//! This is an mighty card game server.

mod actor;
mod app_state;
mod config;
mod db;
mod error;
mod https;
mod service;
mod util;

mod constant {
    use std::time::Duration;

    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
    pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
    // const LAST_ACTIVITY_INTERVAL: Duration = Duration::from_secs(30);
    // const CHECK_ACTIVITY_INTERVAL: Duration = Duration::from_secs(15);
    pub const RECONNECTION_TIME: Duration = Duration::from_secs(10);
    pub const ABSENT_TIME: Duration = Duration::from_secs(300);

    // const MAX_CHAT_HISTORY: usize = 50;

    pub const TOKEN_VALID_DURATION: Duration = Duration::from_secs(24 * 60 * 60);
}

mod dev {
    pub use crate::actor::session::*;
    pub use crate::constant::*;
    pub use crate::error::{Error, Result};
    pub use crate::util::*;
    pub use crate::{bail, ensure, err, ignore, try_ctx};
    pub use actix_web::http::StatusCode;
    pub use types::*;

    use r2d2_postgres::postgres::NoTls;
    use r2d2_postgres::PostgresConnectionManager;

    pub type Pool = r2d2_postgres::r2d2::Pool<PostgresConnectionManager<NoTls>>;
    pub type PgConfig = r2d2_postgres::postgres::Config;
}

pub mod prelude {
    pub use crate::db::*;
    pub use crate::dev::*;
}

pub mod internal {
    use crate::app_state::AppState;
    use crate::config::Config;
    use crate::dev::*;
    use crate::https::RedirectHttps;
    use crate::service::{config_services, p404};
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use actix_web::middleware::Logger;
    use actix_web::{web, App, HttpServer};
    use clap::Clap;
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

    #[cfg(not(tarpaulin_include))]
    async fn main_https(conf: Config) -> std::io::Result<()> {
        let private_key = conf.get_private_key();
        let _guard = conf.get_logger();
        let pg_config = conf.get_pg_config();
        let serve_path = conf.serve_path.clone();
        let host = conf.get_host();
        let http_port = conf.port;
        let https_port = conf.https.as_ref().unwrap().port;
        let builder = conf.get_ssl_builder();
        let mail = conf.get_mail();
        let redirect = conf.https.as_ref().unwrap().redirect.unwrap_or(false);

        let state = AppState::new(to_absolute_path(serve_path), pg_config, mail);

        HttpServer::new(move || {
            App::new()
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&private_key)
                        .name("web-mighty-auth")
                        .secure(true),
                ))
                .wrap(RedirectHttps::new(http_port, https_port, redirect))
                .wrap(Logger::default())
                .app_data(state.clone())
                .configure(config_services)
                .default_service(web::to(p404))
        })
        .bind(format!("{}:{}", host, http_port))?
        .bind_openssl(format!("{}:{}", host, https_port), builder)?
        .run()
        .await
    }

    #[cfg(not(tarpaulin_include))]
    async fn main_http(conf: Config) -> std::io::Result<()> {
        let private_key = conf.get_private_key();
        let _guard = conf.get_logger();
        let pg_config = conf.get_pg_config();
        let serve_path = conf.serve_path.clone();
        let host = conf.get_host();
        let http_port = conf.port;
        let mail = conf.get_mail();

        let state = AppState::new(to_absolute_path(serve_path), pg_config, mail);

        HttpServer::new(move || {
            App::new()
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&private_key)
                        .name("web-mighty-auth")
                        .secure(true),
                ))
                .wrap(Logger::default())
                .app_data(state.clone())
                .configure(config_services)
                .default_service(web::to(p404))
        })
        .bind(format!("{}:{}", host, http_port))?
        .run()
        .await
    }

    #[cfg(not(tarpaulin_include))]
    pub async fn main() -> std::io::Result<()> {
        let opts: Opts = Opts::parse();
        let path = if let Some(path) = std::env::var_os("CONFIG") {
            PathBuf::from(path)
        } else {
            opts.config
        };
        let conf = Config::generate(path);

        if conf.https.is_some() {
            main_https(conf).await
        } else {
            main_http(conf).await
        }
    }
}

#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => {
        anyhow::anyhow!($msg).into()
    };
    ($err:expr $(,)?) => {
        anyhow::anyhow!($err).into()
    };
    ($code:expr, $msg:literal $(,)?) => {
        $crate::error::Error($code, anyhow::anyhow!($msg))
    };
    ($code:expr, $err:expr $(,)?) => {
        $crate::error::Error($code, anyhow::anyhow!($err))
    };
    ($code:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::Error($code, anyhow::anyhow!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::err!($msg))
    };
    ($err:expr $(,)?) => {
        return Err($crate::err!($err))
    };
    ($code:expr, $msg:literal $(,)?) => {
        return Err($crate::err!($code, $msg))
    };
    ($code:expr, $err:expr $(,)?) => {
        return Err($crate::err!($code, $err))
    };
    ($code:expr, $fmt:expr, $($arg:tt)*) => {
        return Err($crate::err!($code, $fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        $crate::ensure!($cond, $crate::error::error!("condition failed"))
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err($crate::err!($msg));
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err($crate::err!($err));
        }
    };
    ($cond:expr, $code:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err($crate::err!($code, $msg));
        }
    };
    ($cond:expr, $code:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err($crate::err!($code, $err));
        }
    };
    ($cond:expr, $code:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err($crate::err!($code, $fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! try_ctx {
    ($exp:expr, $ctx:expr) => {
        match $exp {
            Ok(x) => x,
            _ => {
                $ctx.stop();
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! ignore {
    ($exp:expr) => {
        match $exp {
            Ok(x) => x,
            _ => return,
        }
    };
}
