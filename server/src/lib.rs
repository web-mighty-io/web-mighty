#![allow(clippy::type_complexity)]
//! This is an mighty card game server.

use std::time::Duration;

pub mod actor;
pub mod app_state;
pub mod config;
#[cfg(feature = "https")]
pub mod https;
pub mod service;
pub mod session;
pub mod util;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
const LAST_ACTIVITY_INTERVAL: Duration = Duration::from_secs(30);
const CHECK_ACTIVITY_INTERVAL: Duration = Duration::from_secs(15);
const RECONNECTION_TIME: Duration = Duration::from_secs(10);
const ABSENT_TIME: Duration = Duration::from_secs(300);

const MAX_CHAT_HISTORY: usize = 50;
