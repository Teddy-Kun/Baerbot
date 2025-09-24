use std::sync::{Arc, LazyLock};

use tauri::async_runtime::{JoinHandle, RwLock};
use twitch_api::HelixClient;
use twitch_oauth2::UserToken;

use crate::twitch::chat::chat_listener;

pub mod actions;
pub mod auth;
pub mod chat;
pub mod counter;

pub static TWITCH_CLIENT: LazyLock<Arc<RwLock<TwitchClient>>> =
	LazyLock::new(|| Arc::new(RwLock::new(TwitchClient::new())));

pub struct TwitchClient {
	client: HelixClient<'static, reqwest::Client>,
	token: Option<Arc<UserToken>>,
	chat_listener: Option<JoinHandle<()>>,
}

impl Default for TwitchClient {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for TwitchClient {
	fn drop(&mut self) {
		if let Some(join_handle) = &self.chat_listener {
			join_handle.abort();
		}
	}
}

impl TwitchClient {
	pub fn new() -> Self {
		Self {
			client: HelixClient::default(),
			token: None,
			chat_listener: None,
		}
	}

	pub fn get_token(&self) -> Option<Arc<UserToken>> {
		self.token.clone()
	}

	// sets the token and starts listening to chat
	pub async fn set_token(&mut self, tkn: UserToken) {
		self.token = Some(Arc::new(tkn));

		if let Some(join_handle) = &self.chat_listener {
			join_handle.abort();
		}

		match chat_listener(self).await {
			Ok(h) => self.chat_listener = Some(h),
			Err(e) => tracing::error!("Error starting chat listener: {e}"),
		};
	}

	pub fn get_username(&self) -> Option<String> {
		self.token.as_ref().map(|tkn| tkn.login.to_string())
	}
}
