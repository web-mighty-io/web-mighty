use std::time::Duration;

pub mod room;
pub mod server;
pub mod session;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);
