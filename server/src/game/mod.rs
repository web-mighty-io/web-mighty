use std::time::Duration;

pub mod room;
pub mod server;
pub mod session;
pub mod user;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
const RECONNECTION_TIME: Duration = Duration::from_secs(60);
