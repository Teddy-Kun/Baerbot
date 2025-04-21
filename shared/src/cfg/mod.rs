use crate::data::SimpleResponse;
use cli::get_args;
use eyre::{Result, eyre};
use file::CfgFile;
use std::{path::Path, sync::Arc};

mod cli;
mod file;

trait SharedAttributes {
	fn get_username(&self) -> Option<Arc<str>>;
}

#[derive(Clone, Debug)]
pub struct Config {
	username: Arc<str>,
	token_file: Option<Arc<str>>,
	pub counters: Arc<[Arc<str>]>,
	pub simple_responses: Arc<[SimpleResponse]>,
	pub tts_chance: f64,
}

impl Config {
	pub fn get_username(&self) -> Arc<str> {
		self.username.clone()
	}

	pub fn get_token_file(&self) -> Option<Arc<str>> {
		self.token_file.clone()
	}
}

pub fn get_merged_cfg() -> Result<Config> {
	let args = get_args();
	let cfg_path: Arc<Path> = match args.config.clone() {
		Some(path) => Arc::from(path),
		None => match dirs::config_dir() {
			None => return Err(eyre!("Could not determine config directory")),
			Some(mut path) => {
				path.push("tedbot.toml");
				Arc::from(path)
			}
		},
	};

	let cfg = CfgFile::from_file(cfg_path.as_ref())?;

	match args.get_username().or(cfg.get_username()) {
		None => Err(eyre!("No username provided")),
		Some(username) => Ok(Config {
			username: username,
			token_file: args.token_file,
			counters: cfg.counters.unwrap_or(Arc::new([])),
			simple_responses: cfg.simple_responses.unwrap_or(Arc::new([])),
			tts_chance: cfg.tts_chance.unwrap_or(0.0),
		}),
	}
}
