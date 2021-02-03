//! # Web Mighty Server
//!
//! This is a web-mighty server crate. This uses `actix-web` to serve & `actix` for handling
//! web sockets. It uses `postgresql` to save data can serve https. Also, it uses handlebars
//! for html template.
//!
//! ## Routes
//!
//! ### Get
//!
//! | path | logged in | guest |
//! |:----:|:---------:|:-----:|
//! | `/` | move to `/main` | welcome page |
//! | `/main` | dashboard page | move to `/` |
//! | `/res/{file}` | don't check | get the resource of {file} |
//! todo

#![allow(clippy::type_complexity)]

mod actor;
mod app_state;
mod config;
mod db;
pub mod error;
mod middlewares;
mod service;

/// # Constant module
///
/// This module contains constants used in this crate. By defining in one place, it is easy to
/// manipulate.
mod constant {
    use std::time::Duration;

    /// Sends ping to client every `HEARTBEAT_INTERVAL`.
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);

    /// After no return of pong for `CLIENT_TIMEOUT`, it is disconnected.
    pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);

    /// If disconnected connection reconnects in `RECONNECTION_TIME`, client is not offline.
    pub const RECONNECTION_TIME: Duration = Duration::from_secs(10);

    /// If user doesn't have feedback during `ABSENT_TIME`, user is absent.
    pub const ABSENT_TIME: Duration = Duration::from_secs(300);

    /// Token is valid during `TOKEN_VALID_DURATION`.
    pub const TOKEN_VALID_DURATION: Duration = Duration::from_secs(24 * 60 * 60);
}

/// # Dev module
///
/// This module contains useful things can be used in this crate. Put `use crate::dev::*` to
/// shorten the code.
mod dev {
    pub use crate::constant::*;
    pub use crate::error::{Error, Result};
    pub use crate::{bail, ensure, err, ignore, try_ctx};
    pub use actix_web::http::StatusCode;
    pub use regex;
    pub use types::*;

    use r2d2_postgres::postgres::NoTls;
    use r2d2_postgres::PostgresConnectionManager;

    /// Postgresql Pool type managed by r2d2
    pub type Pool = r2d2_postgres::r2d2::Pool<PostgresConnectionManager<NoTls>>;

    /// Postgresql configuration type
    pub type PgConfig = r2d2_postgres::postgres::Config;
}

/// # Prelude module
///
/// This module is used for tests and examples.
pub mod prelude {
    pub use crate::db::*;
    pub use crate::dev::*;
}

/// # Internal module
///
/// This module is used for `main.rs` to make `main.rs` short and easier to code.
pub mod internal {
    use crate::app_state::AppState;
    use crate::config::Config;
    use crate::middlewares::https::RedirectHttps;
    use crate::service::{config_services, p404};
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use actix_web::middleware::Logger;
    use actix_web::{web, App, HttpServer};
    use clap::Clap;
    use std::path::PathBuf;

    /// Configuration for this server
    ///
    /// - `config`: configuration file location (defaults to `server.toml`)
    #[derive(Clap)]
    #[clap(version = "1.0.0-dev", about = "The Mighty Mighty Card Game Server")]
    struct Opts {
        #[clap(
            short = 'c',
            long = "config",
            parse(from_os_str),
            about = "configuration file path (json, toml, yaml, hjson, ini files supported) (defaults to find server.*)"
        )]
        config: Option<PathBuf>,
    }

    /// Main function with https enabled
    ///
    /// Gets values from `conf` and serve
    #[cfg(not(tarpaulin_include))]
    async fn main_https(conf: Config) -> std::io::Result<()> {
        let http_port = conf.port;
        let https_port = conf.https.as_ref().unwrap().port;
        let host = conf.host.clone();
        let mail = conf.get_mail();
        let serve_path = conf.serve_path.clone();
        let ssl_builder = conf.get_ssl_builder();
        let pg_config = conf.get_pg_config();
        let private_key = conf.secret.clone();
        let redirect = conf.https.as_ref().unwrap().redirect;

        let state = AppState::new(serve_path, pg_config, mail);

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
        .bind_openssl(format!("{}:{}", host, https_port), ssl_builder)?
        .run()
        .await
    }

    /// Main function with http only
    ///
    /// Gets value from `conf` and serve
    #[cfg(not(tarpaulin_include))]
    async fn main_http(conf: Config) -> std::io::Result<()> {
        let host = conf.host.clone();
        let http_port = conf.port;
        let mail = conf.get_mail();
        let serve_path = conf.serve_path.clone();
        let pg_config = conf.get_pg_config();
        let private_key = conf.secret.clone();

        let state = AppState::new(serve_path, pg_config, mail);

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

    /// Main function used in outer code
    ///
    /// Call this function to start the server. This reads the configuration from
    /// file & environment and starts the server
    #[cfg(not(tarpaulin_include))]
    pub async fn main() -> std::io::Result<()> {
        let opts: Opts = Opts::parse();
        let path = opts
            .config
            .or_else(|| std::env::var_os("CONFIG_PATH").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("server.toml"));

        let conf = Config::builder().add_file(path).add_env().build();

        if conf.https.is_some() {
            main_https(conf).await
        } else {
            main_http(conf).await
        }
    }
}

