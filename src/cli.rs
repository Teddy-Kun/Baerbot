use clap::{CommandFactory, Parser};
use clap_config::ClapConfig;
use color_eyre::eyre::Result;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::warn;

#[derive(ClapConfig, Clone, Parser, Debug, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long, env, help = "Your Twitch username")]
	pub username: Option<Arc<str>>,

	#[arg(long, env, help = "Path to the config file")]
	pub config: Option<Arc<str>>,
}

#[derive(Debug, Clone)]
pub struct Config {
	pub username: Arc<str>,
}

impl Config {
	pub fn get() -> Result<Self> {
		let matches = <Args as CommandFactory>::command().get_matches();

		let mut warn_on_config_read_fail = false;

		let config_path = match matches.get_one::<Arc<str>>("config") {
			Some(path) => {
				// warn that reading the config file failed, if we manually specified a path
				warn_on_config_read_fail = true;
				Some(path.clone())
			}
			None => match dirs::config_dir() {
				None => None,
				Some(mut path) => {
					path.push("tedbot.toml");
					path.to_str().map(Arc::from)
				}
			},
		};

		let config = match config_path {
			None => None,
			Some(config_path) => match std::fs::read_to_string(config_path.as_ref()) {
				Ok(config_str) => Some(toml::from_str(&config_str)?),
				Err(e) => {
					if warn_on_config_read_fail {
						warn!("Error reading config file: {}", e);
					}
					None
				}
			},
		};

		let args = Args::from_merged(matches, config);

		let config = Config {
			username: args.username.ok_or(eyre!("Missing Username"))?,
		};

		Ok(config)
	}
}
