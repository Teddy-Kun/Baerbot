use clap::Parser;
use clap_config::ClapConfig;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

use super::SharedAttributes;

#[derive(ClapConfig, Clone, Debug, Parser, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[arg(short, long, env, help = "Your Twitch username")]
	username: Option<Arc<str>>,

	#[arg(
		long,
		env,
		help = "Will use this file for storing key data, instead of the OS keyring"
	)]
	pub token_file: Option<Arc<str>>,

	#[arg(long, env, help = "Path to the tts model (json)")]
	tts_model: Option<PathBuf>,

	#[arg(short, long, env, help = "Enable debug mode")]
	debug: Option<bool>,

	#[arg(long, env, help = "Path to the config file")]
	pub config: Option<PathBuf>,
}

impl SharedAttributes for Args {
	fn get_username(&self) -> Option<Arc<str>> {
		self.username.clone()
	}

	fn get_tts_model(&self) -> Option<PathBuf> {
		self.tts_model.clone()
	}

	fn get_debug(&self) -> bool {
		self.debug.unwrap_or(false)
	}
}

pub fn get_args() -> Args {
	Args::parse()
}
