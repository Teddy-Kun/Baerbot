// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::Level;

mod tauri;

fn main() {
	let sub = tracing_subscriber::fmt()
		.with_max_level(Level::INFO)
		.finish();
	if let Err(err) = tracing::subscriber::set_global_default(sub) {
		eprintln!("Error setting up logger: {}", err)
	}
	// tedbot_lib::tts::tts();
	tauri::run()
}
