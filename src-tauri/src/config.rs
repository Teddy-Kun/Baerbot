use std::{
	fs,
	sync::{Arc, LazyLock, nonpoison::RwLock},
};

use clap::Parser;
use clap_config::ClapConfig;
use serde::{Deserialize, Serialize};
use twitch_oauth2::Scope;

use crate::{error::Error, utils::CFG_DIR_PATH};

// global config
pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);
pub static CONFIG: LazyLock<RwLock<Config>> =
	LazyLock::new(|| RwLock::new(Config::read_or_default()));

#[derive(ClapConfig, Clone, Debug, Parser, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[arg(
		long,
		env,
		help = "Will use this file for storing key data, instead of the OS keyring. DO NOT USE! ONLY USEFUL FOR DEBUGGING! MASSIVE SECURITY ISSUE"
	)]
	pub token_file: Option<Arc<str>>,

	#[arg(
		long,
		env,
		help = "Token won't be loaded or saved and login will be forgotten upon closing the application"
	)]
	pub temp_token: bool,

	#[arg(short, long, env, help = "Enable debug mode")]
	pub debug: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Config for the OBS Websocket stuff
pub struct ObsConfig {
	pub enable: bool,
	pub url: Arc<str>,
	pub port: u16,
	pub password: Option<Arc<str>>,
}

impl Default for ObsConfig {
	fn default() -> Self {
		Self {
			enable: false,
			url: Arc::from("localhost"),
			port: 4455,
			password: None,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub enable_redeems: bool,
	pub obs: ObsConfig,
	// used to track the scopes the token was last initialized with
	// if changed the token should be forgotten
	pub scopes: Vec<Scope>,
}

impl Config {
	pub fn save(&self) -> Result<(), Error> {
		let p = CFG_DIR_PATH.join("cache.toml");
		let s = toml::to_string_pretty(self)?;
		fs::write(p, s)?;
		Ok(())
	}

	fn read_or_default() -> Self {
		match Self::read() {
			Ok(s) => s,
			Err(e) => {
				tracing::warn!("Couldn't open config: {e}");
				let s = Self::default();
				if let Err(e) = s.save() {
					tracing::warn!("Couldn't save default config: {e}");
				}

				s
			}
		}
	}

	pub fn read() -> Result<Self, Error> {
		let p = CFG_DIR_PATH.join("cache.toml");
		let s = fs::read_to_string(p)?;
		let c = toml::from_str(s.as_str())?;
		Ok(c)
	}

	pub fn equal_scope(&self, other: &Self) -> bool {
		self.scopes.eq(&other.scopes)
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			enable_redeems: true,
			obs: ObsConfig::default(),
			scopes: vec![
				Scope::ChatEdit,
				Scope::ChatRead,
				Scope::ChannelReadRedemptions,
				Scope::ChannelManageRedemptions,
				Scope::ModeratorReadBannedUsers,
				Scope::ModeratorManageBannedUsers,
				Scope::ModeratorReadChatters,
				Scope::ModeratorReadVips,
				Scope::ChannelReadSubscriptions,
			],
		}
	}
}
