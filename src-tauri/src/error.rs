use std::{error::Error as StdError, fmt::Display};

use serde::Serialize;
use specta::Type;
use twitch_api::{client::CompatError, helix};
use twitch_irc::{SecureTCPTransport, login::StaticLoginCredentials};
use twitch_oauth2::tokens::errors::{DeviceUserTokenExchangeError, ValidationError};

// message for the frontend
#[derive(Debug, Default, Clone, Copy, Serialize, Type, PartialEq, Eq)]
pub enum ErrorMsg {
	#[default]
	Unknown,
	TokenLoad,
	TokenSave,
	TwitchAuth,
	GetColorScheme,
	UsernameGone,
	TokenGone,
	ChatMsgSend,
	AlreadyLoggedIn,
	FeatureDisabled,
	WebSocketSetup,
	RedeemRequest,
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
	pub fn new(msg: ErrorMsg) -> Self {
		Self { msg, src: None }
	}

	pub fn from_err(src: anyhow::Error, msg: ErrorMsg) -> Self {
		// TODO: log error
		Self {
			src: Some(src),
			msg,
		}
	}

	/// Only sets the message if the error is unknown
	pub fn try_set_msg(mut self, msg: ErrorMsg) -> Self {
		if msg == ErrorMsg::Unknown {
			self.msg = msg;
		}

		self
	}

	/// Overwrites the message, no matter what
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

impl From<ErrorMsg> for Error {
	fn from(value: ErrorMsg) -> Self {
		Self::new(value)
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<keyring::Error> for Error {
	fn from(value: keyring::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

type ThreadsafeError = dyn StdError + Send + Sync + 'static;

impl From<ValidationError<&ThreadsafeError>> for Error {
	fn from(value: ValidationError<&ThreadsafeError>) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<ValidationError<CompatError<reqwest::Error>>> for Error {
	fn from(value: ValidationError<CompatError<reqwest::Error>>) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<DeviceUserTokenExchangeError<CompatError<reqwest::Error>>> for Error {
	fn from(value: DeviceUserTokenExchangeError<CompatError<reqwest::Error>>) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<toml::ser::Error> for Error {
	fn from(value: toml::ser::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<toml::de::Error> for Error {
	fn from(value: toml::de::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<twitch_irc::Error<SecureTCPTransport, StaticLoginCredentials>> for Error {
	fn from(value: twitch_irc::Error<SecureTCPTransport, StaticLoginCredentials>) -> Self {
		Self::from_err(value.into(), ErrorMsg::ChatMsgSend)
	}
}

impl From<twitch_irc::validate::Error> for Error {
	fn from(value: twitch_irc::validate::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::ChatMsgSend)
	}
}

impl From<anyhow::Error> for Error {
	fn from(value: anyhow::Error) -> Self {
		Self::from_err(value, ErrorMsg::Unknown)
	}
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
	fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

impl From<helix::ClientRequestError<reqwest::Error>> for Error {
	fn from(value: helix::ClientRequestError<reqwest::Error>) -> Self {
		Self::from_err(value.into(), ErrorMsg::Unknown)
	}
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
	fn from(value: windows::core::Error) -> Self {
		Self::from_err(value.into(), ErrorMsg::GetColorScheme)
	}
}

#[cfg(target_os = "windows")]
impl From<hex::FromHexError> for Error {
	fn from(value: hex::FromHexError) -> Self {
		Self::from_err(value.into(), ErrorMsg::GetColorScheme)
	}
}
