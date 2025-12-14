#![warn(clippy::indexing_slicing)]
#![feature(trim_prefix_suffix)]
#![feature(nonpoison_rwlock)]
#![feature(sync_nonpoison)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use baerbot_lib::{
	config::{CONFIG, Config},
	twitch,
};

mod logs;
mod tauri;

fn setup_config() {
	let def_config = Config::default();
	let mut disk_config = CONFIG.write();
	if !def_config.equal_scope(&disk_config) {
		tracing::debug!("Changed scopes detected. Forgetting old token");
		_ = twitch::auth::forget_token();

		disk_config.scopes = def_config.scopes;
		if let Err(e) = disk_config.save() {
			tracing::warn!("Couldn't save current disk version: {e}");
		}
	}
}

fn main() {
	logs::setup_logging();
	setup_config();

	tauri::run()
}
