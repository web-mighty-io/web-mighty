#![allow(clippy::type_complexity)]
//! This is an mighty card game server.

mod actor;
mod app_state;
mod config;
mod error;
#[cfg(feature = "https")]
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
    pub use crate::actor::group::*;
    pub use crate::actor::session::*;
    pub use crate::constant::*;
    pub use crate::error::{Error, Result};
    pub use crate::util::*;
    pub use crate::{bail, ensure, err, ignore, try_ctx};
    pub use actix_web::http::StatusCode;
}

pub mod prelude {
    pub use crate::dev::*;
    // todo
}

pub mod internal {
    use crate::app_state::AppState;
    use crate::config::Config;
    use crate::dev::*;
    #[cfg(feature = "https")]
    use crate::https::RedirectHttps;
    use crate::service::{config, p404};
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

    #[cfg(feature = "https")]
    #[cfg(not(tarpaulin_include))]
    pub async fn main() -> std::io::Result<()> {
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
        let mail = conf.get_mail();

        let state = AppState::new(to_absolute_path(public), pool, mail);

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
    pub async fn main() -> std::io::Result<()> {
        let opts: Opts = Opts::parse();
        let conf = Config::from_path(opts.config);
        let private_key = conf.private_key();
        let _guard = conf.logger();
        let pool = conf.db_pool();
        let public = conf.server.public.clone();
        let host = conf.server.host.clone();
        let http_port = conf.server.port;
        let mail = conf.get_mail();

        let state = AppState::new(to_absolute_path(public), pool, mail);

        HttpServer::new(move || {
            App::new()
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&private_key)
                        .name("web-mighty-auth")
                        .secure(true),
                ))
                .wrap(Logger::default())
                .app_data(state.clone())
                .configure(config)
                .default_service(web::to(p404))
        })
        .bind(format!("{}:{}", host, http_port))?
        .run()
        .await
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
