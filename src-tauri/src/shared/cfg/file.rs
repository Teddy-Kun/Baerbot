use eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use crate::shared::data::SimpleResponse;
use super::SharedAttributes;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CfgFile {
	username: Option<Arc<str>>,
	pub counters: Option<Arc<[Arc<str>]>>,
	pub simple_responses: Option<Arc<[SimpleResponse]>>,
	pub tts_chance: Option<f64>,
}

impl SharedAttributes for CfgFile {
	fn get_username(&self) -> Option<Arc<str>> {
		self.username.clone()
	}
}

impl CfgFile {
	pub fn from_file(path: &Path) -> Result<Self> {
		match std::fs::read_to_string(path) {
			Ok(config_str) => Ok(toml::from_str(&config_str)?),
			Err(e) => Err(eyre!("Error reading config file: {}", e)),
		}
	}
}
