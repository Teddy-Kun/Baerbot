use std::{
	fs,
	sync::{Arc, LazyLock},
};

use clap::Parser;
use clap_config::ClapConfig;
use serde::{Deserialize, Serialize};
use twitch_oauth2::Scope;

use crate::{error::Error, utils::CFG_DIR_PATH};

// global config
pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);
pub static DEFAULT_CACHE: LazyLock<Cache> = LazyLock::new(Cache::default);

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
	pub scopes: Vec<Scope>,
}

impl Cache {
	pub fn save(&self) -> Result<(), Error> {
		let p = CFG_DIR_PATH.join("cache.toml");
		let s = toml::to_string_pretty(self)?;
		fs::write(p, s)?;
		Ok(())
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

impl Default for Cache {
	fn default() -> Self {
		Self {
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
