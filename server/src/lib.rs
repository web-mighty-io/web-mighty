#![allow(clippy::type_complexity)]
#![feature(into_future)]
//! This is an mighty card game server.

pub mod actor;
pub mod app_state;
pub mod config;
#[cfg(feature = "https")]
pub mod https;
pub mod service;
pub mod session;
pub mod util;
