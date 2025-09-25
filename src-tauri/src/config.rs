use std::sync::{Arc, LazyLock};

use clap::Parser;
use clap_config::ClapConfig;
use serde::{Deserialize, Serialize};

// global config
pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(ClapConfig, Clone, Debug, Parser, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[arg(
		long,
		env,
		help = "Will use this file for storing key data, instead of the OS keyring. DO NOT USE! ONLY USEFUL FOR DEBUGGING! MASSIVE SECURITY ISSUE"
	)]
	pub token_file: Option<Arc<str>>,

	#[arg(short, long, env, help = "Enable debug mode")]
	pub debug: bool,
}

pub struct Config {}
