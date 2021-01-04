#![allow(clippy::type_complexity)]
//! This is an mighty card game server.

pub mod actor;
pub mod app_state;
pub mod config;
pub mod error;
#[cfg(feature = "https")]
pub mod https;
pub mod service;
pub mod session;
pub mod util;

pub mod constant {
    use std::time::Duration;

    // const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
    // const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
    // const LAST_ACTIVITY_INTERVAL: Duration = Duration::from_secs(30);
    // const CHECK_ACTIVITY_INTERVAL: Duration = Duration::from_secs(15);
    pub const RECONNECTION_TIME: Duration = Duration::from_secs(10);
    pub const ABSENT_TIME: Duration = Duration::from_secs(300);

    // const MAX_CHAT_HISTORY: usize = 50;

    pub const TOKEN_VALID_DURATION: Duration = Duration::from_secs(24 * 60 * 60);
}

pub mod prelude {
    pub use crate::constant::*;
    pub use crate::error::{Error, Result};
    pub use crate::util::*;
    pub use crate::{bail, ensure, err};
    pub use actix_web::http::StatusCode;
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
