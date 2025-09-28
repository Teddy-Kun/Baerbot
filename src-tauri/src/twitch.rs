use std::sync::{Arc, LazyLock};

use tauri::async_runtime::{JoinHandle, RwLock};
use twitch_api::{
	HelixClient,
	helix::{points::CustomReward, users::User},
};
use twitch_oauth2::UserToken;

use crate::twitch::chat::chat_listener;

pub mod actions;
pub mod auth;
pub mod chat;
pub mod counter;
pub mod events;
pub mod redeems;

pub static TWITCH_CLIENT: LazyLock<Arc<RwLock<TwitchClient>>> =
	LazyLock::new(|| Arc::new(RwLock::new(TwitchClient::new())));

pub struct TwitchClient {
	client: HelixClient<'static, reqwest::Client>,
	token: Option<Arc<UserToken>>,
	chat_listener: Option<JoinHandle<()>>,
	user_info: Option<User>,
	websocket_id: Option<Box<str>>,
	redeems: Option<Vec<CustomReward>>,
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
			user_info: None,
			websocket_id: None,
			redeems: None,
		}
	}

	pub fn get_token(&self) -> Option<Arc<UserToken>> {
		self.token.clone()
	}

	// sets the token and starts listening to chat
	pub async fn set_token(&mut self, tkn: UserToken) {
		let info = self
			.client
			.get_user_from_login(&tkn.login.clone().take(), &tkn)
			.await;

		match info {
			Err(e) => tracing::error!("Couldn't get user info: {e}"),
			Ok(info) => self.user_info = info,
		}

		self.token = Some(Arc::new(tkn));

		if let Err(e) = Self::setup_websocket().await {
			tracing::error!("Error setting up websocket: {e}");
		};

		match self.update_redeems().await {
			Err(e) => tracing::error!("Error getting redeems: {e}"),
			Ok(r) => self.redeems = Some(r),
		};

		if let Some(join_handle) = &self.chat_listener {
			join_handle.abort();
		}

		match chat_listener(self).await {
			Ok(h) => self.chat_listener = Some(h),
			Err(e) => tracing::error!("Error starting chat listener: {e}"),
		};
	}

	pub fn get_redeems(&self) -> Option<&[CustomReward]> {
		match &self.redeems {
			None => None,
			Some(r) => Some(r.as_slice()),
		}
	}

	pub fn get_username(&self) -> Option<String> {
		self.token.as_ref().map(|tkn| tkn.login.to_string())
	}
}