/// Similar to `anyhow::err` macro but returns `crate::error::Error`
///
/// # Examples
///
/// ```
/// use server::prelude::*;
///
/// let e = err!("This is an error");
/// let e = err!(StatusCode::NOT_FOUND, "This is an error");
/// let e = err!(StatusCode::BAD_REQUEST, "error occurred: {}", 123);
/// ```
#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => {
        $crate::error::Error::from($crate::error::_anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::error::Error::from($crate::error::_anyhow::anyhow!($err))
    };
    ($code:expr, $msg:literal $(,)?) => {
        $crate::error::Error($code, $crate::error::_anyhow::anyhow!($msg))
    };
    ($code:expr, $err:expr $(,)?) => {
        $crate::error::Error($code, $crate::error::_anyhow::anyhow!($err))
    };
    ($code:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::Error($code, $crate::error::_anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// Similar to `anyhow::bail` macro but returns `crate::error::Error`
///
/// # Examples
///
/// ```
/// use server::prelude::*;
///
/// fn a() -> Result<()> {
///     bail!("This is an error");
/// }
///
/// fn b() -> Result<()> {
///     bail!(StatusCode::NOT_FOUND, "This is an error");
/// }
///
/// fn c() -> Result<()> {
///     bail!(StatusCode::BAD_REQUEST, "error occurred: {}", 123);
/// }
/// ```
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

/// Similar to `anyhow::ensure` macro but returns `crate::error::Error`
///
/// # Examples
///
/// ```
/// use server::prelude::*;
///
/// fn a(x: u32) -> Result<()> {
///     ensure!(x == 1, "This is an error");
///     Ok(())
/// }
///
/// fn b(y: String) -> Result<()> {
///     ensure!(y.len() == 4, StatusCode::NOT_FOUND, "This is an error");
///     Ok(())
/// }
///
/// fn c(z: f32) -> Result<()> {
///     ensure!(z < 1.23, StatusCode::BAD_REQUEST, "error occurred: {}", z);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        $crate::ensure!($cond, $crate::err!("condition failed"))
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

/// It tries to unwrap `Result`, when it can't, it would stop context of actor
///
/// # Examples
///
/// ```
/// use actix::prelude::*;
/// use server::prelude::*;
///
/// struct A;
///
/// impl Actor for A {
///     type Context = Context<Self>;
///
///     fn started(&mut self, ctx: &mut Self::Context) {
///         let res = Err(err!("Hello"));
///         let _: u32 = try_ctx!(res, ctx);
///     }
/// }
/// ```
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

/// It unwraps the `Result`, however if it is error, it just ignores and return.
///
/// # Examples
///
/// ```
/// use actix::prelude::*;
/// use server::prelude::*;
///
/// struct A;
///
/// impl Actor for A {
///     type Context = Context<Self>;
///
///     fn started(&mut self, ctx: &mut Self::Context) {
///         let res = Err(err!("Hello"));
///         let _: u32 = ignore!(res);
///     }
/// }
/// ```
#[macro_export]
macro_rules! ignore {
    ($exp:expr) => {
        match $exp {
            Ok(x) => x,
            _ => return,
        }
    };
    ($exp:expr, $ret:expr) => {
        match $exp {
            Ok(x) => x,
            _ => return $ret,
        }
    };
}

/// # Path module
///
/// Useful functions for path manipulation.
pub mod path {
    use std::env;
    use std::path::{Path, PathBuf};

    /// This compresses the input path.
    /// `Path::join` just pushes second path to first one.
    /// Therefore joining `/hello/..` and `world` results to `/hello/../world`
    ///
    /// This function compresses the result of joined path.
    /// `.` will be removed and `..` will make the path to go parent.
    ///
    /// If input is absolute path, `/..` will be ignored.
    /// If input is relative path, `..` in front will be remain same.
    ///
    /// # Examples
    ///
    /// ```
    /// use server::path::compress;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(compress("/../world/./"), PathBuf::from("/world"));
    /// assert_eq!(compress("hello/../../world"), PathBuf::from("../world"));
    /// ```
    pub fn compress<P: AsRef<Path>>(from: P) -> PathBuf {
        let from = from.as_ref();
        let mut path = PathBuf::new();
        let is_absolute = from.is_absolute();

        for i in from.iter() {
            match &*i.to_string_lossy() {
                "." => {}
                ".." => {
                    if let Some(parent) = path.parent() {
                        path = parent.to_path_buf();
                    } else if !is_absolute {
                        path = path.join("..")
                    }
                }
                _ => {
                    path = path.join(i);
                }
            }
        }

        path
    }

    /// Changes the path to absolute path.
    /// If input path is relative, it concat with current directory path.
    /// If input path is absolute, it returns input.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use server::path::to_absolute_path;
    /// use std::env;
    /// use std::path::PathBuf;
    ///
    /// env::set_current_dir("/hello");
    /// assert_eq!(to_absolute_path("world"), PathBuf::from("/hello/world"));
    /// assert_eq!(to_absolute_path("/world"), PathBuf::from("/world"));
    /// ```
    pub fn to_absolute_path<P: AsRef<Path>>(from: P) -> PathBuf {
        let from = from.as_ref();
        if from.is_relative() {
            compress(env::current_dir().unwrap().join(from))
        } else {
            from.to_path_buf()
        }
    }

    /// Join two paths and compress
    ///
    /// # Examples
    ///
    /// ```
    /// use server::path::join;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(join("hello/..", "world"), PathBuf::from("world"));
    /// assert_eq!(join("hello/..", "./world"), PathBuf::from("world"));
    /// assert_eq!(join("hello/./world.json", "../world"), PathBuf::from("hello/world"));
    /// ```
    pub fn join<P: AsRef<Path>, Q: AsRef<Path>>(base: P, path: Q) -> PathBuf {
        let base = base.as_ref();
        let path = path.as_ref();
        compress(base.join(path))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn compress_test() {
            assert_eq!(compress("/hello/../world/./"), PathBuf::from("/world"));
            assert_eq!(compress("/../world/./"), PathBuf::from("/world"));
            assert_eq!(compress("hello/../../world"), PathBuf::from("../world"));
        }
    }
}
