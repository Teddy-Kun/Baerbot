#![feature(trim_prefix_suffix)]
#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]

#[deny(clippy::unwrap_used)]
pub mod config;
pub mod error;
pub mod os_color;
pub mod twitch;
pub mod utils;
