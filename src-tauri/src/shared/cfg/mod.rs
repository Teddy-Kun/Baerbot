use super::data::SimpleResponse;
use cli::get_args;
use eyre::{Result, eyre};
use file::CfgFile;
use std::{
	fs::File,
	path::{Path, PathBuf},
	sync::Arc,
};

mod cli;
mod file;

trait SharedAttributes {
	fn get_username(&self) -> Option<Arc<str>>;
	fn get_tts_model(&self) -> Option<PathBuf>;
	fn get_debug(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct Config {
	debug: bool,
	username: Arc<str>,
	token_file: Option<Arc<str>>,
	pub counters: Arc<[Arc<str>]>,
	pub simple_responses: Arc<[SimpleResponse]>,
	pub tts_model: Option<Arc<Path>>,
	pub tts_chance: f64,
}

impl Config {
	pub fn get_username(&self) -> Arc<str> {
		self.username.clone()
	}

	pub fn get_token_file(&self) -> Option<Arc<str>> {
		self.token_file.clone()
	}

	pub fn debug(&self) -> bool {
		self.debug
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

	if !cfg_path.exists() {
		File::create(&cfg_path)?;
	}

	let cfg = CfgFile::from_file(cfg_path.as_ref())?;

	let owned_tts_model = args.get_tts_model().or(cfg.get_tts_model());
	let tts_model: Option<Arc<Path>> = match owned_tts_model {
		None => None,
		Some(p) => Some(Arc::from(p.as_ref())),
	};

	let tts_chance = match tts_model {
		None => 0.0,
		Some(_) => cfg.tts_chance.unwrap_or(0.0),
	};

	match args.get_username().or(cfg.get_username()) {
		None => Err(eyre!("No username provided")),
		Some(username) => Ok(Config {
			debug: args.get_debug() || cfg.get_debug(),
			username,
			token_file: args.token_file,
			counters: cfg.counters.unwrap_or(Arc::new([])),
			simple_responses: cfg.simple_responses.unwrap_or(Arc::new([])),
			tts_model,
			tts_chance,
		}),
	}
}
