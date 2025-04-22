// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::Result;
use tedbot_lib::{start_service, start_ui};

#[tokio::main]
async fn main() -> Result<()> {
	start_ui();
	start_service().await
}
