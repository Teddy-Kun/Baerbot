use std::{error::Error as StdError, fmt::Display};

use serde::Serialize;
use specta::Type;
use twitch_api::client::CompatError;
use twitch_oauth2::tokens::errors::{DeviceUserTokenExchangeError, ValidationError};

pub type Res<T> = Result<T, Error>;

// message for the frontend
#[derive(Debug, Default, Clone, Copy, Serialize, Type, PartialEq, Eq)]
pub enum ErrorMsg {
	#[default]
	Unknown,
	TokenLoad,
	TokenSave,
	TwitchAuth,
	GetColorScheme,
}

impl From<Error> for ErrorMsg {
	fn from(value: Error) -> Self {
		value.msg
	}
}

#[derive(Debug)]
pub struct Error {
	src: Option<anyhow::Error>,
	pub msg: ErrorMsg,
}

impl Error {
	pub fn new(src: Option<anyhow::Error>, msg: ErrorMsg) -> Self {
		// TODO: log error
		Self { src, msg }
	}

	pub fn try_set_msg(mut self, msg: ErrorMsg) -> Self {
		if msg == ErrorMsg::Unknown {
			self.msg = msg;
		}

		self
	}

	pub fn overwrite_msg(mut self, msg: ErrorMsg) -> Self {
		self.msg = msg;
		self
	}
}

impl StdError for Error {}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.src {
			None => write!(f, "{:?}", self.msg),
			Some(err) => write!(f, "{:?}: {}", self.msg, err),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

impl From<keyring::Error> for Error {
	fn from(value: keyring::Error) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

type ThreadsafeError = dyn StdError + Send + Sync + 'static;

impl From<ValidationError<&ThreadsafeError>> for Error {
	fn from(value: ValidationError<&ThreadsafeError>) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

impl From<ValidationError<CompatError<reqwest::Error>>> for Error {
	fn from(value: ValidationError<CompatError<reqwest::Error>>) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

impl From<DeviceUserTokenExchangeError<CompatError<reqwest::Error>>> for Error {
	fn from(value: DeviceUserTokenExchangeError<CompatError<reqwest::Error>>) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

impl From<toml::ser::Error> for Error {
	fn from(value: toml::ser::Error) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

impl From<toml::de::Error> for Error {
	fn from(value: toml::de::Error) -> Self {
		Self::new(Some(value.into()), ErrorMsg::Unknown)
	}
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
	fn from(value: windows::core::Error) -> Self {
		Self::new(Some(value.into()), ErrorMsg::GetColorScheme)
	}
}

#[cfg(target_os = "windows")]
impl From<hex::FromHexError> for Error {
	fn from(value: hex::FromHexError) -> Self {
		Self::new(Some(value.into()), ErrorMsg::GetColorScheme)
	}
}
