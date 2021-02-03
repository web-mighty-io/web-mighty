pub mod game;
pub mod user;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CREATE_USER_SQL: &str = include_str!("./sql/create_user.sql");
#[cfg(target_os = "windows")]
const CREATE_USER_SQL: &str = include_str!(".\\sql\\create_user.sql");

#[cfg(any(target_os = "linux", target_os = "macos"))]
const CREATE_ROOM_SQL: &str = include_str!("./sql/create_room.sql");
#[cfg(target_os = "windows")]
const CREATE_ROOM_SQL: &str = include_str!(".\\sql\\create_room.sql");

use crate::dev::*;

pub fn init(pool: Pool) -> Result<()> {
    let mut client = pool.get()?;
    client.simple_query(CREATE_USER_SQL)?;
    client.simple_query(CREATE_ROOM_SQL)?;
    Ok(())
}
