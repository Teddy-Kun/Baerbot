use std::{
	fs,
	sync::{Arc, LazyLock, nonpoison::RwLock},
};

use clap::Parser;
use clap_config::ClapConfig;
use serde::{Deserialize, Serialize};
use specta::Type;
use twitch_oauth2::Scope;

use crate::{
	error::Error,
	tts::{TtsBackend, VoiceData},
	utils::CFG_DIR_PATH,
};

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
	pub enable_host: Option<bool>,
	pub enable_ws: Option<bool>,
	pub url: Option<Box<str>>,
	pub host_port: Option<u16>,
	pub ws_port: Option<u16>,
	pub password: Option<Arc<str>>,
}

impl Default for ObsConfig {
	fn default() -> Self {
		Self {
			enable_host: Some(false),
			enable_ws: Some(false),
			url: Some(Box::from("localhost")),
			host_port: Some(8564), // chosen at random
			ws_port: Some(4455),   // should be the OBS default one
			password: None,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub struct TtsConfig {
	pub backend: TtsBackend,
	pub voice: Option<VoiceData>,
}

impl Default for TtsConfig {
	fn default() -> Self {
		Self {
			backend: TtsBackend::System,
			voice: None,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub use_os_color: Option<bool>,
	pub custom_color: Option<Box<str>>,
	pub enable_redeems: Option<bool>,
	pub obs: Option<ObsConfig>,
	pub tts: Option<TtsConfig>,
	// used to track the scopes the token was last initialized with
	// if changed the token should be forgotten
	pub scopes: Option<Vec<Scope>>,
}

impl Config {
	pub fn save(&self) -> Result<(), Error> {
		let p = CFG_DIR_PATH.join("config.toml");
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
		let p = CFG_DIR_PATH.join("config.toml");
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
			use_os_color: Some(true),
			custom_color: None,
			enable_redeems: Some(true),
			obs: Some(ObsConfig::default()),
			tts: Some(TtsConfig::default()),
			scopes: Some(vec![
				Scope::ChatEdit,
				Scope::ChatRead,
				Scope::ChannelReadRedemptions,
				Scope::ChannelManageRedemptions,
				Scope::ModeratorReadBannedUsers,
				Scope::ModeratorManageBannedUsers,
				Scope::ModeratorReadChatters,
				Scope::ModeratorReadVips,
				Scope::ChannelReadSubscriptions,
			]),
		}
	}
}
