use std::{
	collections::HashMap,
	sync::{Arc, LazyLock},
};

use tokio::sync::Mutex;
use twitch_oauth2::TwitchToken;

use crate::twitch::TwitchClient;

pub enum BanResult {
	Nope,
	TimedOut,
	Banned,
}

static USER_CACHE: LazyLock<Mutex<HashMap<Arc<str>, Arc<str>>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

impl TwitchClient {
	async fn get_twitch_user_id(&self, username: &str) -> Option<Arc<str>> {
		let mut cache = USER_CACHE.lock().await;
		match cache.get(username) {
			Some(user_id) => Some(user_id.clone()),
			None => {
				let token = self.token.clone()?;
				let usr = self
					.client
					.get_user_from_login(username, token.as_ref())
					.await;
				if let Ok(Some(usr)) = usr {
					let id: Arc<str> = Arc::from(usr.id.as_str());
					cache.insert(Arc::from(usr.login.as_str()), id.clone());
					Some(id)
				} else {
					None
				}
			}
		}
	}

	/// Return Some<()> if the user was banned/timed out. Returns None if they weren't
	pub async fn ban_user(
		&self,
		username: &str,
		reason: &str,
		duration: Option<u32>,
	) -> Option<()> {
		let token = self.token.clone()?;
		let user_id = token.user_id()?;

		if username == token.login.as_str() {
			return None;
		}

		let id = self
			.get_twitch_user_id(username.to_lowercase().as_str())
			.await?;

		_ = self
			.client
			.ban_user(
				id.as_ref(),
				reason,
				duration,
				user_id,
				user_id,
				token.as_ref(),
			)
			.await
			.ok()?;

		Some(())
	}
}
