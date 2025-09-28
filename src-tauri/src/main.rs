// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;

use tedbot_lib::{
	config::{Cache, DEFAULT_CACHE},
	twitch,
};

mod logs;
mod tauri;

fn main() {
	logs::setup_logging();

	let def_cache = DEFAULT_CACHE.deref();
	if let Ok(disk_cache) = Cache::read()
		&& !def_cache.equal_scope(&disk_cache)
	{
		tracing::debug!("Changed scopes detected. Forgetting old token");
		_ = twitch::auth::forget_token();
	}

	if let Err(e) = def_cache.save() {
		tracing::warn!("Couldn't save current disk version: {e}");
	};

	tauri::run()
}
