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

	#[arg(long, env, help = "Path to the config file")]
	pub config: Option<PathBuf>,
}

impl SharedAttributes for Args {
	fn get_username(&self) -> Option<Arc<str>> {
		self.username.clone()
	}
}

pub fn get_args() -> Args {
	Args::parse()
}
