// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;

use tedbot_lib::{
	config::{ARGS, Cache, DEFAULT_CACHE},
	twitch,
};
use tracing::Level;

mod tauri;

fn main() {
	let dbg = ARGS.debug;

	let lvl = match dbg {
		true => Level::DEBUG,
		false => Level::INFO,
	};

	let sub = tracing_subscriber::fmt().with_max_level(lvl).finish();
	if let Err(err) = tracing::subscriber::set_global_default(sub) {
		eprintln!("Error setting up logger: {}", err)
	}

	let def_cache = DEFAULT_CACHE.deref();
	if let Ok(disk_cache) = Cache::read()
		&& !def_cache.equal_scope(&disk_cache)
	{
		_ = twitch::auth::forget_token();
	}

	if let Err(e) = def_cache.save() {
		tracing::warn!("Couldn't save current disk version: {e}");
	};

	tauri::run()
}
