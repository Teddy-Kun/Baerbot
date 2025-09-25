// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tedbot_lib::config::ARGS;
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

	tauri::run()
}
