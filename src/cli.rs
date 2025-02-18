use clap::{CommandFactory, Parser};
use clap_config::ClapConfig;
use color_eyre::eyre::Result;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::warn;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleResponse {
	pub trigger: Arc<str>,
	pub response: Arc<str>,
}

#[derive(ClapConfig, Clone, Debug, Parser, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(short, long, env, help = "Your Twitch username")]
	pub username: Option<Arc<str>>,

	#[arg(
		long,
		env,
		help = "Will use this file for storing key data, instead of the OS keyring"
	)]
	pub token_file: Option<Arc<str>>,

	#[arg(long, env, help = "Path to the config file")]
	pub config: Option<Arc<str>>,
}

impl From<Args> for ArgsConfig {
	fn from(args: Args) -> Self {
		ArgsConfig {
			config: args.config.clone(),
			token_file: args.token_file.clone(),
			username: args.username.clone(),
		}
	}
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
struct CfgFile {
	pub args: Option<Args>,
	pub counters: Option<Arc<[Arc<str>]>>,
	pub simple_responses: Option<Arc<[SimpleResponse]>>,
	pub tts_chance: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Config {
	pub username: Arc<str>,
	pub token_file: Option<Arc<str>>,
	pub counters: Arc<[Arc<str>]>,
	pub simple_response: Arc<[SimpleResponse]>,
	pub tts_chance: f64,
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

		let config: Option<CfgFile> = match config_path {
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

		let cfg_args: Option<ArgsConfig> = match config.clone() {
			None => None,
			Some(c) => c.args.map(|a| a.into()),
		};

		let args = Args::from_merged(matches, cfg_args);

		let tts_chance = match &config {
			None => 0.0,
			Some(c) => c.tts_chance.unwrap_or(0.0),
		};

		let counters = match &config {
			None => Arc::from([]),
			Some(c) => c.counters.clone().unwrap_or(Arc::from([])),
		};

		let simple_response = match config {
			None => Arc::from([]),
			Some(c) => c.simple_responses.unwrap_or(Arc::from([])),
		};

		let config = Config {
			username: args.username.ok_or(eyre!("Missing Username"))?,
			token_file: args.token_file,
			simple_response,
			tts_chance,
			counters,
		};

		Ok(config)
	}
}
