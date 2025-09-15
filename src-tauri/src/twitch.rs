use std::sync::{Arc, LazyLock};

use tokio::sync::RwLock;
use twitch_api::HelixClient;
use twitch_oauth2::UserToken;

pub mod auth;

pub static TWITCH_CLIENT: LazyLock<Arc<RwLock<TwitchClient>>> =
	LazyLock::new(|| Arc::new(RwLock::new(TwitchClient::new())));

pub struct TwitchClient {
	client: HelixClient<'static, reqwest::Client>,
	token: Option<Box<UserToken>>,
}

impl TwitchClient {
	pub fn new() -> Self {
		Self {
			client: HelixClient::default(),
			token: None,
		}
	}

	pub fn is_logged_in(&self) -> bool {
		self.token.is_some()
	}

	pub fn set_token(&mut self, tkn: UserToken) {
		self.token = Some(Box::new(tkn))
	}

	pub fn get_username(&self) {
		// TODO
	}
}
