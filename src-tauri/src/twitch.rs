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

impl Default for TwitchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TwitchClient {
	pub fn new() -> Self {
		Self {
			client: HelixClient::default(),
			token: None,
		}
	}

	pub fn set_token(&mut self, tkn: UserToken) {
		self.token = Some(Box::new(tkn))
	}

	pub fn get_username(&self) -> Option<String> {
		self.token.as_ref().map(|tkn| tkn.login.to_string())
	}
}
