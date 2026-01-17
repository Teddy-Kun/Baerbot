#![warn(clippy::indexing_slicing)]
#![feature(trim_prefix_suffix)]
#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]
#![feature(never_type)]

#[deny(clippy::unwrap_used)]
pub mod config;
pub mod error;
pub mod obs;
pub mod os_color;
pub mod tts;
pub mod twitch;
pub mod utils;
