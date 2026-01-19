#![warn(clippy::indexing_slicing)]
#![warn(clippy::large_types_passed_by_value)]
#![feature(trim_prefix_suffix)]
#![feature(nonpoison_rwlock)]
#![feature(nonpoison_mutex)]
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
